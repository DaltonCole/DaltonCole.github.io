//! The page shell shared by every templated page.

use std::time::{SystemTime, UNIX_EPOCH};

use leptos::{config::LeptosOptions, prelude::*};

const FONTS: &str = "https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@500;600;700&family=Inter:wght@400;500;600&display=swap";
const ANALYTICS_SCRIPT: &str = "https://www.googletagmanager.com/gtag/js?id=G-SPY7S5B91F";
const ANALYTICS_INIT: &str = "window.dataLayer=window.dataLayer||[];function gtag(){dataLayer.push(arguments);}gtag('js',new Date());gtag('config','G-SPY7S5B91F');";

/// Label, link, and the first path segment that marks it as the current section.
const NAV_LINKS: [(&str, &str, &str); 4] = [
    ("About", "/about/", "about"),
    ("Experience", "/work-experience/", "work-experience"),
    ("Résumé", "/resume/", "resume"),
    ("Projects", "/projects/", "projects"),
];

fn current_year() -> u64 {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    1970 + seconds / 31_556_952
}

#[component]
fn Header(url: String) -> impl IntoView {
    let section = url.split('/').next().unwrap_or("").to_string();
    let links = NAV_LINKS
        .into_iter()
        .map(|(label, href, name)| {
            view! { <a href=href class:active-link=section == name>{label}</a> }
        })
        .collect_view();

    // The menu opens with a checkbox so that ordinary pages ship no JavaScript.
    view! {
        <header class="site-header">
            <nav class="site-nav">
                <a class="brand" href="/">"DC"</a>
                <input type="checkbox" id="nav-toggle" class="nav-toggle-box" aria-label="Toggle navigation"/>
                <label for="nav-toggle" class="nav-toggle" aria-hidden="true">"☰"</label>
                <div class="nav-links">
                    {links}
                    <a class="nav-cta" href="/contact/">"Contact"</a>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <p>{format!("Copyright © 2020–{} Dalton Cole", current_year())}</p>
            <div class="footer-links">
                <a href="https://github.com/DaltonCole" target="_blank" rel="noopener">"GitHub"</a>
                <a href="https://www.linkedin.com/in/daltoncole1/" target="_blank" rel="noopener">"LinkedIn"</a>
            </div>
        </footer>
    }
}

/// A complete HTML document. `islands` loads the WebAssembly bundle, so only pages with an
/// interactive tool pay for it.
#[component]
pub fn Document(
    #[prop(into)] title: String,
    #[prop(into)] url: String,
    options: LeptosOptions,
    #[prop(optional)] islands: bool,
    children: Children,
) -> impl IntoView {
    let full_title = if url.is_empty() {
        "Dalton Cole".to_string()
    } else {
        format!("{title} | Dalton Cole")
    };

    view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="Dalton Cole — software engineer and researcher."/>
                <title>{full_title}</title>
                <link rel="icon" href="/static/home/images/favicon.ico"/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
                <link rel="stylesheet" href=FONTS/>
                <link rel="stylesheet" href="/pkg/daltoncole.css"/>
                <script async src=ANALYTICS_SCRIPT></script>
                <script inner_html=ANALYTICS_INIT></script>
                {islands.then(|| view! { <HydrationScripts options=options islands=true/> })}
            </head>
            <body>
                <Header url=url/>
                <main class="site-main">{children()}</main>
                <Footer/>
            </body>
        </html>
    }
}

/// A tool page: a link back to Projects, the title, and the tool.
#[component]
pub fn Article(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <section class="doc">
            <div class="container">
                <a class="back-link" href="/projects/">"← Projects"</a>
                <h1 class="doc-title">{title}</h1>
                {children()}
            </div>
        </section>
    }
}
