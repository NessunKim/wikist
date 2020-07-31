use parse_wiki_text::{Node, TableCaption, TableCell, TableCellType, TableRow};

pub fn render_table(
    attributes: &[Node],
    captions: &[TableCaption],
    rows: &[TableRow],
    state: &mut super::State,
) -> String {
    format!(
        "{}<table>\n{}</table>{}",
        super::paragraph::close_paragraph(state),
        render_rows(rows, state),
        super::paragraph::open_paragraph()
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
    format!(
        "<tr>\n{}</tr>",
        row.cells
            .iter()
            .map(|cell| render_cell(&cell, state))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn render_cell(cell: &TableCell, state: &mut super::State) -> String {
    let break_pos = cell.content.iter().position(|x| {
        if let Node::ParagraphBreak { .. } = x {
            true
        } else {
            false
        }
    });
    if let Some(break_pos) = break_pos {
        let before_break = super::render_nodes(&cell.content[..break_pos], state);
        let after_break = super::render_nodes(&cell.content[break_pos + 1..], state);
        match cell.type_ {
            TableCellType::Heading => format!(
                "<th>{}{}{}{}\n</th>",
                before_break,
                super::paragraph::open_paragraph(),
                after_break,
                super::paragraph::close_paragraph(state)
            ),
            TableCellType::Ordinary => format!(
                "<td>{}{}{}{}\n</td>",
                before_break,
                super::paragraph::open_paragraph(),
                after_break,
                super::paragraph::close_paragraph(state)
            ),
        }
    } else {
        let content = super::render_nodes(&cell.content, state);
        match cell.type_ {
            TableCellType::Heading => format!("<th>{}\n</th>", content),
            TableCellType::Ordinary => format!("<td>{}\n</td>", content),
        }
    }
}

#[cfg(test)]
mod tests {
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_table() {
        use super::super::*;

        let wikitext = "{|\n|A\n|B\n|}";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(
            render(&result),
            "<table>\n<tbody><tr>\n<td>A\n</td>\n<td>B\n</td></tr></tbody></table>\n",
        );

        let wikitext = "{|\n!A\n!B\n|-\n|C\n|D\n|}";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(
            render(&result),
            "<table>\n<tbody><tr>\n<th>A\n</th>\n<th>B\n</th></tr>\n<tr>\n<td>C\n</td>\n<td>D\n</td></tr></tbody></table>\n",
        );

        let wikitext = "{|\n| A\nasdf\n|}";
        let result = Configuration::default().parse(wikitext);
        assert_eq!(
            render(&result),
            "<table>\n<tbody><tr>\n<td>A\n<p>asdf\n</p>\n</td></tr></tbody></table>\n",
        );
    }
}
