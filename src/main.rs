#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::get_configuration;
    use tower_http::services::{ServeDir, ServeFile};

    let options = get_configuration(None)
        .expect("reading [package.metadata.leptos] from Cargo.toml")
        .leptos_options;

    daltoncole::site::generate(&options).expect("generating the site");

    let serve =
        std::env::args().any(|arg| arg == "--serve") || std::env::var_os("LEPTOS_WATCH").is_some();
    if !serve {
        return;
    }

    let root = options.site_root.to_string();
    let files = ServeDir::new(&root).not_found_service(ServeFile::new(format!("{root}/404.html")));
    let listener = tokio::net::TcpListener::bind(options.site_addr)
        .await
        .expect("binding the preview server");
    println!("Preview at http://{}", options.site_addr);
    axum::serve(listener, Router::new().fallback_service(files))
        .await
        .expect("running the preview server");
}

#[cfg(not(feature = "ssr"))]
fn main() {}
