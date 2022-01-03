use crate::{ast::*, token::Token};
use std::{iter::Peekable, vec::IntoIter};

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl From<Vec<Token>> for Parser {
    fn from(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Vec<Expr> {
        let exprs = self.parse_exprs();

        if let Some(token) = self.tokens.next() {
            panic!("Unexpected token in parse {:?}", token);
        }

        exprs
    }

    fn parse_exprs(&mut self) -> Vec<Expr> {
        let mut expressions = Vec::new();

        loop {
            self.eat_whitespaces();
            match self.parse_expr() {
                Some(expr) => {
                    expressions.push(expr);
                }
                None => break,
            }
        }

        expressions
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let (variants, variant_group) = self.parse_variants();

        if variants.is_empty() && variant_group.is_empty() {
            match self.tokens.peek() {
                Some(Token::RawValue(_)) => {
                    if let Some(Token::RawValue(raw_css)) = self.tokens.next() {
                        return Some(Expr::RawCss(raw_css));
                    }
                    unreachable!()
                }
                _ => Some(Expr::Util(self.parse_util()?)),
            }
        } else {
            match self.tokens.peek() {
                Some(Token::LParen) => {
                    // Eat (
                    self.tokens.next();
                    let exprs = self.parse_exprs();
                    // Eat )
                    self.assert_next(Token::RParen);
                    Some(Expr::Variant {
                        variants,
                        variant_group,
                        exprs,
                    })
                }

                _ => Some(Expr::Variant {
                    variants,
                    variant_group,
                    exprs: vec![self.parse_expr().expect("Expected expr")],
                }),
            }
        }
    }

    fn parse_util(&mut self) -> Option<Util> {
        let properties = self.parse_properties()?;
        let tree = match self.tokens.peek() {
            // Branch, multi util
            Some(Token::LParen) => {
                // Eat (
                self.tokens.next();
                let tree = UtilTree::Branch(self.parse_exprs());
                // Eat )
                self.assert_next(Token::RParen);
                tree
            }

            // Leaf, assignment
            Some(Token::Eq) => {
                // Eat =
                self.tokens.next();
                UtilTree::Leaf(Some(self.parse_value()))
            }

            // Util without assignment
            _ => UtilTree::Leaf(None),
        };

        Some(Util { properties, tree })
    }

    fn parse_variants(&mut self) -> (Vec<Variant>, Vec<Option<Variant>>) {
        let mut variants = Vec::new();
        let mut is_variant_group = false;
        let mut variant_group = Vec::new();
        loop {
            self.eat_whitespaces();
            match self.tokens.peek() {
                // Pseudo
                Some(Token::Word(_)) => {
                    if !is_variant_group {
                        match self.peek_nth(1) {
                            Some(Token::Colon) => {}
                            _ => break, // Is util
                        }
                    }

                    if let Some(Token::Word(word)) = self.tokens.next() {
                        if is_variant_group {
                            variant_group.push(Some(Variant::Pseudo(Value::Iden(word))));
                        } else {
                            variants.push(Variant::Pseudo(Value::Iden(word)));
                            // Eat :
                            self.tokens.next();
                        }
                    }
                }
                Some(Token::RawValue(_)) => {
                    if !is_variant_group {
                        match self.peek_nth(1) {
                            Some(Token::Colon) => {}
                            _ => break, // Is util
                        }
                    }
                    if let Some(Token::RawValue(raw)) = self.tokens.next() {
                        if is_variant_group {
                            variant_group.push(Some(Variant::Pseudo(Value::Raw(raw))));
                        } else {
                            variants.push(Variant::Pseudo(Value::Raw(raw)));
                            // Eat :
                            self.tokens.next();
                        }
                    }
                }
                // Media
                Some(Token::At) => {
                    // Eat @
                    self.tokens.next();
                    match self.tokens.next() {
                        Some(Token::Word(word)) => {
                            if is_variant_group {
                                variant_group.push(Some(Variant::Media(Value::Iden(word))));
                            } else {
                                variants.push(Variant::Media(Value::Iden(word)));
                                self.assert_next(Token::Colon);
                            }
                        }
                        Some(Token::RawValue(raw)) => {
                            if is_variant_group {
                                variant_group.push(Some(Variant::Media(Value::Raw(raw))));
                            } else {
                                variants.push(Variant::Media(Value::Raw(raw)));
                                self.assert_next(Token::Colon);
                            }
                        }
                        _ => panic!("Expected word or raw value"),
                    }
                }
                Some(Token::Underscore) if is_variant_group => {
                    // Eat _
                    self.tokens.next();
                    variant_group.push(None);
                }
                Some(Token::LParen) if !is_variant_group => {
                    match self.peek_after_token(Token::RParen) {
                        Some(Token::Colon) => {
                            // Is variant group
                            if !variant_group.is_empty() {
                                panic!("Only one variant group allowed");
                            }
                            // Eat (
                            self.tokens.next();
                            is_variant_group = true;
                        }
                        _ => break,
                    }
                }
                Some(Token::RParen) if is_variant_group => {
                    // eat )
                    self.tokens.next();
                    is_variant_group = false;
                    self.assert_next(Token::Colon);
                }
                _ => break,
            }
        }

        // Hasn't closed group
        if is_variant_group {
            panic!("Expected )")
        }

        (variants, variant_group)
    }

    fn parse_properties(&mut self) -> Option<Vec<String>> {
        match self.tokens.peek() {
            Some(Token::Word(_)) => {
                if let Some(Token::Word(iden)) = self.tokens.next() {
                    return Some(vec![iden]);
                }
                unreachable!()
            }
            Some(Token::LParen) => {
                // Eat (
                self.tokens.next();
                let mut properties = Vec::new();

                loop {
                    self.eat_whitespaces();
                    match self.tokens.next() {
                        Some(Token::Word(iden)) => {
                            properties.push(iden);
                        }
                        Some(Token::RParen) => {
                            break;
                        }
                        token => panic!("parse_properties unexpected token: {:?}", token),
                    }
                }

                Some(properties)
            }
            _ => None,
        }
    }

    fn parse_value(&mut self) -> ValueOrGroup {
        match self.tokens.next() {
            Some(Token::RawValue(raw_value)) => ValueOrGroup::Value(Value::Raw(raw_value)),
            Some(Token::Word(word)) => ValueOrGroup::Value(Value::Iden(word)),
            Some(Token::LParen) => {
                let mut values = Vec::new();

                loop {
                    self.eat_whitespaces();
                    match self.tokens.next() {
                        Some(Token::RawValue(raw_value)) => {
                            values.push(Some(Value::Raw(raw_value)));
                        }
                        Some(Token::Word(word)) => {
                            values.push(Some(Value::Iden(word)));
                        }
                        Some(Token::Underscore) => {
                            values.push(None);
                        }
                        Some(Token::RParen) => break,
                        token => panic!("Unexpected token while parsing value group: {:?}", token),
                    }
                }

                if values.is_empty() {
                    panic!("Value is empty.");
                }

                ValueOrGroup::Group(values)
            }
            _ => panic!("Failed to parse value"),
        }
    }

    fn eat_whitespaces(&mut self) {
        while let Some(Token::Whitespace) = self.tokens.peek() {
            self.tokens.next();
        }
    }

    fn assert_next(&mut self, token: Token) {
        match self.tokens.next() {
            Some(c) => {
                assert_eq!(c, token);
            }
            _ => panic!("Expected {:?}", token),
        }
    }

    fn peek_nth(&self, nth: usize) -> Option<Token> {
        self.tokens.clone().nth(nth)
    }

    fn peek_after_token(&self, after_token: Token) -> Option<Token> {
        let mut tokens = self.tokens.clone();
        loop {
            if let Some(token) = tokens.next() {
                if token == after_token {
                    return tokens.next();
                }
            } else {
                return None;
            }
        }
    }
}
