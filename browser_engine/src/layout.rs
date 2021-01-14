use std::fmt;

use crate::css::{Unit, Value};
use crate::style::{Display, StyledNode};

#[derive(Clone)]
pub struct LayoutBox<'a> {
    pub(crate) dimensions: Dimensions,
    box_type: BoxType,
    pub(crate) styled_node: &'a StyledNode<'a>,
    pub(crate) children: Vec<LayoutBox<'a>>,
}

#[derive(Clone, Copy, Default)]
pub struct Dimensions {
    pub(crate) content: Rectangle,
    padding: EdgeSizes,
    pub(crate) border: EdgeSizes,
    margin: EdgeSizes,
    current: Rectangle,
}

#[derive(Clone, Copy, Default)]
pub struct Rectangle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Copy, Clone, Default)]
pub struct EdgeSizes {
    pub(crate) left: f32,
    pub(crate) right: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
}

pub enum BoxType {
    Block,
    Inline,
    InlineBlock,
    Anonymous,
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType, styled_node: &'a StyledNode) -> LayoutBox<'a> {
        LayoutBox {
            box_type,
            styled_node,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    fn layout(&mut self, boundary_box: Dimensions) {
        match self.box_type {
            BoxType::Block => self.layout_block(boundary_box),
            BoxType::Inline => self.layout_block(boundary_box),
            BoxType::InlineBlock => self.layout_inline_block(boundary_box),
            BoxType::Anonymous => {}
        }
    }

    fn layout_inline_block(&mut self, boundary_box: Dimensions) {
        self.calculate_inline_width(boundary_box);
        self.calculate_inline_position(boundary_box);
        self.layout_children();
        self.calculate_height();
    }

    fn calculate_inline_width(&mut self, boundary_box: Dimensions) {
        let node = self.styled_node;
        let dimension = &mut self.dimensions;

        dimension.content.height = get_absolute_nums(node, boundary_box, "width").unwrap_or(0.0);

        dimension.margin.left = node.num_or("margin-left", 0.0);
        dimension.margin.right = node.num_or("margin-right", 0.0);

        dimension.padding.left = node.num_or("padding-left", 0.0);
        dimension.padding.right = node.num_or("padding-right", 0.0);

        dimension.border.left = node.num_or("border-left-width", 0.0);
        dimension.border.right = node.num_or("border-right-width", 0.0);
    }

    fn calculate_inline_position(&mut self, boundary_box: Dimensions) {
        let node = self.styled_node;
        let dimension = &mut self.dimensions;

        dimension.margin.top = node.num_or("margin-top", 0.0);
        dimension.margin.bottom = node.num_or("margin-bottom", 0.0);

        dimension.border.top = node.num_or("border-top-width", 0.0);
        dimension.border.bottom = node.num_or("border-bottom-width", 0.0);

        dimension.padding.top = node.num_or("padding-top", 0.0);
        dimension.padding.bottom = node.num_or("padding-bottom", 0.0);

        dimension.content.x = boundary_box.content.x
            + boundary_box.current.x
            + dimension.margin.left
            + dimension.border.left
            + dimension.padding.left;

        dimension.content.y = boundary_box.content.height
            + boundary_box.content.y
            + dimension.margin.top
            + dimension.border.top
            + dimension.padding.top;
    }

    fn layout_block(&mut self, boundary_box: Dimensions) {
        self.calculate_width(boundary_box);
        self.calculate_position(boundary_box);
        self.layout_children();
        self.calculate_height();
    }

    fn calculate_width(&mut self, boundary_box: Dimensions) {
        let node = self.styled_node;
        let dimension = &mut self.dimensions;

        let width = get_absolute_num(node, boundary_box, "width").unwrap_or(0.0);
        let margin_left = style.value("margin-left");
        let margin_right = style.value("margin-right");

        let margin_left_num = match margin_left {
            Some(margin) => match **margin {
                Value::Other(ref string) => string.parse().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        };

        let margin_right_num = match margin_right {
            Some(margin) => match **margin {
                Value::Other(ref string) => string.parse().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        };

        dimension.border.left = style.num_or("border-left-width", 0.0);
        dimension.border.right = style.num_or("border-right-width", 0.0);

        dimension.padding.left = style.num_or("padding-left", 0.0);
        dimension.padding.right = style.num_or("padding-right", 0.0);

        let total_size = width
            + margin_left_num
            + margin_right_num
            + dimension.border.left
            + dimension.border.right
            + dimension.padding.left
            + dimension.padding.right;

        let underflow = boundary_box.content.width - total_size;

        match (width, margin_left, margin_right_num) {
            (0.0, _, _) => {
                if underflow >= 0.0 {
                    dimension.content.width = underflow;
                    dimension.margin.right = margin_right_num;
                } else {
                    dimension.margin.right = margin_right_num + underflow;
                    dimension.content.width = width;
                }
                dimension.margin.left = margin_left_num;
            }
            (value, None, Some(_)) if value != 0.0 => {
                dimension.margin.left = underflow;
                dimension.margin.right = margin_right_num;
                dimension.content.width = value;
            }
            (value, Some(_), None) if value != 0.0 => {
                dimension.margin.right = underflow;
                dimension.margin.left = margin_left_num;
                dimension.content.width = value;
            }
            (value, None, None) if value != 0.0 => {
                dimension.margin.left = underflow / 2.0;
                dimension.margin.right = underflow / 2.0;
                dimension.content.width = value;
            }
            (_, _, _) => {
                dimension.margin.right = margin_right_num + underflow;
                dimension.margin.left = margin_left_num;
                dimension.content.width = width
            }
        }
    }

    fn calculate_position(&mut self, boundary_box: Dimensions) {
        let node = self.styled_node;
        let dimension = &mut self.dimensions;

        dimension.margin.top = node.num_or("margin-top", 0.0);
        dimension.margin.bottom = node.num_or("margin-bottom", 0.0);

        dimension.border.top = node.num_or("border-top-width", 0.0);
        dimension.border.bottom = node.num_or("border-bottom-width", 0.0);

        dimension.padding.top = node.num_or("padding-top", 0.0);
        dimension.padding.bottom = node.num_or("padding-bottom", 0.0);

        dimension.content.x = boundary_box.content.x
            + dimension.margin.left
            + dimension.border.left
            + dimension.padding.left;
        dimension.content.y = boundary_box.content.height
            + boundary_box.content.y
            + dimension.margin.top
            + dimension.border.top
            + dimension.padding.top;
    }

    fn calculate_height(&mut self) {
        self.styled_node
            .value("height")
            .map_or((), |height| match **height {
                Value::Length(num, _) => self.dimensions.content.height = num,
                _ => {}
            })
    }

    fn layout_children(&mut self) {
        let dimension = &mut self.dimensions;
        let mut max_child_height = 0.0;

        let mut previous_box_type = BoxType::Block;

        for child in &mut self.children {
            match previous_box_type {
                BoxType::InlineBlock => match child.box_type {
                    BoxType::Block => {
                        dimension.content.height += max_child_height;
                        dimension.current.x = 0.0
                    }
                    _ => {}
                },
                _ => {}
            }

            child.layout(*dimension);
            let new_height = child.dimensions.margin_box().height;

            if new_height > max_child_height {
                max_child_height = new_height;
            }

            match child.box_type {
                BoxType::Block => dimension.content.height += child.dimensions.margin_box().height,
                BoxType::InlineBlock => {
                    dimension.current.x += child.dimensions.margin_box().width;

                    if dimension.current.x > dimension.content.width {
                        dimension.content.height += max_child_height;
                        dimension.current.x = 0.0;
                        child.layout(*dimension);
                        dimension.current.x += child.dimensions.margin_box().width;
                    }
                }
                _ => {}
            }

            previous_box_type = child.box_type.clone();
        }
    }
}

impl<'a> fmt::Debug for LayoutBox<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(
            format,
            "type:\n  {:?}\n{:?}\n",
            self.box_type, self.dimensions
        )
    }
}

impl Dimensions {
    fn padding_box(&self) -> Rectangle {
        self.content.expanded(self.padding)
    }

