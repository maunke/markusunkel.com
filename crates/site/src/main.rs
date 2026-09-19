#[cfg(feature = "ssr")]
async fn strip_trailing_slash(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::{IntoResponse, Redirect};

    let uri = request.uri();
    let path = uri.path();
    if path.len() > 1 && path.ends_with('/') {
        let trimmed = path.trim_end_matches('/');
        let target = match (trimmed.is_empty(), uri.query()) {
            (true, _) => "/".to_string(),
            (false, Some(query)) => format!("{trimmed}?{query}"),
            (false, None) => trimmed.to_string(),
        };
        return Redirect::permanent(&target).into_response();
    }
    next.run(request).await
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::http::Response;
    use axum::http::header::CACHE_CONTROL;
    use axum::{Router, http::HeaderValue};
    use leptos::{logging::log, prelude::*};
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use markusunkel_com::app::*;
    use markusunkel_com::content::Content;
    use markusunkel_com::rss::rss_xml;
    use markusunkel_com::sitemap::sitemap_xml;
    use std::sync::Arc;
    use tower::ServiceBuilder;
    use tower_http::compression::CompressionLayer;
    use tower_http::services::ServeFile;
    use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let mut leptos_options = conf.leptos_options;
    leptos_options.hash_files = true;

    let routes = generate_route_list({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    let content = Content::load().unwrap();

    let sitemap_xml_str: Arc<str> = sitemap_xml(&content).into();
    let rss_xml_str: Arc<str> = rss_xml(&content).into();

    let immutable = SetResponseHeaderLayer::overriding(CACHE_CONTROL, |response: &Response<_>| {
        response
            .status()
            .is_success()
            .then(|| HeaderValue::from_static("public, max-age=31536000, immutable"))
    });

    let app = Router::new()
        .nest_service(
            "/assets",
            ServiceBuilder::new()
                .layer(immutable.clone())
                .service(ServeDir::new("target/site-assets").precompressed_br()),
        )
        .nest_service(
            "/media",
            ServiceBuilder::new().layer(immutable.clone()).service(
                ServeDir::new("target/site/media")
                    .precompressed_br()
                    .precompressed_gzip()
                    .precompressed_zstd(),
            ),
        )
        .nest_service(
            "/fonts",
            ServiceBuilder::new()
                .layer(immutable.clone())
                .service(ServeDir::new("target/site/fonts")),
        )
        .nest_service(
            "/pkg",
            ServiceBuilder::new().layer(immutable).service(
                ServeDir::new("target/site/pkg")
                    .precompressed_br()
                    .precompressed_gzip()
                    .precompressed_zstd(),
            ),
        )
        .route(
            "/sitemap.xml",
            axum::routing::get({
                let xml = Arc::clone(&sitemap_xml_str);
                move || {
                    let xml = Arc::clone(&xml);
                    async move {
                        use axum::http::header::CONTENT_TYPE;
                        ([(CONTENT_TYPE, "application/xml")], xml.to_string())
                    }
                }
            }),
        )
        .route(
            "/rss.xml",
            axum::routing::get({
                let xml = Arc::clone(&rss_xml_str);
                move || {
                    let xml = Arc::clone(&xml);
                    async move {
                        use axum::http::header::CONTENT_TYPE;
                        ([(CONTENT_TYPE, "application/rss+xml")], xml.to_string())
                    }
                }
            }),
        )
        .route_service("/robots.txt", ServeFile::new("target/site/robots.txt"))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let content = content.clone();
                move || provide_context(content.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .route(
            "/",
            axum::routing::get(leptos_axum::render_app_to_stream_with_context(
                {
                    let content = content.clone();
                    move || provide_context(content.clone())
                },
                {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                },
            )),
        )
        .fallback(leptos_axum::file_and_error_handler_with_context(
            move || {
                provide_context(content.clone());
            },
            shell,
        ))
        .with_state(leptos_options)
        .layer(CompressionLayer::new())
        .layer(axum::middleware::from_fn(strip_trailing_slash));

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
