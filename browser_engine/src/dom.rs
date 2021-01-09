use std::collections::{HashMap, HashSet};
use std::fmt;

type AttributeMap = HashMap<String, String>;

struct ElementData {
	tag_name: String,
	attributes: AttributeMap,
}

enum NodeType {
	Text(String),
	Element(ElementData),
	Comment(String),
}

struct Node {
	children: Vec<Node>,
	node_type: NodeType,
}

impl fmt::Debug for ElementData {
	fn fmt(&self, format: &mut fmt::Formatter) {
		let mut attribute_string = String::new();

		for (attribute, value) in self.attributes.into_iter() {
			attribute_string.push_str(&format!(" {0}=\"{1}\"", attribute, value));
		}
		write!(format, "<{0},{1}>", self.tag_name, attribute_string);
	}
}

impl fmt::Debug for NodeType {
	fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			NodeType::Text(ref text) | NodeType::Comment(ref text) => write!(format, "{}", text),
			NodeType::Element(ref element) => write!(format, "{:?}", element),
		}
	}
}

impl fmt::Debug for Node {
	fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
		write!(format, "{:?}", self.node_type)
	}
}

impl Node {
	fn new(node_type: NodeType, children: Vec<Node>) -> Node {
		Node {
			node_type,
			children,
		}
	}
}

impl ElementData {
	fn new(tag_name: String, attributes: AttributeMap) -> ElementData {
		ElementData {
			tag_name,
			attributes,
		}
	}

	fn get_id(&self) -> Option<&String> {
		self.attributes.get("id")
	}

	fn get_classes(&self) -> HashSet<&str> {
		match self.attributes.get("class") {
			Some(string) => string.split(" ").collect(),
			None => HashSet::new(),
		}
	}
}

fn pretty_print(node: &Node, indent_size: usize) {
	let indent = (0..indent_size).map(|_| " ").collect::<String>();

	match node.node_type {
		NodeType::Element(ref element) => println!("{0}{1:?}",indent, element),
		NodeType::Text(ref text ) => println!("{0}{1}", indent, text),
		NodeType::Comment(ref comment) => println!("{0}<!--{1}-->", indent, comment),
	}

	for child in node.children.iter() {
		pretty_print(&child, indent_size + 2);
	}

	match node.node_type {
		NodeType::Element(ref element) => println!("{0}<{1}/>", indent, element.tag_name),
		_ => {},
	}
}