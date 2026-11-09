//! Every page on the site, and how each one is assembled.

use leptos::{config::LeptosOptions, prelude::*};

use super::{
    layout::{Article, Document},
    profile::{About, Contact, Experience, Home, Resume},
};
use crate::islands::{ChineseRemainderSolver, MurderMysteryGame, PhaseTenScoreKeeper};

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

struct Project {
    href: &'static str,
    title: &'static str,
    kind: &'static str,
    accent: &'static str,
    summary: &'static str,
    features: &'static [&'static str],
    tags: &'static [&'static str],
    source: &'static str,
}

const PROJECTS: [Project; 2] = [
    Project {
        href: "/projects/phase-10-score-keeper/",
        title: "Phase 10 Score Keeper",
        kind: "Game night",
        accent: "amber",
        summary: "Keeps score for a game of Phase 10 from a phone or laptop, so nobody has to do the math on paper.",
        features: &[
            "Tracks every player's phase and running total",
            "Shows what each player's current phase asks for",
            "Saves the game in your browser, with undo for a mistyped round",
        ],
        tags: &["Rust", "Leptos", "WebAssembly", "SVG charts"],
        source: "https://github.com/DaltonCole/DaltonCole.github.io/blob/main/src/islands/phase10.rs",
    },
    Project {
        href: "/projects/chinese-remainder-theorem/",
        title: "Chinese Remainder Theorem",
        kind: "Number theory",
        accent: "cyan",
        summary: "Solves a system of congruences as you type, and walks through how the answer was found.",
        features: &[
            "Handles any number of equations",
            "Shows each step, including the modular inverses",
            "Names the two moduli at fault when a system can't be solved",
        ],
        tags: &["Rust", "Leptos", "WebAssembly", "Modular arithmetic"],
        source: "https://github.com/DaltonCole/DaltonCole.github.io/blob/main/src/islands/crt.rs",
    },
];

fn project_card(project: &Project) -> impl IntoView {
    let style = format!(
        "--card-accent: var(--{0}); --card-accent-soft: var(--{0}-soft);",
        project.accent
    );
    let features = project
        .features
        .iter()
        .map(|feature| view! { <li>{*feature}</li> })
        .collect_view();
    let tags = project
        .tags
        .iter()
        .map(|tag| view! { <li class="pill">{*tag}</li> })
        .collect_view();

    view! {
        <article class="project" style=style>
            <div class="project-kind">{project.kind}</div>
            <h2>{project.title}</h2>
            <p>{project.summary}</p>
            <ul class="project-features">{features}</ul>
            <ul class="pill-list" aria-label="Built with">{tags}</ul>
            <div class="button-row project-links">
                <a class="btn btn-primary" href=project.href>"Open tool"</a>
                <a class="btn btn-secondary" href=project.source target="_blank" rel="noopener">"View source"</a>
            </div>
        </article>
    }
}

fn projects_index() -> AnyView {
    let cards = PROJECTS.iter().map(project_card).collect_view();
    view! {
        <section class="doc">
            <div class="container wide">
                <div class="eyebrow" style="color: var(--pink)">"Projects"</div>
                <h1 class="doc-title">"Things I've built"</h1>
                <p class="lede">
                    "Small tools that run entirely in your browser, written in Rust and compiled to "
                    "WebAssembly. No server and no sign-up."
                </p>
                <div class="project-grid">{cards}</div>
            </div>
        </section>
    }
    .into_any()
}

fn crt_page() -> AnyView {
    view! {
        <Article title="Chinese Remainder Theorem">
            <p class="lede">"Enter a system of congruences to find x, and see how it's solved."</p>
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
        <Article title="Phase 10 Score Keeper">
            <p class="lede">"Add your players, then enter everyone's points after each round. Your game is saved in this browser."</p>
            <PhaseTenScoreKeeper/>
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
                <p class="lede">"That page doesn't exist, or it's no longer part of this site."</p>
                <div class="button-row">
                    <a class="btn btn-primary" href="/">"Go home"</a>
                    <a class="btn btn-secondary" href="/projects/">"See projects"</a>
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

struct Builder<'a> {
    options: &'a LeptosOptions,
    pages: Vec<Rendered>,
}

impl Builder<'_> {
    fn add(
        &mut self,
        url: &str,
        title: &str,
        islands: bool,
        in_sitemap: bool,
        body: impl FnOnce() -> AnyView,
    ) {
        let options = self.options.clone();
        let html = to_document(|| {
            let body = body();
            let doc_url = url.to_string();
            let title = title.to_string();
            view! {
                <Document title=title url=doc_url options=options islands=islands>
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
        self.add(url, title, false, true, body);
    }

    fn tool(&mut self, url: &str, title: &str, body: impl FnOnce() -> AnyView) {
        self.add(url, title, true, true, body);
    }
}

pub fn render_all(options: &LeptosOptions) -> Vec<Rendered> {
    let mut site = Builder {
        options,
        pages: Vec::new(),
    };

    site.listed("", "Home", || view! { <Home/> }.into_any());
    site.listed("about", "About", || view! { <About/> }.into_any());
    site.listed("work-experience", "Experience", || {
        view! { <Experience/> }.into_any()
    });
    site.listed("resume", "Résumé", || view! { <Resume/> }.into_any());
    site.listed("contact", "Contact", || view! { <Contact/> }.into_any());
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
    site.add("404", "Page Not Found", false, false, not_found_page);

    let game = to_document(|| murder_mystery_document(options.clone()));
    site.pages.push(Rendered {
        url: "murder-mystery".to_string(),
        html: game,
        in_sitemap: false,
    });

    site.pages
}
