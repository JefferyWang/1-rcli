use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    info!("Serving {:?} on port {}", path, port);
    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, HeaderMap, String) {
    let mut headers = HeaderMap::new();
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            headers,
            format!("File not found: {:?}", p),
        )
    } else if p.is_dir() {
        match tokio::fs::read_dir(p.clone()).await {
            Ok(mut entries) => {
                let mut content = String::from("<html><body><ul>");
                while let Some(entry) = entries.next_entry().await.unwrap() {
                    info!("Reading directory {:?}", p);
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    let path = entry.path();
                    let path = path.strip_prefix(&state.path).unwrap();
                    let path = path.to_string_lossy();
                    content.push_str(&format!(r#"<li><a href="/{}">{}</a></li>"#, path, name));
                }
                content.push_str("</ul></body></html>");
                headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
                (StatusCode::OK, headers, content)
            }
            Err(e) => {
                warn!("Error reading directory: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, headers, e.to_string())
            }
        }
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, headers, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, headers, e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, headers, content) =
            file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
        assert_eq!(headers.get(header::CONTENT_TYPE), None);
    }
}
