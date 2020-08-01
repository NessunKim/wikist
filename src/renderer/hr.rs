use parse_wiki_text::Node;

pub fn render_hr(state: &mut super::State) -> String {
    format!(
        "{}<hr>{}",
        super::paragraph::close_paragraph(state),
        super::paragraph::open_paragraph()
    )
}

#[cfg(test)]
mod tests {
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_hr() {
        use super::super::*;
        let wikitext = "----";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "<hr>\n");

        let wikitext = "aa\n----\n";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "<p>aa\n</p><hr>\n");
    }
}
