use lazy_static::lazy_static;
use sanitize_html::{errors::SanitizeError, rules::Rules, sanitize_str};

lazy_static! {
    pub static ref RULES: Rules = Rules::new()
        .space("address")
        .space("article")
        .space("aside")
        .space("blockquote")
        .space("br")
        .space("dd")
        .space("div")
        .space("dl")
        .space("dt")
        .space("footer")
        .space("h1")
        .space("h2")
        .space("h3")
        .space("h4")
        .space("h5")
        .space("h6")
        .space("header")
        .space("hgroup")
        .space("hr")
        .space("li")
        .space("nav")
        .space("ol")
        .space("p")
        .space("pre")
        .space("section")
        .space("ul")
        .delete("iframe")
        .delete("noembed")
        .delete("noframes")
        .delete("noscript")
        .delete("script")
        .delete("style");
}

pub(crate) fn strip_tags(input: &str) -> Result<String, SanitizeError> {
    sanitize_str(&RULES, input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_strips_tags() {
        assert_eq!(
            strip_tags(
                "<p>This is a <em>simple</em> test.</p><div><div><img src='foo.png' /></div></div>"
            )
            .unwrap()
            .trim(),
            "This is a simple test."
        );
    }

    #[test]
    fn it_puts_whitespace_around_inline_tags() {
        assert_eq!(
            strip_tags("<p>Paragraph</p><span>Next to paragraph.</span>")
                .unwrap()
                .trim(),
            "Paragraph Next to paragraph."
        );
    }

    #[test]
    fn it_strips_content_from_script_style_etc_tags() {
        assert_eq!(strip_tags("Do<script>var x = 1;</script> not<iframe>hello</iframe> want. <style>body { background-color: red }").unwrap().trim(), "Do not want.");
    }
}
