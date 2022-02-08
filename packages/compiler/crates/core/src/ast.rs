use std::fmt::Display;

// AST
#[derive(Debug)]
pub enum Expr {
    Variant {
        variant_group: Vec<Option<Variant>>,
        variants: Vec<Variant>,
        exprs: Vec<Expr>,
    },
    Util(Util),
    RawCss(String),
}

#[derive(Debug)]
pub struct Util {
    pub properties: Vec<String>,
    pub tree: UtilTree,
}

#[derive(Debug)]
pub enum UtilTree {
    Leaf(Option<ValueOrGroup>),
    Branch(Vec<Expr>),
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Variant {
    Pseudo(Value),
    Media(Value),
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueOrGroup {
    Value(Value),
    Group(Vec<Option<Value>>),
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Iden(String),
    Raw(String),
}

// Flattened AST
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FlatExpr {
    pub variants: Option<Vec<Variant>>,
    pub css: FlatUtilOrRaw,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FlatUtilOrRaw {
    FlatUtil(FlatUtil),
    RawCss(String),
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FlatUtil {
    pub prop: String,
    pub value: Option<Value>,
}

impl FlatExpr {
    pub fn get_class_name(&self) {
        self.
    }
}

pub fn flatten_expr(
    expr: &Expr,
    mut variants: Vec<Variant>,
    variant_group: Vec<Option<Variant>>,
    prev_prop: String,
) -> Vec<FlatExpr> {
    match expr {
        Expr::Util(util) => util
            .properties
            .iter()
            .flat_map(|prop| match &util.tree {
                UtilTree::Leaf(value) => {
                    match value {
                        Some(ValueOrGroup::Group(values)) => {
                            assert_eq!(values.len(), variant_group.len());

                            let mut entries = Vec::new();

                            for (index, value) in values.iter().enumerate() {
                                match value {
                                    Some(value) => {
                                        let mut variants = variants.clone();
                                        if let Some(group_variant) =
                                            variant_group.get(index).unwrap()
                                        {
                                            variants.push(group_variant.clone());
                                        }
                                        // Sort so order won't change hash
                                        variants.sort();
                                        entries.push(FlatExpr {
                                            variants: Some(variants),
                                            css: FlatUtilOrRaw::FlatUtil(FlatUtil {
                                                prop: format!("{}{}", prev_prop, prop),
                                                value: Some(value.clone()),
                                            }),
                                        })
                                    }
                                    None => {}
                                }
                            }

                            entries
                        }
                        Some(ValueOrGroup::Value(value)) => {
                            vec![FlatExpr {
                                variants: if variants.is_empty() {
                                    None
                                } else {
                                    let mut variants = variants.clone();
                                    // Sort so order won't change hash
                                    variants.sort();
                                    Some(variants)
                                },
                                css: FlatUtilOrRaw::FlatUtil(FlatUtil {
                                    prop: format!("{}{}", prev_prop, prop),
                                    value: Some(value.clone()),
                                }),
                            }]
                        }
                        None => {
                            vec![FlatExpr {
                                variants: if variants.is_empty() {
                                    None
                                } else {
                                    let mut variants = variants.clone();
                                    // Sort so order won't change hash
                                    variants.sort();
                                    Some(variants)
                                },
                                css: FlatUtilOrRaw::FlatUtil(FlatUtil {
                                    prop: format!("{}{}", prev_prop, prop),
                                    value: None,
                                }),
                            }]
                        }
                    }
                }
                UtilTree::Branch(exprs) => exprs
                    .iter()
                    .flat_map(|expr| {
                        flatten_expr(
                            expr,
                            variants.clone(),
                            variant_group.clone(),
                            format!("{}{}", prev_prop, prop),
                        )
                    })
                    .collect(),
            })
            .collect(),
        Expr::RawCss(raw_css) => vec![FlatExpr {
            variants: None,
            css: FlatUtilOrRaw::RawCss(raw_css.clone()),
        }],
        Expr::Variant {
            variants: v,
            variant_group,
            exprs,
        } => {
            variants.append(&mut v.clone());
            exprs
                .iter()
                .flat_map(|e| {
                    flatten_expr(
                        e,
                        variants.clone(),
                        variant_group.clone(),
                        prev_prop.clone(),
                    )
                })
                .collect()
        }
    }
}
