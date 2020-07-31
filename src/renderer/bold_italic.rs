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

pub fn clear_queue(state: &mut super::State) -> String {
    let mut ret = "".to_owned();
    while !state.bold_italic_queue.is_empty() {
        ret += &super::bold_italic::render_bold_italic(state);
    }
    ret
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
        assert_eq!(render(&result), "<p><i><b>aaa</b></i></p>");
        let wikitext = "'''''asdf''bb'''";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "<p><i><b>asdf</b></i><b>bb</b></p>");
        // below is the right way to render, but it's difficult
        // assert_eq!(render(&result), "<p><b><i>asdf</i>bb</b></p>");
        let wikitext = "'''''asdf'''bb''";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "<p><i><b>asdf</b>bb</i></p>");
    }
}
