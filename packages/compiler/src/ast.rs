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
