pub mod ast;
pub mod parser;

pub use ast::AstNode;
pub use parser::Parser;

use crate::error::ParseError;
use crate::markdown_parser::lexer::tokenize;

/// Parse tokens into an AST
pub fn parse_tokens(
    tokens: Vec<crate::markdown_parser::lexer::Token>,
) -> Result<AstNode, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Parse markdown text into an AST
pub fn parse_markdown(input: &str) -> Result<AstNode, ParseError> {
    let tokens = tokenize(input)?;
    parse_tokens(tokens)
}

/// Parse markdown text into an AST, returning a default document on error
pub fn parse_markdown_or_default(input: &str) -> AstNode {
    parse_markdown(input).unwrap_or_else(|_| AstNode::Document { children: vec![] })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_markdown() {
        let markdown = "# Hello\n\nThis is a paragraph.";
        let ast = parse_markdown(markdown).unwrap();

        if let AstNode::Document { children } = ast {
            assert!(children.len() >= 2);
            assert!(matches!(children[0], AstNode::Heading { level: 1, .. }));
            // Look for paragraph anywhere in children (accounting for whitespace nodes)
            let has_paragraph = children
                .iter()
                .any(|child| matches!(child, AstNode::Paragraph { .. }));
            assert!(has_paragraph);
        }
    }

    #[test]
    fn test_parse_or_default() {
        let invalid_markdown = "";
        let ast = parse_markdown_or_default(invalid_markdown);

        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 0);
        }
    }
}
