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
    use super::super::*;
    use crate::db::create_connection;
    use diesel::prelude::*;
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_preformatted() {
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let wikitext = " Start each line with a space.
 Text is '''preformatted''' and
 ''markups'' '''''can''''' be done.";
            let result = Configuration::default().parse(wikitext);
            assert!(result.warnings.is_empty());
            assert_eq!(
                render(&conn, &result),
                "<pre>Start each line with a space.
Text is <b>preformatted</b> and
<i>markups</i> <i><b>can</b></i> be done.
</pre>\n"
            );
            Ok(())
        })
    }
}
