use lazy_static::lazy_static;
use regex::Regex;

pub fn render(input: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(={1,6})(.+?)={1,6}$").unwrap();
    }
    input
        .split("\n")
        .map(|line| {
            let trimmed = line.trim_end();
            let level = if trimmed.starts_with("======") && trimmed.ends_with("======") {
                6
            } else if trimmed.starts_with("=====") && trimmed.ends_with("=====") {
                5
            } else if trimmed.starts_with("====") && trimmed.ends_with("====") {
                4
            } else if trimmed.starts_with("===") && trimmed.ends_with("===") {
                3
            } else if trimmed.starts_with("==") && trimmed.ends_with("==") {
                2
            } else if trimmed.starts_with("=") && trimmed.ends_with("=") {
                1
            } else {
                return trimmed.to_string();
            };
            format!(
                "<h{}>{}</h{}>",
                level,
                trimmed[level..(trimmed.len() - level)].trim().to_string(),
                level
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_render_headings() {
        assert_eq!(render("nothing"), "nothing");
        assert_eq!(render("=a="), "<h1>a</h1>");
        assert_eq!(render("==a=="), "<h2>a</h2>");
        assert_eq!(render("===a==="), "<h3>a</h3>");
        assert_eq!(render("====a===="), "<h4>a</h4>");
        assert_eq!(render("=====a====="), "<h5>a</h5>");
        assert_eq!(render("======a======"), "<h6>a</h6>");
        assert_eq!(render("===a===\n====b===="), "<h3>a</h3>\n<h4>b</h4>");
        assert_eq!(render("== a =="), "<h2>a</h2>");
        assert_eq!(render("== a == "), "<h2>a</h2>");
        assert_eq!(render(" ==a=="), " ==a==");
        assert_eq!(render("==a==="), "<h2>a=</h2>");
        assert_eq!(render("x\n===ab===\na"), "x\n<h3>ab</h3>\na");
        assert_eq!(render("==x<b>a</b>x=="), "<h2>x<b>a</b>x</h2>");
    }
}
