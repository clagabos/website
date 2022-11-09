pub fn minify_string(str: &str) -> String {
    // minify string by removing new lines and tabs
    let mut minified = String::new();
    for line in str.lines() {
        minified.push_str(line.trim());
    }
    minified
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minify_string() {
        let str = "Hello, world!";
        let minified = minify_string(str);
        assert_eq!(minified, "Hello, world!");
    }

    #[test]
    fn test_minify_string_with_newlines() {
        let str = "Hello, world!
    Hello, world!";
        let minified = minify_string(str);
        assert_eq!(minified, "Hello, world!Hello, world!");
    }

    #[test]
    fn test_minify_string_with_tabs() {
        let str = "Hello, world!    ";
        let minified = minify_string(str);
        assert_eq!(minified, "Hello, world!");
    }
}