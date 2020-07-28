use parse_wiki_text::{Node, Output};

mod bold_italic;
mod list;
mod paragraph_break;

pub struct State {
    bold_open: bool,
    italic_open: bool,
}

fn render(ast: &Output) -> String {
    let mut state = State {
        bold_open: false,
        italic_open: false,
    };
    render_nodes(&ast.nodes, &mut state)
}

fn render_nodes(nodes: &Vec<Node>, state: &mut State) -> String {
    // split bolditalic
    let mut bold_italic: &Node;
    let mut bold_open_first: bool = false;
    let mut italic_open_first: bool = false;
    let mut bold_close_first: bool = false;
    let mut italic_close_first: bool = false;
    for node in nodes {
        match node {
            Node::Bold { .. } => {
                if !italic_open_first {
                    bold_open_first = true
                }
            }
            Node::Italic { .. } => {}
            Node::BoldItalic { .. } => {
                bold_italic = node;
            }
            _ => {}
        }
    }
    nodes
        .iter()
        .map(|node| render_node(node, state))
        .collect::<Vec<String>>()
        .join("")
}

fn render_text(value: &str) -> String {
    value.to_string()
}

fn render_node(node: &Node, state: &mut State) -> String {
    match node {
        Node::OrderedList { items, .. } => list::render_ordered_list(items, state),
        Node::UnorderedList { items, .. } => list::render_unordered_list(items, state),
        Node::Text { value, .. } => render_text(value),
        Node::ParagraphBreak { .. } => paragraph_break::render_paragraph_break(state),
        Node::Bold { .. } => bold_italic::render_bold(state),
        Node::Italic { .. } => bold_italic::render_italic(state),
        Node::BoldItalic { .. } => bold_italic::render_bold_italic(state),
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
