use parse_wiki_text::Node;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub fn render_internal_link(target: &str, text: &[Node], state: &mut super::State) -> String {
    state.internal_links.push(target.to_owned());
    let text_rendered = super::render_nodes(text, state);
    if let Some(_) = (state.get_article)(target) {
        format!(
            r#"<a href="{}{}">{}</a>"#,
            state.read_base_url,
            utf8_percent_encode(target, NON_ALPHANUMERIC).to_string(),
            text_rendered
        )
    } else {
        format!(
            r#"<a class="new" href="{}{}">{}</a>"#,
            state.edit_base_url,
            utf8_percent_encode(target, NON_ALPHANUMERIC).to_string(),
            text_rendered
        )
    }
}

pub fn render_external_link(nodes: &[Node], state: &mut super::State) -> String {
    if let Node::Text { value, .. } = nodes[0] {
        let splitted = value.splitn(2, " ").collect::<Vec<&str>>();
        let target = splitted[0];
        if let Some(rest) = splitted.get(1) {
            let text = format!("{}{}", rest, super::render_nodes(&nodes[1..], state));
            format!(
                r#"<a target="_blank" rel="nofollow noreferrer noopener" class="external text" href="{}">{}</a>"#,
                target.to_string(),
                text
            )
        } else {
            state.external_link_auto_number += 1;
            format!(
                r#"<a target="_blank" rel="nofollow noreferrer noopener" class="external autonumber" href="{}">[{}]</a>"#,
                target.to_string(),
                state.external_link_auto_number
            )
        }
    } else {
        "<code>External link does not starts with text node</code>".to_owned()
    }
}

pub fn render_category(target: &str, ordinal: &[Node], state: &mut super::State) -> String {
    let ordinal = super::render_nodes(ordinal, state);
    state.categories.push(super::CategoryLink {
        target: target.to_owned(),
        ordinal,
    });
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use parse_wiki_text::Configuration;

    #[test]
    fn test_render_internal_link() {
        use super::super::*;
        let wikitext = "[[aa]]";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(
            render(&result),
            r#"<p><a class="new" href="/edit/aa">aa</a></p>"#
        );

        let wikitext = "[[aa|bb]]";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(
            render(&result),
            r#"<p><a class="new" href="/edit/aa">bb</a></p>"#
        );

        let wikitext = "[[aa|'''bb''']]";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(
            render(&result),
            r#"<p><a class="new" href="/edit/aa"><b>bb</b></a></p>"#
        );
    }

    #[test]
    fn test_render_external_link() {
        use super::super::*;
        let wikitext = "[http://www.google.com]";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(
            render(&result),
            r#"<p><a target="_blank" rel="nofollow noreferrer noopener" class="external autonumber" href="http://www.google.com">[1]</a></p>"#
        );
        let wikitext = "[http://www.google.com a'''aa''']";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(
            render(&result),
            r#"<p><a target="_blank" rel="nofollow noreferrer noopener" class="external text" href="http://www.google.com">a<b>aa</b></a></p>"#
        );
        let wikitext = "[http://www.google.com][http://www.google.com]";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(
            render(&result),
            r#"<p><a target="_blank" rel="nofollow noreferrer noopener" class="external autonumber" href="http://www.google.com">[1]</a><a target="_blank" rel="nofollow noreferrer noopener" class="external autonumber" href="http://www.google.com">[2]</a></p>"#
        );
    }

    #[test]
    fn test_render_category() {
        use super::super::*;
        let wikitext = "[[category:asdf]]";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "");

        let wikitext = "[[category:asdf|asfd]]";
        let result = Configuration::default().parse(wikitext);
        assert!(result.warnings.is_empty());
        assert_eq!(render(&result), "");
    }
}
