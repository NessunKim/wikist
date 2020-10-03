use crate::models::Article;
use parse_wiki_text::{Node, Parameter};

pub fn render_template(
    name: &[Node],
    parameters: &[Parameter],
    state: &mut super::State,
) -> String {
    // @TODO: add template namespace automatically
    let full_title = render_template_name(name);
    let article = match Article::find_by_full_title(state.conn, &full_title) {
        Ok(Some(article)) => article,
        Ok(None) => return render_template_not_found(&full_title),
        Err(_) => return "Error".to_owned(),
    };
    match article.get_html(state.conn) {
        Ok(html) => html,
        Err(_) => return "Error".to_owned(),
    }
}

fn render_template_name(name: &[Node]) -> String {
    name.iter()
        .map(|node| {
            if let Node::Text { value, .. } = node {
                value
            } else {
                ""
            }
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn render_template_not_found(title: &str) -> String {
    format!("{{{}}}", title)
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::db::create_connection;
    use diesel::prelude::*;
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_template() {
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let wikitext = "AAA{{aa\nt}}BBB";
            let result = Configuration::default().parse(wikitext);
            assert_eq!(render(&conn, &result), "<p>AAA(template)BBB</p>\n",);
            Ok(())
        })
    }
}
