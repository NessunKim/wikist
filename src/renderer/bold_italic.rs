pub fn render_bold_italic(state: &mut super::State) -> String {
    use super::BIStatus;
    let mut last_n = -1;
    let mut res = "".to_owned();
    while let Some((st, n)) = state.bold_italic_queue.front() {
        if last_n == -1 {
            last_n = *n;
        } else if last_n != *n {
            break;
        }
        match st {
            BIStatus::BoldOpen => res += "<b>",
            BIStatus::BoldClose => res += "</b>",
            BIStatus::ItalicOpen => res += "<i>",
            BIStatus::ItalicClose => res += "</i>",
        }
        state.bold_italic_queue.pop_front();
    }
    res
}

#[cfg(test)]
mod tests {
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_bold_italic() {
        use super::super::*;
        let wikitext = "'''''aaa'''''";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "<i><b>aaa</b></i>");
        let wikitext = "'''''asdf''bb'''";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "<b><i>asdf</i>bb</b>");
        let wikitext = "'''''asdf'''bb''";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "<i><b>asdf</b>bb</i>");
    }
}
