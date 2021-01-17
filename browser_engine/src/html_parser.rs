use std::iter::Peekable;
use std::str::Chars;

use crate::dom::{AttributeMap, ElementData, Node, NodeType};

pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>,
    node_queue: Vec<String>,
}

impl<'a> HtmlParser<'a> {
    pub fn new(full_html: &str) -> HtmlParser {
        HtmlParser {
            chars: full_html.chars().peekable(),
            node_queue: Vec::new(),
        }
    }

    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.chars.peek().is_some() {
            self.consume_while(char::is_whitespace);

            if self.chars.peek().map_or(false, |c| *c == '<') {
                self.chars.next();

                if self.chars.peek().map_or(false, |c| *c == '/') {
                    self.chars.next();
                    self.consume_while(char::is_whitespace);

                    let closing_tag_name = self.consume_while(is_valid_tag_name);

                    self.consume_while(|x| x != '>');
                    self.chars.next();

                    self.node_queue.push(closing_tag_name);
                    break;
                } else if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    nodes.push(self.parse_comment_node());
                } else {
                    let mut node = self.parse_node();
                    let insert_index = nodes.len();

                    match node.node_type {
                        NodeType::Element(ref element) => {
                            if self.node_queue.len() > 0 {
                                let assumed_tag = self.node_queue.remove(0);

                                if element.tag_name != assumed_tag {
                                    nodes.append(&mut node.children);
                                    self.node_queue.insert(0, assumed_tag);
                                }
                            }
                        }
                        _ => {}
                    }

                    nodes.insert(insert_index, node);
                }
            } else {
                nodes.push(self.parse_text());
            }
        }

        nodes
    }

    fn parse_node(&mut self) -> Node {
        let tag_name = self.consume_while(is_valid_tag_name);
        let attributes = self.parse_attributes();

        let element = ElementData::new(tag_name, attributes);
        let children = self.parse_nodes();

        Node::new(NodeType::Element(element), children)
    }

    fn parse_text(&mut self) -> Node {
        let mut text_content = String::new();

        while self.chars.peek().map_or(false, |c| *c != '<') {
            let whitespace = self.consume_while(char::is_whitespace);
            if whitespace.len() > 0 {
                text_content.push(' ');
            }
            let text_part = self.consume_while(|char| !char.is_whitespace() && char != '<');
            text_content.push_str(&text_part);
        }

        Node::new(NodeType::Text(text_content), Vec::new())
    }

    fn parse_comment_node(&mut self) -> Node {
        let mut comment_content = String::new();

        if self.chars.peek().map_or(false, |c| *c == '-') {
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '-') {
                self.chars.next();
            } else {
                self.consume_while(|c| c != '>');
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            }
        } else {
            self.consume_while(|c| c != '>');
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }

        if self.chars.peek().map_or(false, |c| *c != '>') {
            self.chars.next();
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }

        if self.chars.peek().map_or(false, |c| *c != '-') {
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c != '>') {
                self.chars.next();
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            } else {
                comment_content.push('-');
            }
        }

        while self.chars.peek().is_some() {
            comment_content.push_str(&self.consume_while(|c| c != '<' && c != '-'));
            if self.chars.peek().map_or(false, |c| *c == '<') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '-') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.consume_while(|c| c != '>');

                            return Node::new(NodeType::Comment(String::from("")), Vec::new());
                        } else {
                            comment_content.push_str("<!-");
                        }
                    } else if self.chars.peek().map_or(false, |c| *c == ' ') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.chars.next();
                            if self.chars.peek().map_or(false, |c| *c == '-') {
                                self.chars.next();
                                if self.chars.peek().map_or(false, |c| *c == '-') {
                                    self.chars.next();
                                    if self.chars.peek().map_or(false, |c| *c == '>') {
                                        self.chars.next();
                                        return Node::new(
                                            NodeType::Comment(String::from("")),
                                            Vec::new(),
                                        );
                                    } else {
                                        comment_content.push_str("<! --");
                                    }
                                } else {
                                    comment_content.push_str("<! -");
                                }
                            } else {
                                comment_content.push_str("<! ");
                            }
                        }
                    } else {
                        comment_content.push_str("<!");
                    }
                } else {
                    comment_content.push('<');
                }
            } else if self.chars.peek().map_or(false, |c| *c == '-') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '-') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '>') {
                        self.chars.next();
                        break;
                    } else {
                        comment_content.push_str("--");
                    }
                } else {
                    comment_content.push('-');
                }
            }
        }

        Node::new(NodeType::Comment(comment_content), Vec::new())
    }

    fn parse_attributes(&mut self) -> AttributeMap {
        let mut attributes = AttributeMap::new();

        while self.chars.peek().map_or(false, |c| *c != '>') {
            self.consume_while(char::is_whitespace);
            let name = self
                .consume_while(|char| is_valid_attribute_name(char))
                .to_lowercase();
            self.consume_while(char::is_whitespace);

            let value = if self.chars.peek().map_or(false, |c| *c == '=') {
                self.chars.next();
                self.consume_while(char::is_whitespace);
                let value_string = self.parse_attribute_values();
                self.consume_while(|char| !char.is_whitespace() && char != '>');
                self.consume_while(char::is_whitespace);
                value_string
            } else {
                "".to_string()
            };

            attributes.insert(name, value);
        }

        self.chars.next();

        attributes
    }

    fn parse_attribute_values(&mut self) -> String {
        self.consume_while(char::is_whitespace);

        let string = match self.chars.next() {
            Some(char) if char == '"' || char == '\'' => {
                self.chars.next();
                let parsed_value = self.consume_while(|value| value != char);
                self.chars.next();

                parsed_value
            }
            _ => self.consume_while(is_valid_attribute_value),
        };

        string
    }

    fn consume_while<T>(&mut self, condition: T) -> String
    where
        T: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }

        result
    }
}

fn is_valid_tag_name(char: char) -> bool {
    char.is_digit(36)
}

fn is_control(char: char) -> bool {
    match char {
        '\u{007F}' => true,
        char if char >= '\u{0000}' && char <= '\u{001F}' => true,
        char if char >= '\u{0080}' && char <= '\u{009F}' => true,
        _ => false,
    }
}

fn is_excluded_name(char: char) -> bool {
    match char {
        ' ' | '"' | '\'' | '>' | '/' | '=' => true,
        _ => false,
    }
}

fn is_valid_attribute_name(char: char) -> bool {
    !is_excluded_name(char) && !is_control(char)
}

fn is_valid_attribute_value(char: char) -> bool {
    match char {
        ' ' | '"' | '\'' | '=' | '<' | '>' | '`' => false,
        _ => true,
    }
}
