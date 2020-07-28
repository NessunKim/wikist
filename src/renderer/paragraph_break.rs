pub fn render_paragraph_break(state: &mut super::State) -> String {
    let mut ret = "".to_owned();
    if state.bold_open {
        state.bold_open = false;
        ret.push_str("</b>");
    }
    ret
}

#[cfg(test)]
mod tests {
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_paragraph_break() {
        use super::super::*;

        let wikitext = "a'''b\n\nc";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "a<b>b</b>c");
    }
}
