use std::collections::HashMap;
use std::{fmt, str};

use crate::css::{Selector, StyleSheet, Value};
use crate::dom::{ElementData, Node, NodeType};

type PropertyMap<'a> = HashMap<&'a str, &'a Value>;

pub struct StyledNode<'a> {
    node: &'a Node,
    styles: PropertyMap<'a>,
    pub(crate) children: Vec<StyledNode<'a>>,
}

pub enum Display {
    Block,
    Inline,
    InlineBlock,
    None,
}

impl<'a> StyledNode<'a> {
    pub fn new(node: &'a Node, stylesheet: &'a StyleSheet) -> StyledNode<'a> {
        let mut style_children = Vec::new();

        for child in &node.children {
            match child.node_type {
                NodeType::Element(_) => style_children.push(StyledNode::new(&child, stylesheet)),
                _ => {}
            }
        }

        StyledNode {
            node,
            styles: match node.node_type {
                NodeType::Element(ref element) => StyledNode::get_styles(element, stylesheet),
                _ => PropertyMap::new(),
            },
            children: style_children,
        }
    }

    fn get_styles(element: &'a ElementData, stylesheet: &'a StyleSheet) -> PropertyMap<'a> {
        let mut styles = PropertyMap::new();

        for rule in &stylesheet.rules {
            for selector in &rule.selectors {
                if selector_matches(element, &selector) {
                    for declaration in &rule.declarations {
                        styles.insert(&declaration.property, &declaration.value);
                    }
                    break;
                }
            }
        }
        styles
    }

    pub fn value(&self, name: &str) -> Option<&&Value> {
        self.styles.get(name)
    }

    pub fn get_display(&self) -> Display {
        match self.value("display") {
            Some(string) => match **string {
                Value::Other(ref value) => match value.as_ref() {
                    "block" => Display::Block,
                    "none" => Display::None,
                    "inline-block" => Display::InlineBlock,
                    _ => Display::Inline,
                },
                _ => Display::Inline,
            },
            None => Display::Inline,
        }
    }

    pub fn num_or(&self, name: &str, default: f32) -> f32 {
        match self.value(name) {
            Some(value) => match **value {
                Value::Length(len, _) => len,
                _ => default,
            },
            None => default,
        }
    }
}

impl<'a> fmt::Debug for StyledNode<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "{0:?}: {1:?}", self.node, self.styles)
    }
}

fn selector_matches(element: &ElementData, selector: &Selector) -> bool {
    for simple in &selector.simple {
        let mut selector_match = true;

        match simple.tag_name {
            Some(ref tag) => {
                if *tag != element.tag_name {
                    continue;
                }
            }
            None => {}
        };

        match element.get_id() {
            Some(element_id) => match simple.id {
                Some(ref selector_id) => {
                    if *element_id != *selector_id {
                        continue;
                    }
                }
                None => {}
            },
            None => match simple.id {
                Some(_) => {
                    continue;
                }
                _ => {}
            },
        }

        let element_classes = element.get_classes();
        for class in &simple.classes {
            selector_match &= element_classes.contains::<str>(class);
        }

        if selector_match {
            return true;
        }
    }

    false
}

pub fn pretty_print(node: &StyledNode, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();
    println!("{0}{1:?}", indent, node);

    for child in node.children.iter() {
        pretty_print(&child, indent_size + 2);
    }
}
