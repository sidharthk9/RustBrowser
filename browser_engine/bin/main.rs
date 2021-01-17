extern crate browser_engine;
use browser_engine::{command, css, css_parser, dom, html_parser, layout, render, style};

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn get_html() -> Vec<dom::Node> {
    let mut path = env::current_dir().unwrap();
    path.push("example/example.html");

    let mut file_reader = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(error) => panic!(" {0}: Error {1}", path.display(), error),
    };

    let mut html_input = String::new();
    file_reader.read_to_string(&mut html_input).unwrap();
    let nodes = html_parser::HtmlParser::new(&html_input).parse_nodes();

    nodes
}

fn get_css() -> css::StyleSheet {
    let mut path = env::current_dir().unwrap();
    path.push("example/example.css");

    let mut file_reader = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(error) => panic!(" {0}: Error {1}", path.display(), error),
    };

    let mut css_input = String::new();
    file_reader.read_to_string(&mut css_input).unwrap();
    let stylesheet = css_parser::CssParser::new(&css_input).parse_stylesheet();

    stylesheet
}

fn main() {
    let nodes = get_html();
    for node in nodes.iter() {
        dom::pretty_print(node, 0);
    }

    let ref root_node = nodes[0];

    let stylesheet = get_css();
    println!("{:?}", stylesheet);

    let style_tree_root = style::StyledNode::new(&root_node, &stylesheet);
    style::pretty_print(&style_tree_root, 0);

    let mut viewport = layout::Dimensions::default();
    viewport.content.width = 1024.0;
    viewport.content.height = 768.0;

    let layout_tree = layout::layout_tree(&style_tree_root, viewport);
    layout::pretty_print(&layout_tree, 0);

    let display_commands = command::build_display_commands(&layout_tree);

    render::render_loop(&display_commands);
}
