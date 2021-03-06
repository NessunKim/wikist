use parse_wiki_text::{Node, TableCaption, TableCell, TableCellType, TableRow};

pub fn render_table(
    attributes: &[Node],
    captions: &[TableCaption],
    rows: &[TableRow],
    state: &mut super::State,
) -> String {
    let open_tag = if attributes.len() > 0 {
        format!("<table {}>", super::render_nodes(attributes, state))
    } else {
        "<table>".to_owned()
    };
    let caption_text = captions
        .iter()
        .map(|caption| render_caption(&caption, state))
        .collect::<Vec<String>>()
        .join("");
    format!(
        "{}{}{}\n{}</table>{}",
        super::paragraph::close_paragraph(state),
        open_tag,
        caption_text,
        render_rows(rows, state),
        super::paragraph::open_paragraph()
    )
}

fn render_caption(caption: &TableCaption, state: &mut super::State) -> String {
    let open_tag = if let Some(attributes) = &caption.attributes {
        if attributes.len() > 0 {
            format!("<caption {}>", super::render_nodes(&attributes, state))
        } else {
            "<caption>".to_owned()
        }
    } else {
        "<caption>".to_owned()
    };
    format!(
        "\n{}{}\n</caption>",
        open_tag,
        super::render_nodes(&caption.content, state)
    )
}

fn render_rows(rows: &[TableRow], state: &mut super::State) -> String {
    format!(
        "<tbody>{}</tbody>",
        rows.iter()
            .map(|row| render_row(&row, state))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn render_row(row: &TableRow, state: &mut super::State) -> String {
    let open_tag = if row.attributes.len() > 0 {
        format!("<tr {}>", super::render_nodes(&row.attributes, state))
    } else {
        "<tr>".to_owned()
    };
    format!(
        "{}\n{}</tr>",
        open_tag,
        row.cells
            .iter()
            .map(|cell| render_cell(&cell, state))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn render_cell(cell: &TableCell, state: &mut super::State) -> String {
    let tag_name = match cell.type_ {
        TableCellType::Heading => "th",
        TableCellType::Ordinary => "td",
    };
    let break_pos = cell
        .content
        .iter()
        .position(|x| matches!(x, Node::ParagraphBreak { .. }));
    let open_tag = if let Some(attributes) = &cell.attributes {
        if attributes.len() > 0 {
            format!("<{} {}>", tag_name, super::render_nodes(&attributes, state))
        } else {
            format!("<{}>", tag_name)
        }
    } else {
        format!("<{}>", tag_name)
    };
    if let Some(break_pos) = break_pos {
        let before_break = super::render_nodes(&cell.content[..break_pos], state);
        let after_break = super::render_nodes(&cell.content[break_pos + 1..], state);
        format!(
            "{}{}{}{}{}\n</{}>",
            open_tag,
            before_break,
            super::paragraph::open_paragraph(),
            after_break,
            super::paragraph::close_paragraph(state),
            tag_name
        )
    } else {
        let content = super::render_nodes(&cell.content, state);
        format!("{}{}\n</{}>", open_tag, content, tag_name)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::db::create_connection;
    use diesel::prelude::*;
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_table() {
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
        let wikitext = "{|\n|A\n|B\n|}";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(
            render(&conn, &result),
            "<table>\n<tbody><tr>\n<td>A\n</td>\n<td>B\n</td></tr></tbody></table>\n",
        );

        let wikitext = "{|\n!A\n!B\n|-\n|C\n|D\n|}";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(
            render(&conn, &result),
            "<table>\n<tbody><tr>\n<th>A\n</th>\n<th>B\n</th></tr>\n<tr>\n<td>C\n</td>\n<td>D\n</td></tr></tbody></table>\n",
        );

        let wikitext = "{|\n| A\nasdf\n|}";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(
            render(&conn, &result),
            "<table>\n<tbody><tr>\n<td>A\n<p>asdf\n</p>\n</td></tr></tbody></table>\n",
        );

        let wikitext = "{| class=\"t\"\n|- class=\"r\"\n| class=\"c\" | content\n|}";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(
            render(&conn, &result),
            "<table class=\"t\">\n<tbody><tr class=\"r\">\n<td class=\"c\">content\n</td></tr></tbody></table>\n",
        );

        // xss test
        let wikitext = "{| onclick=\"alert('xss')\"\n| a\n|}";
        let result = Configuration::default().parse(wikitext);
        assert_ne!(
            render(&conn, &result),
            "<table onclick=\"alert('xss')\">\n<tbody><tr>\n<td>a\n</td></tr></tbody></table>\n",
        );

        let wikitext = "{|
|+ c1
|+ c2
|-
| content
|}";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(
            render(&conn, &result),
            "<table>
<caption>c1
</caption>
<caption>c2
</caption>
<tbody><tr>
<td>content
</td></tr></tbody></table>
",
        );
        Ok(())
    })
    }
}
