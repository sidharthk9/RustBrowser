use std::fmt;

use crate::css::{Color, Value};
use crate::layout::{LayoutBox, Rectangle};

pub enum DisplayCommand {
    SolidRectangle(Color, Rectangle),
}

pub type DisplayList = Vec<DisplayCommand>;

pub fn build_display_commands(root: &LayoutBox) -> DisplayList {
    let mut commands = Vec::new();

    render_layout_box(&mut commands, root);

    commands
}

fn render_layout_box(commands: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(commands, layout_box);
    render_borders(commands, layout_box);

    for child in &layout_box.children {
        render_layout_box(commands, child);
    }
}

fn render_background(commands: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background-color").map(|color| {
        commands.push(DisplayCommand::SolidRectangle(
            color,
            layout_box.dimensions.border_box(),
        ))
    });
}

fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.styled_node.value(name) {
        Some(value) => match **value {
            Value::Color(ref color) => return Some(**color.clone()),
            _ => return None,
        },
        None => return None,
    }
}

fn render_borders(commands: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return,
    };

    let dimension = &layout_box.dimensions;
    let border_box = dimension.border_box();

    commands.push(DisplayCommand::SolidRectangle(
        color.clone(),
        Rectangle {
            x: border_box.x,
            y: border_box.y,
            width: dimension.border.left,
            height: border_box.height,
        },
    ));

    commands.push(DisplayCommand::SolidRectangle(
        color.clone(),
        Rectangle {
            x: border_box.x + border_box.width - dimension.border.right,
            y: border_box.y,
            width: dimension.border.left,
            height: border_box.height,
        },
    ));

    commands.push(DisplayCommand::SolidRectangle(
        color.clone(),
        Rectangle {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: dimension.border.top,
        },
    ));

    commands.push(DisplayCommand::SolidRectangle(
        color,
        Rectangle {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: dimension.border.bottom,
        },
    ));
}

impl fmt::Debug for DisplayCommand {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayCommand::SolidRectangle(ref color, ref rectangle) => {
                write!(format, "{:?} {:?}", color, rectangle)
            }
        }
    }
}
