use parse_wiki_text::{Node, TableCaption, TableCell, TableCellType, TableRow};

pub fn render_table(
    attributes: &Vec<Node>,
    captions: &Vec<TableCaption>,
    rows: &Vec<TableRow>,
    state: &mut super::State,
) -> String {
    format!("<table>\n{}</table>\n", render_rows(rows, state))
}

fn render_rows(rows: &Vec<TableRow>, state: &mut super::State) -> String {
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
    match cell.type_ {
        TableCellType::Heading => {
            format!("<th>{}\n</th>", super::render_nodes(&cell.content, state))
        }
        TableCellType::Ordinary => {
            format!("<td>{}\n</td>", super::render_nodes(&cell.content, state))
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
    }
}
