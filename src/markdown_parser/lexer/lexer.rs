use std::{iter::Peekable, str::Chars};

use crate::error::LexerError;
use crate::markdown_parser::lexer::tokens::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            current_pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            if matches!(token, Token::Eof) {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        match self.peek_char().copied() {
            None => Ok(Some(Token::Eof)),
            Some(ch) => {
                let token = self.read_token(ch)?;
                Ok(Some(token))
            }
        }
    }

    fn read_token(&mut self, ch: char) -> Result<Token, LexerError> {
        match ch {
            '\n' => {
                self.advance();
                Ok(Token::Newline)
            }
            '\r' => {
                self.advance();
                // Handle \r\n
                if self.peek_char() == Some(&'\n') {
                    self.advance();
                }
                Ok(Token::Newline)
            }
            ' ' | '\t' => Ok(self.read_whitespace()),
            '#' => Ok(self.read_hashes()),
            '*' => Ok(self.read_asterisks()),
            '`' => Ok(self.read_backticks()),
            '_' => Ok(self.read_underscores()),
            '[' => {
                self.advance();
                Ok(Token::LeftBracket)
            }
            ']' => {
                self.advance();
                Ok(Token::RightBracket)
            }
            '(' => {
                self.advance();
                Ok(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RightParen)
            }
            '!' => {
                self.advance();
                Ok(Token::Exclamation)
            }
            '>' => {
                self.advance();
                Ok(Token::GreaterThan)
            }
            '-' => {
                self.advance();
                Ok(Token::Hyphen)
            }
            '.' => {
                self.advance();
                Ok(Token::Dot)
            }
            '|' => {
                self.advance();
                Ok(Token::Pipe)
            }
            ':' => {
                self.advance();
                Ok(Token::Colon)
            }
            c if c.is_ascii_digit() => self.read_number(),
            '~' => Ok(self.read_tildes()),
            '+' => {
                self.advance();
                Ok(Token::Plus)
            }
            _ => self.read_text(ch),
        }
    }

    fn read_hashes(&mut self) -> Token {
        let mut count = 0;
        while self.peek_char() == Some(&'#') && count < 6 {
            self.advance();
            count += 1;
        }
        Token::Hash(count)
    }

    fn read_asterisks(&mut self) -> Token {
        let mut count = 0;
        while self.peek_char() == Some(&'*') && count < 3 {
            self.advance();
            count += 1;
        }
        Token::Asterisk(count)
    }

    fn read_backticks(&mut self) -> Token {
        let mut count = 0;
        while self.peek_char() == Some(&'`') && count < 4 {
            self.advance();
            count += 1;
        }
        Token::Backtick(count)
    }

    fn read_underscores(&mut self) -> Token {
        let mut count = 0;
        while self.peek_char() == Some(&'_') && count < 3 {
            self.advance();
            count += 1;
        }
        Token::Underscore(count)
    }

    fn read_number(&mut self) -> Result<Token, LexerError> {
        let mut number_str = String::new();
        let start_line = self.line;
        let start_column = self.column;

        while let Some(&ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match number_str.parse::<u32>() {
            Ok(number) => Ok(Token::Number(number)),
            Err(_) => Err(LexerError::number_too_large(
                number_str,
                start_line,
                start_column,
            )),
        }
    }

    fn read_tildes(&mut self) -> Token {
        let mut count = 0;
        while self.peek_char() == Some(&'~') && count < 3 {
            self.advance();
            count += 1;
        }
        Token::Tilde(count)
    }

    fn read_text(&mut self, first_char: char) -> Result<Token, LexerError> {
        let mut text = String::new();
        text.push(first_char);
        self.advance();

        // Read ahead to collect full potential URL or text
        while let Some(&ch) = self.peek_char() {
            match ch {
                // Stop at markdown special characters
                '\n' | '\r' | ' ' | '\t' | '#' | '*' | '`' | '_' | '~' | '[' | ']' | '(' | ')'
                | '!' | '>' | '-' | '|' | '+' => break,
                _ => {
                    text.push(ch);
                    self.advance();
                }
            }
        }

        // Check if this looks like a URL
        if text.starts_with("http://")
            || text.starts_with("https://")
            || text.starts_with("ftp://")
            || text.starts_with("mailto:")
        {
            Ok(Token::Url(text))
        } else {
            Ok(Token::Text(text))
        }
    }

    fn read_whitespace(&mut self) -> Token {
        while let Some(&ch) = self.peek_char() {
            match ch {
                ' ' | '\t' => {
                    self.advance();
                }
                _ => break,
            }
        }
        Token::Whitespace
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn advance(&mut self) -> Option<char> {
        match self.input.next() {
            Some('\n') => {
                self.line += 1;
                self.column = 1;
                self.current_pos += 1;
                Some('\n')
            }
            Some(ch) => {
                self.column += 1;
                self.current_pos += 1;
                Some(ch)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markdown_parser::lexer::tokens::Token;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("# Hello World");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Hash(1));
        assert_eq!(tokens[1], Token::Whitespace);
        assert_eq!(tokens[2], Token::Text("Hello".to_string()));
        assert_eq!(tokens[3], Token::Whitespace);
        assert_eq!(tokens[4], Token::Text("World".to_string()));
        assert_eq!(tokens[5], Token::Eof);
    }

    #[test]
    fn test_multiple_hashes() {
        let mut lexer = Lexer::new("## Header");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Hash(2));
    }

    #[test]
    fn test_emphasis_tokens() {
        let mut lexer = Lexer::new("*bold* **italic** ***both***");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Asterisk(1));
        assert_eq!(tokens[2], Token::Asterisk(1));
        assert_eq!(tokens[4], Token::Asterisk(2));
        assert_eq!(tokens[6], Token::Asterisk(2));
        assert_eq!(tokens[8], Token::Asterisk(3));
    }

    #[test]
    fn test_code_tokens() {
        let mut lexer = Lexer::new("`code` ```block```");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Backtick(1));
        assert_eq!(tokens[4], Token::Backtick(3));
        assert_eq!(tokens[6], Token::Backtick(3));
    }

    #[test]
    fn test_link_tokens() {
        let mut lexer = Lexer::new("[link](url) ![image](src)");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::LeftBracket);
        assert_eq!(tokens[2], Token::RightBracket);
        assert_eq!(tokens[3], Token::LeftParen);
        assert_eq!(tokens[5], Token::RightParen);
        assert_eq!(tokens[7], Token::Exclamation);
        assert_eq!(tokens[8], Token::LeftBracket);
    }

    #[test]
    fn test_list_tokens() {
        let mut lexer = Lexer::new("- item\n+ item\n1. numbered");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Hyphen);
        assert_eq!(tokens[4], Token::Plus);
        assert_eq!(tokens[8], Token::Number(1));
        assert_eq!(tokens[9], Token::Dot);
    }

    #[test]
    fn test_table_tokens() {
        let mut lexer = Lexer::new("| col1 | col2 |\n|:-----|-----:|");
        let tokens = lexer.tokenize().unwrap();

        // Just check that we have pipe tokens
        let pipe_count = tokens.iter().filter(|t| matches!(t, Token::Pipe)).count();
        assert!(pipe_count >= 4);

        let colon_count = tokens.iter().filter(|t| matches!(t, Token::Colon)).count();
        assert!(colon_count >= 1);
    }

    #[test]
    fn test_strikethrough_tokens() {
        let mut lexer = Lexer::new("~~strikethrough~~");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Tilde(2));
        assert_eq!(tokens[2], Token::Tilde(2));
    }

    #[test]
    fn test_url_detection() {
        let mut lexer = Lexer::new("https://example.com");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Url("https://example.com".to_string()));
    }

    #[test]
    fn test_blockquote() {
        let mut lexer = Lexer::new("> quote");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::GreaterThan);
    }

    #[test]
    fn test_newlines() {
        let mut lexer = Lexer::new("line1\nline2\r\nline3");
        let tokens = lexer.tokenize().unwrap();

        // Find the newlines in the token stream
        let newline_positions: Vec<usize> = tokens
            .iter()
            .enumerate()
            .filter_map(|(i, token)| {
                if matches!(token, Token::Newline) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(newline_positions.len(), 2);
    }

    #[test]
    fn test_underscore_emphasis() {
        let mut lexer = Lexer::new("_italic_ __bold__ ___both___");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Underscore(1));
        assert_eq!(tokens[2], Token::Underscore(1));
        assert_eq!(tokens[4], Token::Underscore(2));
        assert_eq!(tokens[6], Token::Underscore(2));
        assert_eq!(tokens[8], Token::Underscore(3));
    }

    #[test]
    fn test_number_overflow() {
        let mut lexer = Lexer::new("99999999999999999999999999999");
        let result = lexer.tokenize();

        assert!(result.is_err());
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("line1\nline2");

        // Initially at line 1, column 1
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.column, 1);

        // Advance through first line
        for _ in 0..5 {
            lexer.advance();
        }
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.column, 6);

        // Advance through newline
        lexer.advance();
        assert_eq!(lexer.line, 2);
        assert_eq!(lexer.column, 1);
    }

    // Edge Case Tests
    #[test]
    fn test_empty_string() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Eof);
    }

    #[test]
    fn test_only_whitespace() {
        let mut lexer = Lexer::new("   \t  \t ");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Whitespace);
        assert_eq!(tokens[1], Token::Eof);
    }

    #[test]
    fn test_only_newlines() {
        let mut lexer = Lexer::new("\n\n\r\n\n");
        let tokens = lexer.tokenize().unwrap();

        let newline_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::Newline))
            .count();
        assert_eq!(newline_count, 4);
    }

    #[test]
    fn test_maximum_hashes() {
        let mut lexer = Lexer::new("######");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Hash(6));
    }

    #[test]
    fn test_excessive_hashes() {
        let mut lexer = Lexer::new("##########");
        let tokens = lexer.tokenize().unwrap();

        // Should stop at 6 hashes
        assert_eq!(tokens[0], Token::Hash(6));
        assert_eq!(tokens[1], Token::Hash(4));
    }

    #[test]
    fn test_maximum_asterisks() {
        let mut lexer = Lexer::new("***");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Asterisk(3));
    }

    #[test]
    fn test_excessive_asterisks() {
        let mut lexer = Lexer::new("******");
        let tokens = lexer.tokenize().unwrap();

        // Should stop at 3 asterisks
        assert_eq!(tokens[0], Token::Asterisk(3));
        assert_eq!(tokens[1], Token::Asterisk(3));
    }

    #[test]
    fn test_maximum_backticks() {
        let mut lexer = Lexer::new("````");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Backtick(4));
    }

    #[test]
    fn test_excessive_backticks() {
        let mut lexer = Lexer::new("``````");
        let tokens = lexer.tokenize().unwrap();

        // Should stop at 4 backticks
        assert_eq!(tokens[0], Token::Backtick(4));
        assert_eq!(tokens[1], Token::Backtick(2));
    }

    #[test]
    fn test_maximum_underscores() {
        let mut lexer = Lexer::new("___");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Underscore(3));
    }

    #[test]
    fn test_excessive_underscores() {
        let mut lexer = Lexer::new("______");
        let tokens = lexer.tokenize().unwrap();

        // Should stop at 3 underscores
        assert_eq!(tokens[0], Token::Underscore(3));
        assert_eq!(tokens[1], Token::Underscore(3));
    }

    #[test]
    fn test_maximum_tildes() {
        let mut lexer = Lexer::new("~~~");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Tilde(3));
    }

    #[test]
    fn test_excessive_tildes() {
        let mut lexer = Lexer::new("~~~~~~");
        let tokens = lexer.tokenize().unwrap();

        // Should stop at 3 tildes
        assert_eq!(tokens[0], Token::Tilde(3));
        assert_eq!(tokens[1], Token::Tilde(3));
    }

    #[test]
    fn test_zero_number() {
        let mut lexer = Lexer::new("0");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Number(0));
    }

    #[test]
    fn test_large_valid_number() {
        let mut lexer = Lexer::new("4294967295"); // Max u32
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Number(4294967295));
    }

    #[test]
    fn test_number_with_leading_zeros() {
        let mut lexer = Lexer::new("00123");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Number(123));
    }

    #[test]
    fn test_mixed_line_endings() {
        let mut lexer = Lexer::new("line1\nline2\r\nline3\rline4");
        let tokens = lexer.tokenize().unwrap();

        let newline_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::Newline))
            .count();
        assert_eq!(newline_count, 3);
    }

    #[test]
    fn test_carriage_return_only() {
        let mut lexer = Lexer::new("line1\rline2");
        let tokens = lexer.tokenize().unwrap();

        assert!(tokens.iter().any(|t| matches!(t, Token::Newline)));
    }

    #[test]
    fn test_unicode_text() {
        let mut lexer = Lexer::new("Hello 世界 🌍");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Text("Hello".to_string()));
        assert_eq!(tokens[2], Token::Text("世界".to_string()));
        assert_eq!(tokens[4], Token::Text("🌍".to_string()));
    }

    #[test]
    fn test_special_characters_in_text() {
        let mut lexer = Lexer::new("hello@world.com $100 %test &more");
        let tokens = lexer.tokenize().unwrap();

        // Should capture special characters as part of text
        assert!(tokens.iter().any(|t| {
            if let Token::Text(text) = t {
                text.contains('@') || text.contains('$') || text.contains('%') || text.contains('&')
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_url_variations() {
        let test_cases = vec![
            ("http://example.com", true),
            ("https://example.com", true),
            ("ftp://example.com", true),
            ("mailto:test@example.com", true),
            ("file://local/path", false), // Not supported
            ("www.example.com", false),   // Not a full URL
        ];

        for (input, should_be_url) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize().unwrap();

            if should_be_url {
                assert!(matches!(tokens[0], Token::Url(_)), "Failed for: {}", input);
            } else {
                assert!(matches!(tokens[0], Token::Text(_)), "Failed for: {}", input);
            }
        }
    }

    #[test]
    fn test_complex_url() {
        let mut lexer = Lexer::new("https://example.com/path?param=value&other=123#anchor");
        let tokens = lexer.tokenize().unwrap();

        // URL should capture everything until whitespace/special chars
        if let Token::Url(url) = &tokens[0] {
            assert!(url.starts_with("https://example.com"));
            assert!(url.contains("path"));
        } else {
            panic!("Expected URL token");
        }
    }

    #[test]
    fn test_consecutive_special_characters() {
        let mut lexer = Lexer::new("**##__``~~");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Asterisk(2));
        assert_eq!(tokens[1], Token::Hash(2));
        assert_eq!(tokens[2], Token::Underscore(2));
        assert_eq!(tokens[3], Token::Backtick(2));
        assert_eq!(tokens[4], Token::Tilde(2));
    }

    #[test]
    fn test_mixed_brackets() {
        let mut lexer = Lexer::new("[({})]");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::LeftBracket);
        assert_eq!(tokens[1], Token::LeftParen);
        // Braces should be captured as text
        let has_brace_text = tokens.iter().any(|t| {
            if let Token::Text(text) = t {
                text.contains('{') || text.contains('}')
            } else {
                false
            }
        });
        assert!(has_brace_text);
    }

    #[test]
    fn test_table_edge_cases() {
        let mut lexer = Lexer::new("||:|::|:::|");
        let tokens = lexer.tokenize().unwrap();

        let pipe_count = tokens.iter().filter(|t| matches!(t, Token::Pipe)).count();
        let colon_count = tokens.iter().filter(|t| matches!(t, Token::Colon)).count();

        assert!(pipe_count >= 4);
        assert_eq!(colon_count, 6);
    }

    #[test]
    fn test_list_markers_with_spaces() {
        let mut lexer = Lexer::new("- item\n+ item\n* item\n1. numbered\n42. another");
        let tokens = lexer.tokenize().unwrap();

        assert!(tokens.iter().any(|t| matches!(t, Token::Hyphen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Plus)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Asterisk(1))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Number(1))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Number(42))));
        assert!(tokens.iter().filter(|t| matches!(t, Token::Dot)).count() >= 2);
    }

    #[test]
    fn test_emphasis_edge_cases() {
        let mut lexer = Lexer::new("*a*b* **c**d** ***e***f***");
        let tokens = lexer.tokenize().unwrap();

        // Check for proper asterisk grouping
        let asterisk_tokens: Vec<_> = tokens
            .iter()
            .filter_map(|t| {
                if let Token::Asterisk(count) = t {
                    Some(*count)
                } else {
                    None
                }
            })
            .collect();

        assert!(asterisk_tokens.contains(&1));
        assert!(asterisk_tokens.contains(&2));
        assert!(asterisk_tokens.contains(&3));
    }

    #[test]
    fn test_blockquote_variations() {
        let mut lexer = Lexer::new("> quote\n>> nested\n> > spaced");
        let tokens = lexer.tokenize().unwrap();

        let gt_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::GreaterThan))
            .count();
        assert!(gt_count >= 4);
    }

    #[test]
    fn test_code_block_edge_cases() {
        let mut lexer = Lexer::new("`inline` ``empty`` ```\nblock\n``` ````four````");
        let tokens = lexer.tokenize().unwrap();

        let backtick_tokens: Vec<_> = tokens
            .iter()
            .filter_map(|t| {
                if let Token::Backtick(count) = t {
                    Some(*count)
                } else {
                    None
                }
            })
            .collect();

        assert!(backtick_tokens.contains(&1));
        assert!(backtick_tokens.contains(&2));
        assert!(backtick_tokens.contains(&3));
        assert!(backtick_tokens.contains(&4));
    }

    #[test]
    fn test_mixed_whitespace() {
        let mut lexer = Lexer::new("  \t  \t  text  \t  ");
        let tokens = lexer.tokenize().unwrap();

        // Should group consecutive whitespace
        assert_eq!(tokens[0], Token::Whitespace);
        assert_eq!(tokens[1], Token::Text("text".to_string()));
        assert_eq!(tokens[2], Token::Whitespace);
    }

    #[test]
    fn test_numbers_in_text() {
        let mut lexer = Lexer::new("version2.0 test123 abc");
        let tokens = lexer.tokenize().unwrap();

        // Text with numbers should be handled properly
        let has_version_text = tokens.iter().any(|t| {
            if let Token::Text(text) = t {
                text.contains("version") || text.contains("test")
            } else {
                false
            }
        });
        assert!(has_version_text);
    }

    #[test]
    fn test_punctuation_combinations() {
        let mut lexer = Lexer::new("... !!! --- +++ |||");
        let tokens = lexer.tokenize().unwrap();

        // Should handle repeated punctuation
        let dot_count = tokens.iter().filter(|t| matches!(t, Token::Dot)).count();
        let excl_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::Exclamation))
            .count();
        let hyphen_count = tokens.iter().filter(|t| matches!(t, Token::Hyphen)).count();
        let plus_count = tokens.iter().filter(|t| matches!(t, Token::Plus)).count();
        let pipe_count = tokens.iter().filter(|t| matches!(t, Token::Pipe)).count();

        assert_eq!(dot_count, 3);
        assert_eq!(excl_count, 3);
        assert_eq!(hyphen_count, 3);
        assert_eq!(plus_count, 3);
        assert_eq!(pipe_count, 3);
    }

    #[test]
    fn test_url_in_complex_text() {
        let mut lexer = Lexer::new("Visit https://example.com for more info");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Text("Visit".to_string()));
        assert_eq!(tokens[2], Token::Url("https://example.com".to_string()));
        assert_eq!(tokens[4], Token::Text("for".to_string()));
    }

    #[test]
    fn test_edge_case_combinations() {
        let mut lexer = Lexer::new("#*`_~[]()|:.-+123");
        let tokens = lexer.tokenize().unwrap();

        // Should tokenize each special character separately
        assert_eq!(tokens[0], Token::Hash(1));
        assert_eq!(tokens[1], Token::Asterisk(1));
        assert_eq!(tokens[2], Token::Backtick(1));
        assert_eq!(tokens[3], Token::Underscore(1));
        assert_eq!(tokens[4], Token::Tilde(1));
        assert_eq!(tokens[5], Token::LeftBracket);
        assert_eq!(tokens[6], Token::RightBracket);
        assert_eq!(tokens[7], Token::LeftParen);
        assert_eq!(tokens[8], Token::RightParen);
        assert_eq!(tokens[9], Token::Pipe);
        assert_eq!(tokens[10], Token::Colon);
        assert_eq!(tokens[11], Token::Dot);
        assert_eq!(tokens[12], Token::Hyphen);
        assert_eq!(tokens[13], Token::Plus);
        assert_eq!(tokens[14], Token::Number(123));
    }

    #[test]
    fn test_position_tracking_multiline() {
        let mut lexer = Lexer::new("line1\nline2\nline3");
        let mut line_positions = Vec::new();
        let mut column_positions = Vec::new();

        // Track positions during tokenization
        while lexer.peek_char().is_some() {
            line_positions.push(lexer.line);
            column_positions.push(lexer.column);
            lexer.advance();
        }

        // Should have proper line tracking
        assert!(line_positions.contains(&1));
        assert!(line_positions.contains(&2));
        assert!(line_positions.contains(&3));

        // Column should reset after newlines
        assert!(column_positions.contains(&1));
    }

    #[test]
    fn test_complex_markdown_document() {
        let complex_md = r#"# Header 1
## Header 2

This is **bold** and *italic* text with `inline code`.

- List item 1
- List item 2
  - Nested item

1. Numbered list
2. Another item

> Blockquote with **bold** text

```rust
let code = "block";
```

[Link](https://example.com) and ![Image](image.png)

| Table | Header |
|:------|-------:|
| Cell  | Value  |

~~Strikethrough~~ text.

---"#;

        let mut lexer = Lexer::new(complex_md);
        let tokens = lexer.tokenize().unwrap();

        // Verify various token types exist
        assert!(tokens.iter().any(|t| matches!(t, Token::Hash(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Asterisk(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Backtick(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Hyphen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Number(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::GreaterThan)));
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftBracket)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightBracket)));
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftParen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightParen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Exclamation)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Pipe)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Colon)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Tilde(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Url(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Text(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Newline)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Whitespace)));

        // Should end with EOF
        assert_eq!(tokens.last(), Some(&Token::Eof));
    }

    // Stress Tests and Additional Boundary Conditions
    #[test]
    fn test_very_long_text() {
        let long_text = "a".repeat(10000);
        let mut lexer = Lexer::new(&long_text);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // Text + EOF
        if let Token::Text(text) = &tokens[0] {
            assert_eq!(text.len(), 10000);
        }
    }

    #[test]
    fn test_many_consecutive_newlines() {
        let input = "\n".repeat(1000);
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize().unwrap();

        let newline_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::Newline))
            .count();
        assert_eq!(newline_count, 1000);
    }

    #[test]
    fn test_alternating_patterns() {
        let mut lexer = Lexer::new("*_*_*_*_*_*_*_*_*_");
        let tokens = lexer.tokenize().unwrap();

        // Should alternate between asterisks and underscores
        for (i, token) in tokens.iter().enumerate() {
            if i % 2 == 0 && i < tokens.len() - 1 {
                assert!(matches!(token, Token::Asterisk(1) | Token::Underscore(1)));
            }
        }
    }

    #[test]
    fn test_nested_emphasis_markers() {
        let mut lexer = Lexer::new("***___***___***");
        let tokens = lexer.tokenize().unwrap();

        let asterisk_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::Asterisk(_)))
            .count();
        let underscore_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::Underscore(_)))
            .count();

        assert!(asterisk_count > 0);
        assert!(underscore_count > 0);
    }

    #[test]
    fn test_all_single_characters() {
        let chars = "!()[]|:~`";
        let mut lexer = Lexer::new(chars);
        let tokens = lexer.tokenize().unwrap();

        // Should handle all these characters (some as special tokens, some as text)
        assert!(tokens.len() > 1);
        assert!(tokens.iter().any(|t| matches!(t, Token::Exclamation)));
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftBracket)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightBracket)));
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftParen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightParen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Pipe)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Colon)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Tilde(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Backtick(_))));

        // Test that other characters become text
        let mut lexer2 = Lexer::new("@#$%^&");
        let tokens2 = lexer2.tokenize().unwrap();
        assert!(tokens2.iter().any(|t| matches!(t, Token::Text(_))));
    }

    #[test]
    fn test_boundary_numbers() {
        let test_cases = vec![
            ("0", Some(0u32)),
            ("1", Some(1u32)),
            ("4294967295", Some(u32::MAX)), // Max u32
            ("4294967296", None),           // Overflow
            ("99999999999999999999", None), // Way too big
        ];

        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let result = lexer.tokenize();

            match expected {
                Some(num) => {
                    let tokens = result.unwrap();
                    assert_eq!(tokens[0], Token::Number(num));
                }
                None => {
                    assert!(result.is_err());
                }
            }
        }
    }

    #[test]
    fn test_mixed_unicode_and_ascii() {
        let mut lexer = Lexer::new("Hello 世界! こんにちは # Header 🚀");
        let tokens = lexer.tokenize().unwrap();

        // Should handle Unicode properly
        let has_unicode = tokens.iter().any(|t| {
            if let Token::Text(text) = t {
                text.chars().any(|c| !c.is_ascii())
            } else {
                false
            }
        });
        assert!(has_unicode);

        // Should still recognize ASCII markdown tokens
        assert!(tokens.iter().any(|t| matches!(t, Token::Hash(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Exclamation)));
    }

    #[test]
    fn test_malformed_markdown_patterns() {
        let test_cases = vec![
            "###", // Incomplete header
            "***", // Incomplete emphasis
            "```", // Incomplete code block
            "|||", // Multiple pipes
            ":::", // Multiple colons
            "---", // Multiple hyphens
            "+++", // Multiple plus signs
        ];

        for input in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize().unwrap();

            // Should tokenize without errors, even if markdown is malformed
            assert!(tokens.len() >= 2); // At least some tokens + EOF
            assert_eq!(tokens.last(), Some(&Token::Eof));
        }
    }

    #[test]
    fn test_position_accuracy_with_complex_content() {
        let content = "Line 1\n  Line 2\n\tLine 3\n";
        let mut lexer = Lexer::new(content);

        // Manually track position while advancing
        let mut positions = Vec::new();
        while lexer.peek_char().is_some() {
            positions.push((lexer.line, lexer.column));
            lexer.advance();
        }

        // Verify line tracking
        assert!(positions.iter().any(|(line, _)| *line == 1));
        assert!(positions.iter().any(|(line, _)| *line == 2));
        assert!(positions.iter().any(|(line, _)| *line == 3));

        // Verify column resets
        let line_starts: Vec<_> = positions.iter().filter(|(_, col)| *col == 1).collect();
        assert!(line_starts.len() >= 3); // At least 3 line starts
    }

    #[test]
    fn test_url_edge_cases() {
        let test_cases = vec![
            ("http://", true),
            ("https://", true),
            ("ftp://", true),
            ("mailto:", true),
            ("http://a", true),
            ("https://localhost:8080", true),
            (
                "https://example.com/path/to/file.html?query=value#fragment",
                true,
            ),
            ("not-a-url", false),
            ("ht tp://broken", false),
        ];

        for (input, should_be_url) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize().unwrap();

            let is_url = matches!(tokens[0], Token::Url(_));
            assert_eq!(is_url, should_be_url, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_whitespace_variations() {
        let inputs = vec![
            " ",        // Single space
            "  ",       // Multiple spaces
            "\t",       // Single tab
            "\t\t",     // Multiple tabs
            " \t ",     // Mixed
            "\t \t \t", // Complex mix
        ];

        for input in inputs {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize().unwrap();

            assert_eq!(tokens[0], Token::Whitespace);
            assert_eq!(tokens[1], Token::Eof);
        }
    }

    #[test]
    fn test_extreme_nesting() {
        let nested = "[[[[[[[[[text]]]]]]]]]";
        let mut lexer = Lexer::new(nested);
        let tokens = lexer.tokenize().unwrap();

        let left_bracket_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::LeftBracket))
            .count();
        let right_bracket_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::RightBracket))
            .count();

        assert_eq!(left_bracket_count, 9);
        assert_eq!(right_bracket_count, 9);
    }

    #[test]
    fn test_mixed_emphasis_with_text() {
        let input = "**bold** *italic* __underline__ _emphasis_ ~~strike~~ `code`";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        // Verify we have the right mix of tokens
        assert!(tokens.iter().any(|t| matches!(t, Token::Asterisk(2))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Asterisk(1))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Underscore(2))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Underscore(1))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Tilde(2))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Backtick(1))));
        assert!(tokens.iter().any(|t| {
            if let Token::Text(text) = t {
                text == "bold"
                    || text == "italic"
                    || text == "underline"
                    || text == "emphasis"
                    || text == "strike"
                    || text == "code"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_error_recovery() {
        // Test that lexer can continue after encountering an error
        let input = "normal text 99999999999999999999999 more text";
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();

        // Should fail on the huge number
        assert!(result.is_err());

        // But we should be able to create a new lexer for the rest
        let mut lexer2 = Lexer::new("more text");
        let tokens = lexer2.tokenize().unwrap();
        assert!(tokens.iter().any(|t| matches!(t, Token::Text(_))));
    }
}
