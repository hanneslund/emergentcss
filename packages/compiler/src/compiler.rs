use crate::css_writer::CssWriter;
use crate::lexer;
use crate::parser::Parser;

pub struct Compiler {
    css_writer: CssWriter,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            css_writer: CssWriter::new(),
        }
    }

    pub fn generate_classes(&mut self, code: &str) -> Vec<String> {
        let tokens = lexer::get_tokens(code);
        let ast = Parser::from(tokens).parse();
        self.css_writer.generate_classes(&ast)
    }

    pub fn get_css(&mut self) -> String {
        self.css_writer.get_css()
    }
}
