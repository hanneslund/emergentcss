use std::collections::HashMap;

use crate::{
    ast::*,
    utils::{get_util_css, media_queries::get_media_query, pseudo_classes::get_pseudo},
};

type ClassName = String;

pub struct CssWriter {
    class_names: HashMap<CssEntry, ClassName>,
    class_count: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CssEntry {
    variants: Option<Vec<Variant>>,
    css: FlatUtilOrRaw,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum FlatUtilOrRaw {
    FlatUtil(FlatUtil),
    RawCss(String),
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct FlatUtil {
    prop: String,
    value: Option<Value>,
}

impl CssWriter {
    pub fn new() -> Self {
        Self {
            class_names: HashMap::new(),
            class_count: 0,
        }
    }

    fn new_class_name(&mut self) -> String {
        let class_name = format!("_E{}", self.class_count);
        self.class_count += 1;
        class_name
    }

    pub fn get_css(&mut self) -> String {
        let mut css: Vec<String> = self
            .class_names
            .iter()
            .map(|(entry, class_name)| {
                let (pseudo, media) =
                    entry
                        .variants
                        .iter()
                        .fold((Vec::new(), Vec::new()), |acc, cur| {
                            let (mut pseudo, mut media) = acc;
                            for variant in cur {
                                match variant {
                                    Variant::Pseudo(p) => {
                                        pseudo.push(p);
                                    }
                                    Variant::Media(m) => {
                                        media.push(m);
                                    }
                                }
                            }
                            (pseudo, media)
                        });
                let mut pseudo_selectors = String::new();
                for p in pseudo {
                    match p {
                        Value::Iden(iden) => {
                            pseudo_selectors.push_str(
                                &get_pseudo(iden).expect(&format!("Unknown pseudo: {}", iden)),
                            );
                        }
                        Value::Raw(raw) => {
                            pseudo_selectors.push_str(&format!(":{}", raw));
                        }
                    }
                }
                let mut css = match &entry.css {
                    FlatUtilOrRaw::FlatUtil(util) => format!(
                        ".{}{}{{{}}}",
                        class_name,
                        pseudo_selectors,
                        get_util_css(&util.prop, &util.value)
                    ),
                    FlatUtilOrRaw::RawCss(raw) => {
                        format!(".{}{}{{{};}}", class_name, pseudo_selectors, raw)
                    }
                };

                if !media.is_empty() {
                    let media_queries: Vec<String> = media
                        .iter()
                        .map(|m| match m {
                            Value::Iden(iden) => get_media_query(iden).unwrap(),
                            Value::Raw(raw) => raw.clone(),
                        })
                        .collect();

                    css = format!("@media {}{{{}}}", media_queries.join(" and "), css)
                }

                css
            })
            .collect();

        css.sort();
        css.join("")
    }

    pub fn generate_classes(&mut self, ast: &[Expr]) -> Vec<String> {
        let mut class_names = Vec::new();

        let css_entries: Vec<CssEntry> = ast
            .iter()
            .flat_map(|expr| self.flatten_expr(expr, Vec::new(), Vec::new(), String::new()))
            .collect();
        for css_entry in css_entries {
            match self.class_names.get(&css_entry) {
                Some(class_name) => {
                    class_names.push(class_name.clone());
                }
                None => {
                    let class_name = self.new_class_name();
                    self.class_names.insert(css_entry, class_name.clone());
                    class_names.push(class_name);
                }
            };
        }

        class_names
    }

    fn flatten_expr(
        &self,
        expr: &Expr,
        mut variants: Vec<Variant>,
        variant_group: Vec<Option<Variant>>,
        prev_prop: String,
    ) -> Vec<CssEntry> {
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
                                            entries.push(CssEntry {
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
                                vec![CssEntry {
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
                                vec![CssEntry {
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
                            self.flatten_expr(
                                expr,
                                variants.clone(),
                                variant_group.clone(),
                                format!("{}{}", prev_prop, prop),
                            )
                        })
                        .collect(),
                })
                .collect(),
            Expr::RawCss(raw_css) => vec![CssEntry {
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
                        self.flatten_expr(
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
}
