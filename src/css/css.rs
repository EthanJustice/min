// std

// crates

// local

/// Incredibly simplistic CSS minification.
pub fn minify_from_str(css: &str) -> String {
    let min_parts: Vec<&str> = css.split("\n").collect();

    // to regex or not to regex
    String::from(min_parts.join(""))
        .replace("\n", "")
        .replace("  ", "")
        .replace(": ", ":")
        .replace(";}", "}")
        .replace(" {", "{")
        .replace(" }", "}")
        .replace("{ ", "{")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_rules() {
        let css = "
        body {
            background: red;
            color: green;
        }
        h1 {
            font-weight: lighter;
        }
        ";
        assert_eq!(
            minify_from_str(css),
            "body{background:red;color:green}h1{font-weight:lighter}"
        );
    }

    #[test]
    fn media_queries() {
        let css = "
        @keyframes fadeIn {
            0% { opacity: 0 }
            100% { opacity: 1 }
        }
        ";
        assert_eq!(
            minify_from_str(css),
            "@keyframes fadeIn{0%{opacity:0}100%{opacity:1}}"
        );
    }

    #[test]
    fn custom_imports() {}

    #[test]
    fn css_functions() {}
}
