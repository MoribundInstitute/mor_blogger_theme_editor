pub fn beautify_css(raw: &str) -> String {
    raw.replace("{", "{\n    ")
       .replace(";", ";\n    ")
       .replace("}", "\n}\n\n")
       .replace("    \n", "\n") // Clean up trailing spaces on empty lines
       .replace("    }", "}")
}
