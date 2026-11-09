//! Renders the whole site to static files in the site root.

mod content;
mod layout;
mod markdown;
mod pages;
mod profile;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use leptos::config::LeptosOptions;

use content::Content;

const ORIGIN: &str = "https://daltoncole.com";

fn output_path(root: &Path, url: &str) -> PathBuf {
    match url {
        "" => root.join("index.html"),
        "404" => root.join("404.html"),
        _ => root.join(url).join("index.html"),
    }
}

fn sitemap(urls: &[String]) -> String {
    let entries: String = urls
        .iter()
        .map(|url| match url.as_str() {
            "" => format!("<url><loc>{ORIGIN}/</loc></url>\n"),
            _ => format!("<url><loc>{ORIGIN}/{url}/</loc></url>\n"),
        })
        .collect();
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{entries}</urlset>\n"
    )
}

pub fn generate(options: &LeptosOptions) -> io::Result<()> {
    let content = Content::load(Path::new("content"))?;
    let root = PathBuf::from(options.site_root.as_ref());
    let rendered = pages::render_all(options, &content);

    let mut listed = Vec::new();
    for page in &rendered {
        let path = output_path(&root, &page.url);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, &page.html)?;
        if page.in_sitemap {
            listed.push(page.url.clone());
        }
    }
    fs::write(root.join("sitemap.xml"), sitemap(&listed))?;

    println!("Generated {} pages in {}", rendered.len(), root.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{output_path, sitemap};

    #[test]
    fn urls_map_to_directory_indexes() {
        let root = Path::new("site");
        assert_eq!(output_path(root, ""), root.join("index.html"));
        assert_eq!(output_path(root, "404"), root.join("404.html"));
        assert_eq!(
            output_path(root, "learn/rust"),
            root.join("learn/rust/index.html")
        );
    }

    #[test]
    fn sitemap_uses_the_real_domain_and_trailing_slashes() {
        let xml = sitemap(&[String::new(), "learn/rust".to_string()]);
        assert!(xml.contains("<loc>https://daltoncole.com/</loc>"));
        assert!(xml.contains("<loc>https://daltoncole.com/learn/rust/</loc>"));
    }
}
