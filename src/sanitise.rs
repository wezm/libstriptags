use sanitize_html::sanitize_str;
use sanitize_html::errors::SanitizeError;
use sanitize_html::rules::predefined::DEFAULT;

pub(crate) fn strip_tags(input: &str) -> Result<String, SanitizeError> {
    sanitize_str(&DEFAULT, input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_strips_tags() {
        assert_eq!(strip_tags("<p>This is a <em>simple</em> test.</p><div><div><img src='foo.png' /></div></div>").unwrap().trim(), "This is a simple test.");
    }

    #[test]
    fn it_puts_whitespace_around_inline_tags() {
        assert_eq!(strip_tags("<p>Paragraph</p><span>Next to paragraph.</span>").unwrap().trim(), "Paragraph Next to paragraph.");
    }

    #[test]
    fn it_strips_content_from_script_style_etc_tags() {
    }
}
