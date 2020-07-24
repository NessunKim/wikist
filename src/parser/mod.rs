mod headings;

pub fn render(wikitext: &str) -> String {
    headings::render(wikitext)
}
