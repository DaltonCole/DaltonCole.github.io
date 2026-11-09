//! Every page on the site, and the data each one is built from.

use std::collections::BTreeSet;

use leptos::{config::LeptosOptions, prelude::*};

use super::{
    content::{Content, Page},
    layout::{Article, Document},
    profile::{About, Card, Contact, Experience, Home, Resume},
};
use crate::islands::{
    ChineseRemainderSolver, MurderMysteryGame, NoteFilter, NoteSummary, PhaseTenScoreKeeper,
};

const SEARCH_SCRIPT: &str = "https://cse.google.com/cse.js?cx=013777161845054467269:a_8tseclx3y";
const NOTE_DISCLAIMER: &str = "Disclaimer: These are my personal notes on this paper. I am in no way related to this paper. All credits go towards the authors.";
const ACCENTS: [&str; 4] = ["violet", "amber", "cyan", "pink"];
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub struct Rendered {
    pub url: String,
    pub html: String,
    pub in_sitemap: bool,
}

fn to_document(build: impl FnOnce() -> AnyView) -> String {
    let owner = Owner::new();
    let html = owner.with(|| build().to_html());
    format!("<!DOCTYPE html>{html}")
}

fn parent_href(url: &str) -> String {
    match url.rsplit_once('/') {
        Some((parent, _)) => format!("/{parent}/"),
        None => "/".to_string(),
    }
}

/// Full display name of a Learn page, for example `Cpp - Keywords - Constexpr`.
fn learn_title(content: &Content, url: &str) -> String {
    let segments: Vec<&str> = url.split('/').collect();
    (2..=segments.len())
        .map(|depth| {
            content
                .page(&segments[..depth].join("/"))
                .meta
                .title
                .clone()
        })
        .collect::<Vec<_>>()
        .join(" - ")
}

/// Parent Learn pages are assembled from their children, so each topic is written only once.
fn learn_article(content: &Content, page: &Page) -> AnyView {
    let children = content.children(&page.url);
    let sections: Vec<(String, String)> = children
        .iter()
        .map(|child| (child.slug().to_string(), child.meta.title.clone()))
        .collect();

    let body = children
        .into_iter()
        .map(|child| {
            let slug = child.slug().to_string();
            let topics = content
                .children(&child.url)
                .into_iter()
                .map(|topic| {
                    view! {
                        <h4 id=format!("{slug}-{}", topic.slug())>{topic.meta.title.clone()}</h4>
                        <div inner_html=topic.html.clone()></div>
                    }
                })
                .collect_view();
            view! {
                <h2 id=slug.clone()><a href=child.href()>{child.meta.title.clone()}</a></h2>
                <div inner_html=child.html.clone()></div>
                {topics}
            }
        })
        .collect_view();
    let intro = page.html.clone();

    view! {
        <Article
            title=learn_title(content, &page.url)
            back=parent_href(&page.url)
            anchor=page.slug().to_string()
            sections=sections
        >
            <article class="prose">
                <div inner_html=intro></div>
                {body}
            </article>
        </Article>
    }
    .into_any()
}

fn learn_index(content: &Content) -> AnyView {
    let cards = content
        .children("learn")
        .into_iter()
        .enumerate()
        .map(|(index, subject)| {
            let description = subject.meta.description.clone().unwrap_or_default();
            view! {
                <Card
                    href=subject.href()
                    title=subject.meta.title.clone()
                    accent=ACCENTS[index % ACCENTS.len()]
                >
                    <p>{description}</p>
                </Card>
            }
        })
        .collect_view();
    view! {
        <section class="doc">
            <div class="container wide">
                <div class="eyebrow" style="color: var(--cyan)">"Notes"</div>
                <h1 class="doc-title">"Learn"</h1>
                <p class="lede">"Spend some time learning about a variety of topics!"</p>
                <div class="card-grid">{cards}</div>
            </div>
        </section>
    }
    .into_any()
}

