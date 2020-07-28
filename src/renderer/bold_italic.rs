pub fn render_bold(state: &mut super::State) -> String {
    println!("bold");
    if state.bold_open {
        state.bold_open = false;
        "</b>".to_owned()
    } else {
        state.bold_open = true;
        "<b>".to_owned()
    }
}

pub fn render_italic(state: &mut super::State) -> String {
    println!("italic");
    if state.italic_open {
        state.italic_open = false;
        "</i>".to_owned()
    } else {
        state.italic_open = true;
        "<i>".to_owned()
    }
}

pub fn render_bold_italic(state: &mut super::State) -> String {
    let mut ret = "".to_owned();
    let mut bold_done = false;
    let mut italic_done = false;
    if state.bold_open {
        state.bold_open = false;
        ret.push_str("</b>");
        bold_done = true
    }
    if state.italic_open {
        state.italic_open = false;
        ret.push_str("</i>");
        italic_done = true
    }
    if !state.italic_open && !bold_done {
        state.italic_open = true;
        ret.push_str("<i>");
    }
    if !state.bold_open && !italic_done {
        state.bold_open = true;
        ret.push_str("<b>");
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
