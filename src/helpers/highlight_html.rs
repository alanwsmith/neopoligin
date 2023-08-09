use minijinja::Error;
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxReference;
use syntect::parsing::SyntaxSet;

pub fn highlight_html(name: String) -> Result<String, Error> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    // let sr = SyntaxReference { "asdf".to_string(),};
    let theme = &ts.themes["base16-ocean.dark"];
    let html = highlighted_html_for_string(name.as_str(), &ss, &ss.syntaxes()[0], theme).unwrap();
    Ok(html)
    // Ok("asdf".to_string())
}
