use parse_wiki_text::Node;

pub fn render_preformatted(nodes: &[Node], state: &mut super::State) -> String {
    format!(
        "{}<pre>{}\n</pre>{}",
        super::paragraph::close_paragraph(state),
        super::render_nodes(&nodes, state),
        super::paragraph::open_paragraph()
    )
}

#[cfg(test)]
mod tests {
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_preformatted() {
        use super::super::*;
        let wikitext = " Start each line with a space.
 Text is '''preformatted''' and
 ''markups'' '''''can''''' be done.";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(
            render(&result),
            "<pre>Start each line with a space.
Text is <b>preformatted</b> and
<i>markups</i> <i><b>can</b></i> be done.
</pre>\n"
        );
    }
}
