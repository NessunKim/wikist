use parse_wiki_text::Node;

pub fn render_heading(level: &u8, nodes: &Vec<Node>, state: &mut super::State) -> String {
    format!(
        "<h{}>{}</h{}>\n",
        level,
        super::render_nodes(&nodes, state),
        level
    )
}

#[cfg(test)]
mod tests {
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_heading() {
        use super::super::*;

        // let wikitext = "===asdf===";
        // let result = Configuration::default().parse(wikitext);
        // assert!(result.warnings.is_empty());
        // assert_eq!(
        //     render(&result),
        //     "<h3><span class=\"headline\" id=\"asdf\">asdf</span></h3>\n"
        // );
        let wikitext = "===asdf==";
        let result = Configuration::default().parse(wikitext);
        println!("{:#?}", result.warnings);
        assert_eq!(render(&result), "<h2>=asdf</h2>\n");
        let wikitext = "==asdf===";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "<h2>asdf=</h2>\n");
    }
}
