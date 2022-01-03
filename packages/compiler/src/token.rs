#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Whitespace,

    // `:``
    Colon,

    /// `(`
    LParen,

    /// `)`
    RParen,

    /// `=`
    Eq,

    /// "@"
    At,

    /// `_`
    Underscore,

    Word(String),

    RawValue(String),
}
