//! Loads `content/**/*.md`. A file's path is its URL: `content/learn/rust/traits.md` is
//! served at `/learn/rust/traits/`.

use std::{collections::BTreeMap, fs, io, path::Path};

use serde::Deserialize;
use walkdir::WalkDir;

use super::markdown;

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Meta {
    pub title: String,
    /// One-line summary, shown on the page that lists this one.
    pub description: Option<String>,
    /// Enables `$...$` math on this page.
    pub math: bool,
    // Research notes
    pub date: Option<String>,
    pub paper_url: Option<String>,
    pub tags: Vec<String>,
    pub citations: u32,
    // Kattis write-ups
    pub kattis_url: Option<String>,
    pub difficulty: Option<String>,
    pub languages: Vec<String>,
    pub writeup: bool,
}

#[derive(Debug)]
pub struct Page {
    /// Path without surrounding slashes, for example `learn/rust/traits`.
    pub url: String,
    pub meta: Meta,
    pub html: String,
}

impl Page {
    pub fn slug(&self) -> &str {
        self.url.rsplit('/').next().unwrap_or(&self.url)
    }

    pub fn href(&self) -> String {
        format!("/{}/", self.url)
    }
}

fn invalid(path: &Path, message: impl std::fmt::Display) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("{}: {message}", path.display()),
    )
}

fn parse(path: &Path, url: String, source: &str) -> io::Result<Page> {
    let rest = source
        .strip_prefix("+++\n")
        .ok_or_else(|| invalid(path, "missing +++ front matter"))?;
    let (front, body) = rest
        .split_once("\n+++")
        .ok_or_else(|| invalid(path, "unterminated +++ front matter"))?;
    let meta: Meta = toml::from_str(front).map_err(|e| invalid(path, e))?;
    if meta.title.is_empty() {
        return Err(invalid(path, "front matter needs a title"));
    }
    let html = markdown::render(body, meta.math);
    Ok(Page { url, meta, html })
}

pub struct Content {
    pages: BTreeMap<String, Page>,
}

impl Content {
    pub fn load(dir: &Path) -> io::Result<Self> {
        let mut pages = BTreeMap::new();
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "md") {
                continue;
            }
            let relative = path.strip_prefix(dir).map_err(|e| invalid(path, e))?;
            let url = relative
                .with_extension("")
                .to_string_lossy()
                .replace('\\', "/");
            let page = parse(path, url.clone(), &fs::read_to_string(path)?)?;
            pages.insert(url, page);
        }
        Ok(Self { pages })
    }

    pub fn page(&self, url: &str) -> &Page {
        self.pages
            .get(url)
            .unwrap_or_else(|| panic!("content/{url}.md is missing"))
    }

    /// Pages directly below `url`, in slug order.
    pub fn children(&self, url: &str) -> Vec<&Page> {
        self.pages
            .values()
            .filter(|page| {
                page.url
                    .rsplit_once('/')
                    .is_some_and(|(parent, _)| parent == url)
            })
            .collect()
    }

    /// Every page below `url`, at any depth.
    pub fn descendants(&self, url: &str) -> Vec<&Page> {
        let prefix = format!("{url}/");
        self.pages
            .values()
            .filter(|page| page.url.starts_with(&prefix))
            .collect()
    }

    /// True if this page, or anything assembled into it, contains math.
    pub fn needs_math(&self, url: &str) -> bool {
        self.page(url).meta.math || self.descendants(url).iter().any(|page| page.meta.math)
    }
}
