use parse_wiki_text::{Node, Output};
use std::collections::VecDeque;

mod bold_italic;
mod heading;
mod link;
mod list;
mod paragraph_break;
mod table;

pub struct State {
    bold_italic_queue: VecDeque<(BIStatus, i32)>,
    external_link_auto_number: i32,
}

pub fn render(ast: &Output) -> String {
    let mut state = State {
        bold_italic_queue: VecDeque::new(),
        external_link_auto_number: 0,
    };
    render_nodes(&ast.nodes, &mut state)
}

#[derive(Debug)]
pub enum BIStatus {
    BoldOpen,
    BoldClose,
    ItalicOpen,
    ItalicClose,
}

#[derive(Debug)]
enum BI {
    None,
    Bold,
    Italic,
}

fn render_nodes(nodes: &[Node], state: &mut State) -> String {
    // split bolditalic
    let mut bold_italic_stack: (BI, BI) = (BI::None, BI::None);

    let mut i = 0;
    for node in nodes {
        match node {
            Node::Bold { .. } => {
                match bold_italic_stack {
                    (BI::None, BI::None) => {
                        state.bold_italic_queue.push_back((BIStatus::BoldOpen, i));
                        bold_italic_stack = (BI::Bold, BI::None)
                    }
                    (BI::Italic, BI::None) => {
                        state.bold_italic_queue.push_back((BIStatus::BoldClose, i));
                        bold_italic_stack = (BI::Italic, BI::Bold)
                    }
                    (BI::Italic, BI::Bold) => {
                        state.bold_italic_queue.push_back((BIStatus::BoldClose, i));
                        bold_italic_stack = (BI::Italic, BI::None)
                    }
                    (BI::Bold, BI::None) => {
                        state.bold_italic_queue.push_back((BIStatus::BoldClose, i));
                        bold_italic_stack = (BI::None, BI::None)
                    }
                    (BI::Bold, BI::Italic) => {
                        state
                            .bold_italic_queue
                            .push_back((BIStatus::ItalicClose, i));
                        state.bold_italic_queue.push_back((BIStatus::BoldClose, i));
                        state.bold_italic_queue.push_back((BIStatus::ItalicOpen, i));
                        bold_italic_stack = (BI::Italic, BI::None)
                    }
                    (BI::None, _) | (BI::Bold, BI::Bold) | (BI::Italic, BI::Italic) => panic!(),
                };
                i += 1;
            }
            Node::Italic { .. } => {
                match bold_italic_stack {
                    (BI::None, BI::None) => {
                        state.bold_italic_queue.push_back((BIStatus::ItalicOpen, i));
                        bold_italic_stack = (BI::Italic, BI::None)
                    }
                    (BI::Bold, BI::None) => {
                        state
                            .bold_italic_queue
                            .push_back((BIStatus::ItalicClose, i));
                        bold_italic_stack = (BI::Bold, BI::Italic)
                    }
                    (BI::Bold, BI::Italic) => {
                        state
                            .bold_italic_queue
                            .push_back((BIStatus::ItalicClose, i));
                        bold_italic_stack = (BI::Bold, BI::None)
                    }
                    (BI::Italic, BI::None) => {
                        state
                            .bold_italic_queue
                            .push_back((BIStatus::ItalicClose, i));
                        bold_italic_stack = (BI::None, BI::None)
                    }
                    (BI::Italic, BI::Bold) => {
                        state.bold_italic_queue.push_back((BIStatus::BoldClose, i));
                        state
                            .bold_italic_queue
                            .push_back((BIStatus::ItalicClose, i));
                        state.bold_italic_queue.push_back((BIStatus::BoldOpen, i));
                        bold_italic_stack = (BI::Bold, BI::None)
                    }
                    (BI::None, _) | (BI::Bold, BI::Bold) | (BI::Italic, BI::Italic) => panic!(),
                };
                i += 1;
            }
            Node::BoldItalic { .. } => {
                match bold_italic_stack {
                    (BI::None, BI::None) => {
                        state.bold_italic_queue.push_back((BIStatus::ItalicOpen, i));
                        state.bold_italic_queue.push_back((BIStatus::BoldOpen, i));
                        bold_italic_stack = (BI::Italic, BI::Bold)
                    }
                    (BI::Bold, BI::None) => {
                        state.bold_italic_queue.push_back((BIStatus::BoldClose, i));
                        state.bold_italic_queue.push_back((BIStatus::ItalicOpen, i));
                        bold_italic_stack = (BI::Italic, BI::None)
                    }
                    (BI::Bold, BI::Italic) => {
                        state
                            .bold_italic_queue
                            .push_back((BIStatus::ItalicClose, i));
                        state.bold_italic_queue.push_back((BIStatus::BoldClose, i));
                        bold_italic_stack = (BI::None, BI::None)
                    }
                    (BI::Italic, BI::None) => {
                        state
                            .bold_italic_queue
                            .push_back((BIStatus::ItalicClose, i));
                        state.bold_italic_queue.push_back((BIStatus::BoldOpen, i));
                        bold_italic_stack = (BI::Bold, BI::None)
                    }
                    (BI::Italic, BI::Bold) => {
                        state.bold_italic_queue.push_back((BIStatus::BoldClose, i));
                        state
                            .bold_italic_queue
                            .push_back((BIStatus::ItalicClose, i));
                        bold_italic_stack = (BI::None, BI::None)
                    }
                    (BI::None, _) | (BI::Bold, BI::Bold) | (BI::Italic, BI::Italic) => panic!(),
                };
                i += 1;
            }
            _ => {}
        }
    }
    let (first, last) = bold_italic_stack;
    match last {
        BI::Bold => state.bold_italic_queue.push_back((BIStatus::BoldClose, i)),
        BI::Italic => state
            .bold_italic_queue
            .push_back((BIStatus::ItalicClose, i)),
        _ => {}
    }
    match first {
        BI::Bold => state.bold_italic_queue.push_back((BIStatus::BoldClose, i)),
        BI::Italic => state
            .bold_italic_queue
            .push_back((BIStatus::ItalicClose, i)),
        _ => {}
    }
    nodes
        .iter()
        .map(|node| render_node(node, state))
        .collect::<Vec<String>>()
        .join("")
        + &bold_italic::clear_queue(state)
}

fn render_text(value: &str) -> String {
    value.to_string()
}

fn render_node(node: &Node, state: &mut State) -> String {
    match node {
        Node::OrderedList { items, .. } => list::render_ordered_list(items, state),
        Node::UnorderedList { items, .. } => list::render_unordered_list(items, state),
        Node::DefinitionList { items, .. } => list::render_definition_list(items, state),
        Node::Text { value, .. } => render_text(value),
        Node::ParagraphBreak { .. } => paragraph_break::render_paragraph_break(state),
        Node::ExternalLink { nodes, .. } => link::render_external_link(nodes, state),
        Node::Heading { level, nodes, .. } => heading::render_heading(level, nodes, state),
        Node::Bold { .. } | Node::Italic { .. } | Node::BoldItalic { .. } => {
            bold_italic::render_bold_italic(state)
        }
        Node::Table {
            attributes,
            captions,
            rows,
            ..
        } => table::render_table(attributes, captions, rows, state),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render() {
        use super::*;

        let wikitext = "text";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(render(&result), "text");
        // let wikitext = "==heading==";
        // let result = Configuration::default().parse(wikitext);
        // assert_eq!(render(&result), "<h2>heading</h2>");

        let wikitext = "''italic''";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(render(&result), "<i>italic</i>");
    }
}