    pub fn border_box(&self) -> Rectangle {
        self.padding_box().expanded(self.border)
    }

    fn margin_box(&self) -> Rectangle {
        self.border_box().expanded(self.margin)
    }
}

impl fmt::Debug for Dimensions {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(
            format,
            "content:\n  {:?}\npadding:\n  {:?}\nborder:\n  {:?}\n margin:\n  {:?}",
            self.content, self.padding, self.border, self.margin
        )
    }
}

impl Rectangle {
    fn expanded(&self, edge: EdgeSizes) -> Rectangle {
        Rectangle {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(
            format,
            "x: {}, y: {}, w: {}, h: {}",
            self.x, self.y, self.width, self.height
        )
    }
}

impl fmt::Debug for EdgeSizes {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(
            format,
            "l: {} r: {} top: {} bot: {}",
            self.left, self.right, self.top, self.bottom
        )
    }
}

impl fmt::Debug for BoxType {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        let display_type = match *self {
            BoxType::Block => "block",
            BoxType::Inline => "inline",
            BoxType::InlineBlock => "inline-block",
            BoxType::Anonymous => "anonymous",
        };

        write!(f, "{}", display_type)
    }
}

fn get_absolute_num(node: &StyledNode, boundary_box: Dimensions, prop: &str) -> Option<f32> {
    match node.value(prop) {
        Some(ref val) => match ***val {
            Value::Length(len, ref unit) => match *unit {
                Unit::Px => Some(len),
                Unit::Pct => Some(len * boundary_box.content.width / 100.0),
                _ => panic!("Unimplemented css length unit"),
            },
            _ => None,
        },
        None => None,
    }
}

pub fn layout_tree<'a>(
    root: &'a StyledNode<'a>,
    mut containing_block: Dimensions,
) -> LayoutBox<'a> {
    containing_block.content.height = 0.0;

    let mut root_box = build_layout_tree(root);
    root_box.layout(containing_block);

    root_box
}

fn build_layout_tree<'a>(node: &'a StyledNode) -> LayoutBox<'a> {
    let mut layout_node = LayoutBox::new(
        match node.get_display() {
            Display::Block => BoxType::Block,
            Display::Inline => BoxType::Inline,
            Display::InlineBlock => BoxType::InlineBlock,
            Display::None => BoxType::Anonymous,
        },
        node,
    );

    for child in &node.children {
        match child.get_display() {
            Display::Block => layout_node.children.push(build_layout_tree(child)),
            Display::Inline => layout_node.children.push(build_layout_tree(child)),
            Display::InlineBlock => layout_node.children.push(build_layout_tree(child)),
            Display::None => {}
        }
    }
    layout_node
}

pub fn pretty_print(node: &LayoutBox, tree_level: usize) {
    println!("{}{:?}\n", tree_level, node);

    for child in node.children.iter() {
        pretty_print(&child, tree_level + 1);
    }
}
