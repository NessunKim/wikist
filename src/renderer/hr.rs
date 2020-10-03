pub fn render_hr(state: &mut super::State) -> String {
    format!(
        "{}<hr>{}",
        super::paragraph::close_paragraph(state),
        super::paragraph::open_paragraph()
    )
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::db::create_connection;
    use diesel::prelude::*;
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_hr() {
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let wikitext = "----";
            let result = Configuration::default().parse(wikitext);
            assert!(result.warnings.is_empty());
            assert_eq!(render(&conn, &result), "<hr>\n");

            let wikitext = "aa\n----\n";
            let result = Configuration::default().parse(wikitext);
            assert!(result.warnings.is_empty());
            assert_eq!(render(&conn, &result), "<p>aa\n</p><hr>\n");
            Ok(())
        })
    }
}