fn long_date(iso: &str) -> String {
    let mut parts = iso.split('-');
    let (year, month, day) = (parts.next(), parts.next(), parts.next());
    match (
        year,
        month.and_then(|m| m.parse::<usize>().ok()),
        day.and_then(|d| d.parse::<u32>().ok()),
    ) {
        (Some(year), Some(month @ 1..=12), Some(day)) => {
            format!("{} {day}, {year}", MONTHS[month - 1])
        }
        _ => iso.to_string(),
    }
}

fn note_article(page: &Page) -> AnyView {
    let date = long_date(page.meta.date.as_deref().unwrap_or_default());
    let tags = page.meta.tags.join(", ");
    let paper = page.meta.paper_url.clone().map(|href| {
        view! { " · " <a href=href target="_blank" rel="noopener">"Paper Link"</a> }
    });
    let html = page.html.clone();
    view! {
        <Article title=page.meta.title.clone() back="/research/notes/">
            <p class="doc-meta">
                {date}
                {paper}
                {(!tags.is_empty()).then(|| format!(" · Tags: {tags}"))}
            </p>
            <p class="doc-note">{NOTE_DISCLAIMER}</p>
            <article class="prose" inner_html=html></article>
        </Article>
    }
    .into_any()
}

fn notes_index(content: &Content) -> AnyView {
    let mut pages = content.children("research/notes");
    pages.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));

    let tags: Vec<String> = pages
        .iter()
        .flat_map(|page| page.meta.tags.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let count = pages.len();
    let notes: Vec<NoteSummary> = pages
        .iter()
        .map(|page| NoteSummary {
            href: page.href(),
            title: page.meta.title.clone(),
            citations: page.meta.citations,
            month: page
                .meta
                .date
                .clone()
                .unwrap_or_default()
                .chars()
                .take(7)
                .collect(),
            tags: page.meta.tags.clone(),
        })
        .collect();

    view! {
        <Article title="Paper Notes" back="/research/">
            <p class="doc-meta">{format!("Paper Count: {count}")}</p>
            <NoteFilter notes=notes tags=tags/>
        </Article>
    }
    .into_any()
}

fn kattis_article(page: &Page) -> AnyView {
    let problem = page.meta.kattis_url.clone().unwrap_or_default();
    let difficulty = page.meta.difficulty.clone().unwrap_or_default();
    let html = page.html.clone();
    view! {
        <Article title=page.meta.title.clone() back="/writeups/kattis/">
            <p class="doc-meta">
                <a href=problem target="_blank" rel="noopener">"View on Kattis"</a>
                {format!(" · Difficulty: {difficulty}")}
            </p>
            <article class="prose" inner_html=html></article>
        </Article>
    }
    .into_any()
}

