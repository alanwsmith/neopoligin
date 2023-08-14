use minijinja::Error;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn highlight_js(name: String) -> Result<String, Error> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];
    let html = highlighted_html_for_string(name.as_str(), &ss, &ss.syntaxes()[1], theme).unwrap();
    Ok(html)
}
