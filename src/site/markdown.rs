//! Markdown to HTML, with code highlighted at build time so pages need no highlighting script.

use std::sync::OnceLock;

use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

const CLASS_STYLE: ClassStyle = ClassStyle::SpacedPrefixed { prefix: "hl-" };

fn syntaxes() -> &'static SyntaxSet {
    static SYNTAXES: OnceLock<SyntaxSet> = OnceLock::new();
    SYNTAXES.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn highlight(code: &str, lang: &str) -> String {
    let set = syntaxes();
    let highlighted = (!lang.is_empty())
        .then(|| set.find_syntax_by_token(lang))
        .flatten()
        .and_then(|syntax| {
            let mut generator =
                ClassedHTMLGenerator::new_with_class_style(syntax, set, CLASS_STYLE);
            for line in LinesWithEndings::from(code) {
                generator
                    .parse_html_for_line_which_includes_newline(line)
                    .ok()?;
            }
            Some(generator.finalize())
        });
    let body = highlighted.unwrap_or_else(|| escape(code));
    format!("<pre><code>{body}</code></pre>\n")
}

fn is_external(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Renders Markdown to HTML. `math` turns on `$...$` and `$$...$$`, for MathJax to typeset.
/// It is per-page because `$` is ordinary text on most pages, especially the shell ones.
pub fn render(source: &str, math: bool) -> String {
    let mut options = Options::ENABLE_TABLES | Options::ENABLE_HEADING_ATTRIBUTES;
    if math {
        options |= Options::ENABLE_MATH;
    }

    let mut events = Vec::new();
    let mut code: Option<(String, String)> = None;
    let mut links = Vec::new();

    for event in Parser::new_ext(source, options) {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(info) => {
                        info.split_whitespace().next().unwrap_or("").to_string()
                    }
                    CodeBlockKind::Indented => String::new(),
                };
                code = Some((lang, String::new()));
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((lang, text)) = code.take() {
                    events.push(Event::Html(highlight(&text, &lang).into()));
                }
            }
            Event::Text(text) if code.is_some() => {
                if let Some((_, buffer)) = code.as_mut() {
                    buffer.push_str(&text);
                }
            }
            Event::Start(Tag::Link { ref dest_url, .. }) if is_external(dest_url) => {
                links.push(true);
                let tag = format!(
                    "<a href=\"{}\" target=\"_blank\" rel=\"noopener\">",
                    escape(dest_url)
                );
                events.push(Event::InlineHtml(tag.into()));
            }
            Event::Start(Tag::Link { .. }) => {
                links.push(false);
                events.push(event);
            }
            Event::End(TagEnd::Link) => {
                if links.pop() == Some(true) {
                    events.push(Event::InlineHtml("</a>".into()));
                } else {
                    events.push(event);
                }
            }
            Event::InlineMath(tex) => {
                events.push(Event::InlineHtml(format!("\\({}\\)", escape(&tex)).into()));
            }
            Event::DisplayMath(tex) => {
                events.push(Event::InlineHtml(format!("\\[{}\\]", escape(&tex)).into()));
            }
            other => events.push(other),
        }
    }

    let mut output = String::with_capacity(source.len() * 2);
    html::push_html(&mut output, events.into_iter());
    output
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn dollars_are_text_unless_math_is_on() {
        assert!(render("echo $HOME and $PATH", false).contains("$HOME and $PATH"));
        assert!(render("a $x_1$ b", true).contains("\\(x_1\\)"));
    }

    #[test]
    fn external_links_open_in_a_new_tab() {
        let html = render("[a](https://example.com) [b](/learn/)", false);
        assert!(html.contains("href=\"https://example.com\" target=\"_blank\""));
        assert!(html.contains("<a href=\"/learn/\">b</a>"));
    }

    #[test]
    fn code_is_highlighted_and_escaped() {
        let html = render("```rust\nlet x: Vec<i32> = vec![];\n```", false);
        assert!(html.contains("hl-"));
        assert!(html.contains("&lt;"));
        assert!(!html.contains("Vec<i32>"));
    }

    #[test]
    fn unknown_languages_fall_back_to_plain_text() {
        let html = render("```notalanguage\na < b\n```", false);
        assert!(html.contains("<pre><code>a &lt; b"));
    }

    #[test]
    fn heading_ids_are_kept() {
        assert!(render("## Title {#custom}", false).contains("id=\"custom\""));
    }
}
