#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Document { children: Vec<AstNode> },

    // Block elements
    Heading { level: u8, content: Vec<AstNode> },
    Paragraph { content: Vec<AstNode> },
    List { ordered: bool, items: Vec<AstNode> },
    ListItem { content: Vec<AstNode> },
    BlockQuote { content: Vec<AstNode> },
    CodeBlock { language: Option<String>, code: String },
    HorizontalRule,
    Table { headers: Vec<AstNode>, rows: Vec<Vec<AstNode>> },
    TableCell { content: Vec<AstNode> },
    TableRow { cells: Vec<AstNode> },

    // Inline elements
    Text(String),
    Bold(Vec<AstNode>),
    Italic(Vec<AstNode>),
    Strikethrough(Vec<AstNode>),
    InlineCode(String),
    Link { text: Vec<AstNode>, url: String },
    Image { alt: Vec<AstNode>, url: String },

    LineBreak,
}

impl AstNode {
    /// Check if this node is an inline element
    pub fn is_inline(&self) -> bool {
        matches!(self,
            AstNode::Text(_) |
            AstNode::Bold(_) |
            AstNode::Italic(_) |
            AstNode::Strikethrough(_) |
            AstNode::InlineCode(_) |
            AstNode::Link { .. } |
            AstNode::Image { .. } |
            AstNode::LineBreak
        )
    }

    /// Check if this node is a block element
    pub fn is_block(&self) -> bool {
        !self.is_inline()
    }

    /// Get the text content of this node recursively
    pub fn text_content(&self) -> String {
        match self {
            AstNode::Text(text) => text.clone(),
            AstNode::InlineCode(code) => code.clone(),
            AstNode::Bold(children) |
            AstNode::Italic(children) |
            AstNode::Strikethrough(children) |
            AstNode::Heading { content: children, .. } |
            AstNode::Paragraph { content: children } |
            AstNode::ListItem { content: children } |
            AstNode::BlockQuote { content: children } |
            AstNode::TableCell { content: children } => {
                children.iter().map(|child| child.text_content()).collect::<Vec<_>>().join("")
            }
            AstNode::Link { text, .. } |
            AstNode::Image { alt: text, .. } => {
                text.iter().map(|child| child.text_content()).collect::<Vec<_>>().join("")
            }
            AstNode::List { items, .. } => {
                items.iter().map(|item| item.text_content()).collect::<Vec<_>>().join("\n")
            }
            AstNode::Table { headers, rows } => {
                let header_text = headers.iter().map(|h| h.text_content()).collect::<Vec<_>>().join(" | ");
                let row_texts: Vec<String> = rows.iter().map(|row| {
                    row.iter().map(|cell| cell.text_content()).collect::<Vec<_>>().join(" | ")
                }).collect();
                format!("{}\n{}", header_text, row_texts.join("\n"))
            }
            AstNode::TableRow { cells } => {
                cells.iter().map(|cell| cell.text_content()).collect::<Vec<_>>().join(" | ")
            }
            AstNode::CodeBlock { code, .. } => code.clone(),
            AstNode::Document { children } => {
                children.iter().map(|child| child.text_content()).collect::<Vec<_>>().join("\n")
            }
            AstNode::HorizontalRule => "---".to_string(),
            AstNode::LineBreak => "\n".to_string(),
        }
    }

    /// Count the number of child nodes recursively
    pub fn count_children(&self) -> usize {
        match self {
            AstNode::Document { children } |
            AstNode::Bold(children) |
            AstNode::Italic(children) |
            AstNode::Strikethrough(children) |
            AstNode::Heading { content: children, .. } |
            AstNode::Paragraph { content: children } |
            AstNode::ListItem { content: children } |
            AstNode::BlockQuote { content: children } |
            AstNode::TableCell { content: children } => {
                children.len() + children.iter().map(|child| child.count_children()).sum::<usize>()
            }
            AstNode::Link { text, .. } |
            AstNode::Image { alt: text, .. } => {
                text.len() + text.iter().map(|child| child.count_children()).sum::<usize>()
            }
            AstNode::List { items, .. } => {
                items.len() + items.iter().map(|item| item.count_children()).sum::<usize>()
            }
            AstNode::Table { headers, rows } => {
                let header_count = headers.len() + headers.iter().map(|h| h.count_children()).sum::<usize>();
                let row_count = rows.iter().map(|row| {
                    row.len() + row.iter().map(|cell| cell.count_children()).sum::<usize>()
                }).sum::<usize>();
                header_count + row_count
            }
            AstNode::TableRow { cells } => {
                cells.len() + cells.iter().map(|cell| cell.count_children()).sum::<usize>()
            }
            _ => 0, // Leaf nodes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_inline() {
        assert!(AstNode::Text("hello".to_string()).is_inline());
        assert!(AstNode::Bold(vec![]).is_inline());
        assert!(AstNode::LineBreak.is_inline());
        assert!(!AstNode::Heading { level: 1, content: vec![] }.is_inline());
        assert!(!AstNode::Paragraph { content: vec![] }.is_inline());
    }

    #[test]
    fn test_is_block() {
        assert!(!AstNode::Text("hello".to_string()).is_block());
        assert!(AstNode::Heading { level: 1, content: vec![] }.is_block());
        assert!(AstNode::Paragraph { content: vec![] }.is_block());
    }

    #[test]
    fn test_text_content() {
        let text_node = AstNode::Text("Hello".to_string());
        assert_eq!(text_node.text_content(), "Hello");

        let bold_node = AstNode::Bold(vec![
            AstNode::Text("Bold".to_string()),
            AstNode::Text(" text".to_string())
        ]);
        assert_eq!(bold_node.text_content(), "Bold text");

        let heading = AstNode::Heading {
            level: 1,
            content: vec![AstNode::Text("Title".to_string())]
        };
        assert_eq!(heading.text_content(), "Title");
    }

    #[test]
    fn test_count_children() {
        let simple_text = AstNode::Text("hello".to_string());
        assert_eq!(simple_text.count_children(), 0);

        let paragraph = AstNode::Paragraph {
            content: vec![
                AstNode::Text("Hello".to_string()),
                AstNode::Bold(vec![AstNode::Text("world".to_string())])
            ]
        };
        assert_eq!(paragraph.count_children(), 3); // 2 direct children + 1 nested child

        let document = AstNode::Document {
            children: vec![paragraph]
        };
        assert_eq!(document.count_children(), 4); // 1 direct + 3 nested
    }
}
