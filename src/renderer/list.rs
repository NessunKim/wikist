use parse_wiki_text::{DefinitionListItem, DefinitionListItemType, ListItem};

fn render_list_items(item: &ListItem, state: &mut super::State) -> String {
    format!("<li>{}</li>", super::render_nodes(&item.nodes, state))
}

pub fn render_unordered_list(items: &[ListItem], state: &mut super::State) -> String {
    format!(
        "<ul>{}</ul>\n",
        items
            .iter()
            .map(|node| render_list_items(node, state))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

pub fn render_ordered_list(items: &[ListItem], state: &mut super::State) -> String {
    format!(
        "<ol>{}</ol>\n",
        items
            .iter()
            .map(|node| render_list_items(node, state))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn render_definition_list_items(item: &DefinitionListItem, state: &mut super::State) -> String {
    match item.type_ {
        DefinitionListItemType::Term => {
            format!("<dt>{}</dt>", super::render_nodes(&item.nodes, state))
        }
        DefinitionListItemType::Details => {
            format!("<dd>{}</dd>", super::render_nodes(&item.nodes, state))
        }
    }
}

pub fn render_definition_list(items: &[DefinitionListItem], state: &mut super::State) -> String {
    format!(
        "<dl>{}</dl>\n",
        items
            .iter()
            .map(|node| render_definition_list_items(node, state))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

#[cfg(test)]
mod tests {
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_list() {
        use super::super::*;

        let wikitext = "*a\n*b\n#c\n*d\n";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(
            render(&result),
            concat!(
                "<ul><li>a</li>\n",
                "<li>b</li></ul>\n",
                "<ol><li>c</li></ol>\n",
                "<ul><li>d</li></ul>\n"
            )
        );
    }
    #[test]
    fn test_render_definition_list() {
        use super::super::*;

        let wikitext = ";asdf\n:aa";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(render(&result), "<dl><dt>asdf</dt>\n<dd>aa</dd></dl>\n");
    }
}