fn kattis_index(content: &Content) -> AnyView {
    let mut problems = content.children("writeups/kattis");
    problems.sort_by_key(|page| page.meta.title.to_lowercase());

    let rows = problems
        .into_iter()
        .map(|page| {
            let languages = page.meta.languages.join(", ");
            let (mark, label) = if page.meta.writeup { ("✓", "Yes") } else { ("✗", "No") };
            view! {
                <tr>
                    <th scope="row"><a href=page.href()>{page.meta.title.clone()}</a></th>
                    <td>{page.meta.difficulty.clone().unwrap_or_default()}</td>
                    <td class=if page.meta.writeup { "mark-yes" } else { "mark-no" } aria-label=label>{mark}</td>
                    <td>{languages}</td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <Article title="Kattis Problems" back="/writeups/">
            <div class="table-scroll">
                <table class="tool-table">
                    <thead>
                        <tr>
                            <th scope="col">"Name"</th>
                            <th scope="col">"Difficulty*"</th>
                            <th scope="col">"Write-Up"</th>
                            <th scope="col">"Languages"</th>
                        </tr>
                    </thead>
                    <tbody>{rows}</tbody>
                </table>
            </div>
            <p class="doc-note">"* Difficulty when problem was completed"</p>
        </Article>
    }
    .into_any()
}

fn plain_article(page: &Page) -> AnyView {
    let html = page.html.clone();
    view! {
        <Article title=page.meta.title.clone() back=parent_href(&page.url)>
            <article class="prose" inner_html=html></article>
        </Article>
    }
    .into_any()
}

/// A section landing page made of cards.
fn card_page(
    eyebrow: &'static str,
    title: &'static str,
    cards: impl IntoView + 'static,
) -> AnyView {
    view! {
        <section class="doc">
            <div class="container wide">
                <div class="eyebrow" style="color: var(--violet)">{eyebrow}</div>
                <h1 class="doc-title">{title}</h1>
                <div class="card-grid">{cards}</div>
            </div>
        </section>
    }
    .into_any()
}

fn research_index() -> AnyView {
    card_page(
        "Research",
        "My Research",
        view! {
            <Card href="/research/papers/" title="Papers" accent="violet">
                <p>"Peer-reviewed publications."</p>
            </Card>
            <Card href="/research/projects/" title="Projects" accent="amber">
                <p>"Research projects and their code."</p>
            </Card>
            <Card href="/research/notes/" title="Notes" accent="cyan">
                <p>"Personal notes on papers in adversarial machine learning and deepfakes."</p>
            </Card>
        },
    )
}

fn writeups_index() -> AnyView {
    card_page(
        "Write-Ups",
        "Write-Ups",
        view! {
            <Card href="/writeups/kattis/" title="Kattis" accent="pink">
                <p>"Kattis is a programming problems platform with thousands of programming problems from a range of difficulties!"</p>
            </Card>
        },
    )
}

fn projects_index() -> AnyView {
    card_page(
        "Projects",
        "Projects",
        view! {
            <Card href="/projects/chinese-remainder-theorem/" title="Chinese Remainder Theorem" accent="cyan">
                <p>"The Chinese remainder theorem is used to find the given remainders when a number p is divided by some given divisors."</p>
            </Card>
            <Card href="/projects/phase-10-score-keeper/" title="Phase 10 Score Keeper" accent="amber">
                <p>"Keep track of each player's phase and score for Phase 10!"</p>
            </Card>
        },
    )
}

fn crt_page() -> AnyView {
    view! {
        <Article title="Chinese Remainder Theorem" back="/projects/">
            <p class="doc-meta">
                <a href="https://brilliant.org/wiki/chinese-remainder-theorem/" target="_blank" rel="noopener">
                    "What is the Chinese remainder theorem?"
                </a>
            </p>
            <ChineseRemainderSolver/>
        </Article>
    }
    .into_any()
}

fn phase_ten_page() -> AnyView {
    view! {
        <Article title="Phase 10 Score Keeper" back="/projects/">
            <PhaseTenScoreKeeper/>
        </Article>
    }
    .into_any()
}

fn search_page() -> AnyView {
    view! {
        <Article title="Search">
            <script async src=SEARCH_SCRIPT></script>
            <div class="search-results">
                <div class="gcse-searchresults-only"></div>
            </div>
        </Article>
    }
    .into_any()
}

fn not_found_page() -> AnyView {
    view! {
        <section class="hero">
            <div class="container">
                <div class="eyebrow hero-tag">"404"</div>
                <h1>"Page not found."</h1>
                <p class="lede">"That page doesn't exist, or it has moved."</p>
                <div class="button-row">
                    <a class="btn btn-primary" href="/">"Go home"</a>
                    <a class="btn btn-secondary" href="/search/">"Search the site"</a>
                </div>
            </div>
        </section>
    }
    .into_any()
}

fn murder_mystery_document(options: LeptosOptions) -> AnyView {
    view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"Medieval Murder Mystery"</title>
                <link rel="stylesheet" href="/static/murder-mystery/style.css"/>
                <HydrationScripts options=options islands=true/>
            </head>
            <body>
                <MurderMysteryGame/>
            </body>
        </html>
    }
    .into_any()
}

#[derive(Clone, Copy, Default)]
struct Extras {
    math: bool,
    islands: bool,
}

struct Builder<'a> {
    options: &'a LeptosOptions,
    pages: Vec<Rendered>,
}

