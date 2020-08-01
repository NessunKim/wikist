use parse_wiki_text::Configuration;
fn parse(wikitext: &str) {
    let result = Configuration::default().parse(wikitext);
    assert!(result.warnings.is_empty());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        use super::*;

        let wikitext = concat!(
            "==Our values==\n",
            "*Correctness\n",
            "*Speed\n",
            "*Ergonomics"
        );
        parse(wikitext);
    }
}
