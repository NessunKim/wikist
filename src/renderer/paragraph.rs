pub fn render_paragraph_break(state: &mut super::State) -> String {
    format!("{}{}", &super::bold_italic::clear_queue(state), "</p><p>")
}

pub fn close_paragraph(state: &mut super::State) -> String {
    format!("{}\n{}", &super::bold_italic::clear_queue(state), "</p>")
}

pub fn open_paragraph() -> String {
    "\n<p>".to_owned()
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::db::create_connection;
    use diesel::prelude::*;
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_paragraph_break() {
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let wikitext = "asdf\n\naaa";
            let result = Configuration::default().parse(wikitext);
            assert!(result.warnings.is_empty());
            assert_eq!(render(&conn, &result), "<p>asdf</p><p>aaa</p>");

            let wikitext = "a'''b\n\nc";
            let result = Configuration::default().parse(wikitext);
            assert!(result.warnings.is_empty());
            assert_eq!(render(&conn, &result), "<p>a<b>b</b></p><p>c</p>");
            Ok(())
        })
    }
}