impl Builder<'_> {
    fn add(
        &mut self,
        url: &str,
        title: String,
        extras: Extras,
        in_sitemap: bool,
        body: impl FnOnce() -> AnyView,
    ) {
        let options = self.options.clone();
        let html = to_document(|| {
            let body = body();
            let doc_url = url.to_string();
            view! {
                <Document
                    title=title
                    url=doc_url
                    options=options
                    math=extras.math
                    islands=extras.islands
                >
                    {body}
                </Document>
            }
            .into_any()
        });
        self.pages.push(Rendered {
            url: url.to_string(),
            html,
            in_sitemap,
        });
    }

    fn listed(&mut self, url: &str, title: &str, body: impl FnOnce() -> AnyView) {
        self.add(url, title.to_string(), Extras::default(), true, body);
    }

    fn tool(&mut self, url: &str, title: &str, body: impl FnOnce() -> AnyView) {
        let extras = Extras {
            islands: true,
            ..Extras::default()
        };
        self.add(url, title.to_string(), extras, true, body);
    }

    /// One page per Markdown file below `prefix`.
    fn add_content_pages(
        &mut self,
        content: &Content,
        pages: Vec<&Page>,
        title: impl Fn(&Page) -> String,
        body: impl Fn(&Page) -> AnyView,
    ) {
        for page in pages {
            let extras = Extras {
                math: content.needs_math(&page.url),
                islands: false,
            };
            self.add(&page.url, title(page), extras, true, || body(page));
        }
    }
}

pub fn render_all(options: &LeptosOptions, content: &Content) -> Vec<Rendered> {
    let mut site = Builder {
        options,
        pages: Vec::new(),
    };

    site.listed("", "Home", || view! { <Home/> }.into_any());
    site.listed("about", "About", || view! { <About/> }.into_any());
    site.listed("work-experience", "Experience", || {
        view! { <Experience/> }.into_any()
    });
    site.listed("contact", "Contact", || view! { <Contact/> }.into_any());
    site.listed("resume", "Résumé", || view! { <Resume/> }.into_any());
    site.listed("search", "Search", search_page);
    site.add(
        "404",
        "Page Not Found".to_string(),
        Extras::default(),
        false,
        not_found_page,
    );

    site.listed("learn", "Learn", || learn_index(content));
    site.add_content_pages(
        content,
        content.descendants("learn"),
        |page| learn_title(content, &page.url),
        |page| learn_article(content, page),
    );

    site.listed("research", "Research", research_index);
    site.tool("research/notes", "Paper Notes", || notes_index(content));
    site.add_content_pages(
        content,
        content.children("research/notes"),
        |page| page.meta.title.clone(),
        note_article,
    );

    site.listed("writeups", "Write-Ups", writeups_index);
    site.listed("writeups/kattis", "Kattis Problems", || {
        kattis_index(content)
    });
    site.add_content_pages(
        content,
        content.children("writeups/kattis"),
        |page| page.meta.title.clone(),
        kattis_article,
    );

    site.listed("projects", "Projects", projects_index);
    site.tool(
        "projects/chinese-remainder-theorem",
        "Chinese Remainder Theorem",
        crt_page,
    );
    site.tool(
        "projects/phase-10-score-keeper",
        "Phase 10 Score Keeper",
        phase_ten_page,
    );

    let standalone = [
        "research/papers",
        "research/projects",
        "course-history",
        "computer-skills",
        "blog",
        "blog/project",
        "blog/project/path-finding-visualizer",
    ];
    let plain: Vec<&Page> = standalone.iter().map(|url| content.page(url)).collect();
    site.add_content_pages(
        content,
        plain,
        |page| page.meta.title.clone(),
        plain_article,
    );

    let game = to_document(|| murder_mystery_document(options.clone()));
    site.pages.push(Rendered {
        url: "murder-mystery".to_string(),
        html: game,
        in_sitemap: false,
    });

    site.pages
}

#[cfg(test)]
mod tests {
    use super::{long_date, parent_href};

    #[test]
    fn dates_read_naturally() {
        assert_eq!(long_date("2017-03-09"), "March 9, 2017");
        assert_eq!(long_date("not a date"), "not a date");
    }

    #[test]
    fn back_links_go_up_one_level() {
        assert_eq!(parent_href("learn/rust/traits"), "/learn/rust/");
        assert_eq!(parent_href("about"), "/");
    }
}
