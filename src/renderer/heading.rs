use parse_wiki_text::Node;

pub fn render_heading(level: &u8, nodes: &[Node], state: &mut super::State) -> String {
    format!(
        "{}<h{}>{}</h{}>{}",
        super::paragraph::close_paragraph(state),
        level,
        super::render_nodes(&nodes, state),
        level,
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
    fn test_render_heading() {
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
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
            assert_eq!(render(&conn, &result), "<h2>=asdf</h2>\n");
            let wikitext = "==asdf===";
            let result = Configuration::default().parse(wikitext);
            assert!(result.warnings.is_empty());
            assert_eq!(render(&conn, &result), "<h2>asdf=</h2>\n");
            Ok(())
        })
    }
}
