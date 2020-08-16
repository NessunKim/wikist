use parse_wiki_text::{Configuration, Output};

pub fn parse(wikitext: &str) -> Output {
    Configuration::default().parse(wikitext)
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
