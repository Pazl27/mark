use crate::error::ParseError;
use crate::markdown_parser::lexer::Token;
use crate::markdown_parser::parser::ast::AstNode;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    column: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn parse(&mut self) -> Result<AstNode, ParseError> {
        let mut children = Vec::new();

        while !self.is_at_end() {
            if let Some(node) = self.parse_block()? {
                children.push(node);
            }
        }

        Ok(AstNode::Document { children })
    }

    fn parse_block(&mut self) -> Result<Option<AstNode>, ParseError> {
        // Skip whitespace at the beginning of blocks
        self.skip_whitespace();

        match self.current_token().cloned() {
            Some(Token::Hash(level)) => Ok(Some(self.parse_heading(level)?)),
            Some(Token::Number(_)) => Ok(Some(self.parse_ordered_list()?)),
            Some(Token::Hyphen) => {
                if self.is_horizontal_rule() {
                    Ok(Some(self.parse_horizontal_rule()?))
                } else {
                    Ok(Some(self.parse_unordered_list()?))
                }
            }
            Some(Token::Plus) => Ok(Some(self.parse_unordered_list()?)),
            Some(Token::GreaterThan) => Ok(Some(self.parse_blockquote()?)),
            Some(Token::Backtick(amount)) if amount >= 3 => {
                Ok(Some(self.parse_code_block(amount)?))
            }
            Some(Token::Pipe) => Ok(Some(self.parse_table()?)),
            Some(Token::Newline) => {
                self.advance();
                Ok(None)
            }
            Some(_) => Ok(Some(self.parse_paragraph()?)),
            None => Ok(None),
        }
    }

    fn parse_heading(&mut self, level: u8) -> Result<AstNode, ParseError> {
        if level > 6 {
            return Err(ParseError::invalid_heading_level(
                level,
                self.line,
                self.column,
            ));
        }

        self.advance(); // Consume the '#'
        self.skip_whitespace();
        let content = self.parse_inline_content_until_newline()?;
        Ok(AstNode::Heading { level, content })
    }

    fn parse_paragraph(&mut self) -> Result<AstNode, ParseError> {
        let mut content = Vec::new();

        while let Some(token) = self.current_token() {
            match token {
                Token::Newline => {
                    // Check if next line starts a new block
                    if self.peek_next_is_block_start() {
                        break;
                    }
                    // If not, treat as line break within paragraph
                    content.push(AstNode::LineBreak);
                    self.advance();
                }
                Token::Eof => break,
                _ => {
                    let inline_nodes = self.parse_inline_content()?;
                    content.extend(inline_nodes);
                }
            }
        }

        Ok(AstNode::Paragraph { content })
    }

    fn parse_ordered_list(&mut self) -> Result<AstNode, ParseError> {
        let mut items = Vec::new();

        while let Some(Token::Number(_)) = self.current_token() {
            self.advance(); // Consume number

            // Expect a dot
            if !matches!(self.current_token(), Some(Token::Dot)) {
                return Err(ParseError::invalid_list(
                    "Expected '.' after list number".to_string(),
                    self.line,
                    self.column,
                ));
            }
            self.advance(); // Consume dot
            self.skip_whitespace();

            let content = self.parse_inline_content_until_newline()?;
            items.push(AstNode::ListItem { content });

            // Skip newlines between items
            while matches!(self.current_token(), Some(Token::Newline)) {
                self.advance();
            }
        }

        Ok(AstNode::List {
            ordered: true,
            items,
        })
    }

    fn parse_unordered_list(&mut self) -> Result<AstNode, ParseError> {
        let mut items = Vec::new();

        while matches!(
            self.current_token(),
            Some(Token::Hyphen) | Some(Token::Plus)
        ) {
            self.advance(); // Consume list marker
            self.skip_whitespace();

            let content = self.parse_inline_content_until_newline()?;
            items.push(AstNode::ListItem { content });

            // Skip newlines between items
            while matches!(self.current_token(), Some(Token::Newline)) {
                self.advance();
            }
        }

        Ok(AstNode::List {
            ordered: false,
            items,
        })
    }

    fn parse_blockquote(&mut self) -> Result<AstNode, ParseError> {
        let mut content = Vec::new();

        while matches!(self.current_token(), Some(Token::GreaterThan)) {
            self.advance(); // Consume '>'
            self.skip_whitespace();

            let line_content = self.parse_inline_content_until_newline()?;
            content.extend(line_content);

            // Skip newlines
            while matches!(self.current_token(), Some(Token::Newline)) {
                self.advance();
            }
        }

        Ok(AstNode::BlockQuote { content })
    }

    fn parse_code_block(&mut self, _fence_length: u8) -> Result<AstNode, ParseError> {
        self.advance(); // Consume opening backticks

        // Parse optional language
        let mut language = None;
        if let Some(Token::Text(lang)) = self.current_token() {
            language = Some(lang.clone());
            self.advance();
        }

        // Skip to end of line
        while !matches!(
            self.current_token(),
            Some(Token::Newline) | Some(Token::Eof)
        ) {
            self.advance();
        }
        self.advance(); // Consume newline

        // Collect code content until closing fence
        let mut code = String::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::Backtick(3) | Token::Backtick(4) | Token::Backtick(5) => {
                    self.advance(); // Consume closing fence
                    break;
                }
                Token::Text(text) => {
                    code.push_str(text);
                    self.advance();
                }
                Token::Newline => {
                    code.push('\n');
                    self.advance();
                }
                Token::Eof => break,
                _ => {
                    // Include other tokens as text in code block
                    code.push_str(&format!("{:?}", token));
                    self.advance();
                }
            }
        }

        Ok(AstNode::CodeBlock { language, code })
    }

    fn parse_horizontal_rule(&mut self) -> Result<AstNode, ParseError> {
        // Consume the three or more hyphens
        let mut count = 0;
        while matches!(self.current_token(), Some(Token::Hyphen)) {
            count += 1;
            self.advance();
        }

        if count < 3 {
            return Err(ParseError::invalid_list(
                "Horizontal rule requires at least 3 hyphens".to_string(),
                self.line,
                self.column,
            ));
        }

        Ok(AstNode::HorizontalRule)
    }

    fn parse_table(&mut self) -> Result<AstNode, ParseError> {
        // For now, parse a simple table
        let mut headers = Vec::new();
        let mut rows = Vec::new();

        // Parse header row
        if matches!(self.current_token(), Some(Token::Pipe)) {
            self.advance(); // Skip initial pipe

            while !matches!(
                self.current_token(),
                Some(Token::Newline) | Some(Token::Eof)
            ) {
                if matches!(self.current_token(), Some(Token::Pipe)) {
                    self.advance();
                    continue;
                }

                let cell_content = self.parse_table_cell_content()?;
                headers.push(AstNode::TableCell {
                    content: cell_content,
                });
            }
        }

        // Skip newline
        if matches!(self.current_token(), Some(Token::Newline)) {
            self.advance();
        }

        // Parse separator row (skip for now)
        while !matches!(
            self.current_token(),
            Some(Token::Newline) | Some(Token::Eof)
        ) {
            self.advance();
        }
        if matches!(self.current_token(), Some(Token::Newline)) {
            self.advance();
        }

        // Parse data rows
        while matches!(self.current_token(), Some(Token::Pipe)) {
            let mut row_cells = Vec::new();
            self.advance(); // Skip initial pipe

            while !matches!(
                self.current_token(),
                Some(Token::Newline) | Some(Token::Eof)
            ) {
                if matches!(self.current_token(), Some(Token::Pipe)) {
                    self.advance();
                    continue;
                }

                let cell_content = self.parse_table_cell_content()?;
                row_cells.push(AstNode::TableCell {
                    content: cell_content,
                });
            }

            rows.push(row_cells);

            if matches!(self.current_token(), Some(Token::Newline)) {
                self.advance();
            }
        }

        Ok(AstNode::Table { headers, rows })
    }

    fn parse_table_cell_content(&mut self) -> Result<Vec<AstNode>, ParseError> {
        let mut content = Vec::new();

        while !matches!(
            self.current_token(),
            Some(Token::Pipe) | Some(Token::Newline) | Some(Token::Eof)
        ) {
            match self.current_token() {
                Some(Token::Text(text)) => {
                    content.push(AstNode::Text(text.clone()));
                    self.advance();
                }
                Some(Token::Asterisk(count)) => {
                    content.push(self.parse_emphasis(*count)?);
                }
                Some(Token::LeftBracket) => {
                    content.push(self.parse_link_or_image()?);
                }
                Some(Token::Backtick(1)) => {
                    content.push(self.parse_inline_code()?);
                }
                _ => {
                    self.advance(); // Skip unknown tokens
                }
            }
        }

        Ok(content)
    }

    fn parse_inline_content_until_newline(&mut self) -> Result<Vec<AstNode>, ParseError> {
        let mut content = Vec::new();

        while let Some(token) = self.current_token() {
            match token {
                Token::Newline | Token::Eof => break,
                Token::Text(text) => {
                    content.push(AstNode::Text(text.clone()));
                    self.advance();
                }
                Token::Asterisk(count) => {
                    content.push(self.parse_emphasis(*count)?);
                }
                Token::Underscore(count) => {
                    content.push(self.parse_underscore_emphasis(*count)?);
                }
                Token::LeftBracket => {
                    content.push(self.parse_link_or_image()?);
                }
                Token::Backtick(1) => {
                    content.push(self.parse_inline_code()?);
                }
                Token::Tilde(2) => {
                    content.push(self.parse_strikethrough()?);
                }
                Token::Whitespace => {
                    content.push(AstNode::Text(" ".to_string()));
                    self.advance();
                }
                _ => {
                    self.advance(); // Skip unhandled tokens for now
                }
            }
        }
        Ok(content)
    }

    fn parse_inline_content(&mut self) -> Result<Vec<AstNode>, ParseError> {
        let mut content = Vec::new();

        match self.current_token() {
            Some(Token::Text(text)) => {
                content.push(AstNode::Text(text.clone()));
                self.advance();
            }
            Some(Token::Asterisk(count)) => {
                content.push(self.parse_emphasis(*count)?);
            }
            Some(Token::Underscore(count)) => {
                content.push(self.parse_underscore_emphasis(*count)?);
            }
            Some(Token::LeftBracket) => {
                content.push(self.parse_link_or_image()?);
            }
            Some(Token::Backtick(1)) => {
                content.push(self.parse_inline_code()?);
            }
            Some(Token::Tilde(2)) => {
                content.push(self.parse_strikethrough()?);
            }
            Some(Token::Whitespace) => {
                content.push(AstNode::Text(" ".to_string()));
                self.advance();
            }
            _ => {
                self.advance(); // Skip unhandled tokens for now
            }
        }

        Ok(content)
    }

    fn parse_emphasis(&mut self, count: u8) -> Result<AstNode, ParseError> {
        self.advance(); // Consume opening asterisks

        let mut content = Vec::new();
        let mut found_closing = false;

        while let Some(token) = self.current_token() {
            match token {
                Token::Asterisk(closing_count) if *closing_count == count => {
                    self.advance();
                    found_closing = true;
                    break;
                }
                Token::Newline | Token::Eof => break,
                _ => {
                    let inline_nodes = self.parse_inline_content()?;
                    content.extend(inline_nodes);
                }
            }
        }

        if !found_closing {
            return Err(ParseError::unmatched_delimiter('*', self.line, self.column));
        }

        match count {
            1 => Ok(AstNode::Italic(content)),
            2 => Ok(AstNode::Bold(content)),
            _ => Ok(AstNode::Text("*".repeat(count as usize))), // Fallback for unexpected counts
        }
    }

    fn parse_underscore_emphasis(&mut self, count: u8) -> Result<AstNode, ParseError> {
        self.advance(); // Consume opening underscores

        let mut content = Vec::new();
        let mut found_closing = false;

        while let Some(token) = self.current_token() {
            match token {
                Token::Underscore(closing_count) if *closing_count == count => {
                    self.advance();
                    found_closing = true;
                    break;
                }
                Token::Newline | Token::Eof => break,
                _ => {
                    let inline_nodes = self.parse_inline_content()?;
                    content.extend(inline_nodes);
                }
            }
        }

        if !found_closing {
            return Err(ParseError::unmatched_delimiter('_', self.line, self.column));
        }

        match count {
            1 => Ok(AstNode::Italic(content)),
            2 => Ok(AstNode::Bold(content)),
            _ => Ok(AstNode::Text("_".repeat(count as usize))), // Fallback for unexpected counts
        }
    }

    fn parse_strikethrough(&mut self) -> Result<AstNode, ParseError> {
        self.advance(); // Consume opening tildes

        let mut content = Vec::new();
        let mut found_closing = false;

        while let Some(token) = self.current_token() {
            match token {
                Token::Tilde(2) => {
                    self.advance();
                    found_closing = true;
                    break;
                }
                Token::Newline | Token::Eof => break,
                _ => {
                    let inline_nodes = self.parse_inline_content()?;
                    content.extend(inline_nodes);
                }
            }
        }

        if !found_closing {
            return Err(ParseError::unmatched_delimiter('~', self.line, self.column));
        }

        Ok(AstNode::Strikethrough(content))
    }

    fn parse_inline_code(&mut self) -> Result<AstNode, ParseError> {
        self.advance(); // Consume opening backtick

        let mut code = String::new();
        let mut found_closing = false;

        while let Some(token) = self.current_token() {
            match token {
                Token::Backtick(1) => {
                    self.advance();
                    found_closing = true;
                    break;
                }
                Token::Text(text) => {
                    code.push_str(text);
                    self.advance();
                }
                Token::Whitespace => {
                    code.push(' ');
                    self.advance();
                }
                Token::Newline | Token::Eof => break,
                _ => {
                    // Include other tokens as literal text in inline code
                    code.push_str(&format!("{:?}", token));
                    self.advance();
                }
            }
        }

        if !found_closing {
            return Err(ParseError::unmatched_delimiter('`', self.line, self.column));
        }

        Ok(AstNode::InlineCode(code))
    }

    fn parse_link_or_image(&mut self) -> Result<AstNode, ParseError> {
        // Check if this is an image (starts with ![)
        if matches!(self.peek_previous(), Some(Token::Exclamation)) {
            self.parse_image()
        } else {
            self.parse_link()
        }
    }

    fn parse_link(&mut self) -> Result<AstNode, ParseError> {
        self.advance(); // Consume '['

        // Parse link text
        let mut text = Vec::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::RightBracket => {
                    self.advance();
                    break;
                }
                Token::Eof => {
                    return Err(ParseError::malformed_link(
                        "Unexpected end of input in link text".to_string(),
                        self.line,
                        self.column,
                    ));
                }
                _ => {
                    let inline_nodes = self.parse_inline_content()?;
                    text.extend(inline_nodes);
                }
            }
        }

        // Expect '('
        if !matches!(self.current_token(), Some(Token::LeftParen)) {
            return Err(ParseError::malformed_link(
                "Expected '(' after link text".to_string(),
                self.line,
                self.column,
            ));
        }
        self.advance();

        // Parse URL
        let mut url = String::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::RightParen => {
                    self.advance();
                    break;
                }
                Token::Text(text_content) => {
                    url.push_str(text_content);
                    self.advance();
                }
                Token::Url(url_content) => {
                    url.push_str(url_content);
                    self.advance();
                }
                Token::Eof => {
                    return Err(ParseError::malformed_link(
                        "Unexpected end of input in link URL".to_string(),
                        self.line,
                        self.column,
                    ));
                }
                _ => {
                    self.advance(); // Skip unexpected tokens
                }
            }
        }

        Ok(AstNode::Link { text, url })
    }

    fn parse_image(&mut self) -> Result<AstNode, ParseError> {
        self.advance(); // Consume '['

        // Parse alt text
        let mut alt = Vec::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::RightBracket => {
                    self.advance();
                    break;
                }
                Token::Eof => {
                    return Err(ParseError::malformed_image(
                        "Unexpected end of input in image alt text".to_string(),
                        self.line,
                        self.column,
                    ));
                }
                _ => {
                    let inline_nodes = self.parse_inline_content()?;
                    alt.extend(inline_nodes);
                }
            }
        }

        // Expect '('
        if !matches!(self.current_token(), Some(Token::LeftParen)) {
            return Err(ParseError::malformed_image(
                "Expected '(' after image alt text".to_string(),
                self.line,
                self.column,
            ));
        }
        self.advance();

        // Parse URL
        let mut url = String::new();
        while let Some(token) = self.current_token() {
            match token {
                Token::RightParen => {
                    self.advance();
                    break;
                }
                Token::Text(text_content) => {
                    url.push_str(text_content);
                    self.advance();
                }
                Token::Url(url_content) => {
                    url.push_str(url_content);
                    self.advance();
                }
                Token::Eof => {
                    return Err(ParseError::malformed_image(
                        "Unexpected end of input in image URL".to_string(),
                        self.line,
                        self.column,
                    ));
                }
                _ => {
                    self.advance(); // Skip unexpected tokens
                }
            }
        }

        Ok(AstNode::Image { alt, url })
    }

    // Helper methods
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    fn peek_previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            if let Some(Token::Newline) = self.current_token() {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.current_token(), Some(Token::Eof))
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current_token(), Some(Token::Whitespace)) {
            self.advance();
        }
    }

    fn is_horizontal_rule(&self) -> bool {
        // Check if we have at least 3 consecutive hyphens
        let mut count = 0;
        let mut pos = self.current;

        while let Some(Token::Hyphen) = self.tokens.get(pos) {
            count += 1;
            pos += 1;
        }

        count >= 3
    }

    fn peek_next_is_block_start(&self) -> bool {
        // Look ahead to see if the next non-whitespace token starts a block
        let mut pos = self.current + 1;

        // Skip whitespace and newlines
        while let Some(token) = self.tokens.get(pos) {
            match token {
                Token::Whitespace | Token::Newline => pos += 1,
                Token::Hash(_)
                | Token::Number(_)
                | Token::Hyphen
                | Token::Plus
                | Token::GreaterThan
                | Token::Pipe => return true,
                Token::Backtick(count) if *count >= 3 => return true,
                _ => return false,
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create common token sequences
    fn heading_tokens(level: u8, text: &str) -> Vec<Token> {
        vec![
            Token::Hash(level),
            Token::Whitespace,
            Token::Text(text.to_string()),
            Token::Eof,
        ]
    }

    fn paragraph_tokens(text: &str) -> Vec<Token> {
        vec![Token::Text(text.to_string()), Token::Eof]
    }

    fn emphasis_tokens(marker: Token, text: &str, closing_marker: Token) -> Vec<Token> {
        vec![
            marker,
            Token::Text(text.to_string()),
            closing_marker,
            Token::Eof,
        ]
    }

    #[test]
    fn test_parse_heading_level_1() {
        let tokens = heading_tokens(1, "Main Title");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::Heading { level, content } = &children[0] {
                assert_eq!(*level, 1);
                assert!(!content.is_empty(), "Heading should have content");
            } else {
                panic!("Expected heading node");
            }
        }
    }

    #[test]
    fn test_parse_heading_level_6() {
        let tokens = heading_tokens(6, "Small Title");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::Heading { level, .. } = &children[0] {
                assert_eq!(*level, 6);
            }
        }
    }

    #[test]
    fn test_parse_invalid_heading_level() {
        let tokens = vec![
            Token::Hash(7),
            Token::Text("Invalid".to_string()),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        if let Err(ParseError::InvalidHeadingLevel { level, .. }) = result {
            assert_eq!(level, 7);
        } else {
            panic!("Expected InvalidHeadingLevel error");
        }
    }

    #[test]
    fn test_parse_simple_paragraph() {
        let tokens = paragraph_tokens("Simple paragraph text");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::Paragraph { content } = &children[0] {
                assert_eq!(content.len(), 1);
                if let AstNode::Text(text) = &content[0] {
                    assert_eq!(text, "Simple paragraph text");
                }
            }
        }
    }

    #[test]
    fn test_parse_italic_emphasis() {
        let tokens = emphasis_tokens(Token::Asterisk(1), "italic", Token::Asterisk(1));
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::Paragraph { content } = &children[0] {
                if let AstNode::Italic(italic_content) = &content[0] {
                    if let AstNode::Text(text) = &italic_content[0] {
                        assert_eq!(text, "italic");
                    }
                } else {
                    panic!("Expected italic node");
                }
            }
        }
    }

    #[test]
    fn test_parse_bold_emphasis() {
        let tokens = emphasis_tokens(Token::Asterisk(2), "bold", Token::Asterisk(2));
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::Paragraph { content } = &children[0] {
                if let AstNode::Bold(bold_content) = &content[0] {
                    if let AstNode::Text(text) = &bold_content[0] {
                        assert_eq!(text, "bold");
                    }
                } else {
                    panic!("Expected bold node");
                }
            }
        }
    }

    #[test]
    fn test_parse_underscore_emphasis() {
        let tokens = emphasis_tokens(Token::Underscore(1), "italic", Token::Underscore(1));
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::Paragraph { content } = &children[0] {
                if let AstNode::Italic(italic_content) = &content[0] {
                    if let AstNode::Text(text) = &italic_content[0] {
                        assert_eq!(text, "italic");
                    }
                } else {
                    panic!("Expected italic node");
                }
            }
        }
    }

    #[test]
    fn test_parse_strikethrough() {
        let tokens = vec![
            Token::Tilde(2),
            Token::Text("strikethrough".to_string()),
            Token::Tilde(2),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::Paragraph { content } = &children[0] {
                if let AstNode::Strikethrough(strike_content) = &content[0] {
                    if let AstNode::Text(text) = &strike_content[0] {
                        assert_eq!(text, "strikethrough");
                    }
                } else {
                    panic!("Expected strikethrough node");
                }
            }
        }
    }

    #[test]
    fn test_parse_inline_code() {
        let tokens = vec![
            Token::Backtick(1),
            Token::Text("console.log()".to_string()),
            Token::Backtick(1),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::Paragraph { content } = &children[0] {
                if let AstNode::InlineCode(code) = &content[0] {
                    assert_eq!(code, "console.log()");
                } else {
                    panic!("Expected inline code node");
                }
            }
        }
    }

    #[test]
    fn test_parse_unmatched_emphasis() {
        let tokens = vec![
            Token::Asterisk(1),
            Token::Text("unmatched".to_string()),
            Token::Newline,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ParseError::UnmatchedDelimiter { delimiter: '*', .. })
        ));
    }

    #[test]
    fn test_parse_ordered_list() {
        let tokens = vec![
            Token::Number(1),
            Token::Dot,
            Token::Whitespace,
            Token::Text("First item".to_string()),
            Token::Newline,
            Token::Number(2),
            Token::Dot,
            Token::Whitespace,
            Token::Text("Second item".to_string()),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::List { ordered, items } = &children[0] {
                assert!(*ordered);
                assert_eq!(items.len(), 2);

                if let AstNode::ListItem { content } = &items[0] {
                    if let AstNode::Text(text) = &content[0] {
                        assert_eq!(text, "First item");
                    }
                }
            } else {
                panic!("Expected ordered list");
            }
        }
    }

    #[test]
    fn test_parse_unordered_list() {
        let tokens = vec![
            Token::Hyphen,
            Token::Whitespace,
            Token::Text("First item".to_string()),
            Token::Newline,
            Token::Plus,
            Token::Whitespace,
            Token::Text("Second item".to_string()),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::List { ordered, items } = &children[0] {
                assert!(!*ordered);
                assert_eq!(items.len(), 2);
            } else {
                panic!("Expected unordered list");
            }
        }
    }

    #[test]
    fn test_parse_blockquote() {
        let tokens = vec![
            Token::GreaterThan,
            Token::Whitespace,
            Token::Text("Quoted text".to_string()),
            Token::Newline,
            Token::GreaterThan,
            Token::Whitespace,
            Token::Text("More quoted text".to_string()),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::BlockQuote { content } = &children[0] {
                assert!(content.len() >= 2);
            } else {
                panic!("Expected blockquote");
            }
        }
    }

    #[test]
    fn test_parse_code_block() {
        let tokens = vec![
            Token::Backtick(3),
            Token::Text("rust".to_string()),
            Token::Newline,
            Token::Text("fn main() {}".to_string()),
            Token::Newline,
            Token::Backtick(3),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::CodeBlock { language, code } = &children[0] {
                assert_eq!(language.as_ref().unwrap(), "rust");
                assert!(code.contains("fn main() {}"));
            } else {
                panic!("Expected code block");
            }
        }
    }

    #[test]
    fn test_parse_horizontal_rule() {
        let tokens = vec![Token::Hyphen, Token::Hyphen, Token::Hyphen, Token::Eof];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::HorizontalRule = &children[0] {
                // Success
            } else {
                panic!("Expected horizontal rule");
            }
        }
    }

    #[test]
    fn test_parse_link() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Text("Link text".to_string()),
            Token::RightBracket,
            Token::LeftParen,
            Token::Url("https://example.com".to_string()),
            Token::RightParen,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::Paragraph { content } = &children[0] {
                if let AstNode::Link { text, url } = &content[0] {
                    assert_eq!(url, "https://example.com");
                    if let AstNode::Text(link_text) = &text[0] {
                        assert_eq!(link_text, "Link text");
                    }
                } else {
                    panic!("Expected link node");
                }
            }
        }
    }

    #[test]
    fn test_parse_malformed_link() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Text("Malformed".to_string()),
            Token::RightBracket,
            Token::Eof, // Missing opening parenthesis
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert!(matches!(result, Err(ParseError::MalformedLink { .. })));
    }

    #[test]
    fn test_parse_empty_document() {
        let tokens = vec![Token::Eof];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 0);
        }
    }

    #[test]
    fn test_parse_mixed_content() {
        let tokens = vec![
            Token::Hash(1),
            Token::Whitespace,
            Token::Text("Title".to_string()),
            Token::Newline,
            Token::Newline,
            Token::Text("Paragraph with ".to_string()),
            Token::Asterisk(1),
            Token::Text("emphasis".to_string()),
            Token::Asterisk(1),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            assert!(children.len() >= 2);
            assert!(matches!(children[0], AstNode::Heading { .. }));

            // Find paragraph (might not be at index 1 due to whitespace handling)
            let has_paragraph = children
                .iter()
                .any(|child| matches!(child, AstNode::Paragraph { .. }));
            assert!(has_paragraph);
        }
    }

    #[test]
    fn test_parse_nested_emphasis() {
        let tokens = vec![
            Token::Asterisk(2), // Bold start
            Token::Text("Bold and ".to_string()),
            Token::Asterisk(1), // Italic start
            Token::Text("italic".to_string()),
            Token::Asterisk(1), // Italic end
            Token::Asterisk(2), // Bold end
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        if let AstNode::Document { children } = ast {
            if let AstNode::Paragraph { content } = &children[0] {
                if let AstNode::Bold(bold_content) = &content[0] {
                    assert!(bold_content.len() >= 2);
                    // Should contain both text and nested italic
                } else {
                    panic!("Expected bold node");
                }
            }
        }
    }
}
