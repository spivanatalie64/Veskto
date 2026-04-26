use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{http::header::CONTENT_TYPE, AppHandle, Manager};

const MIME_TYPES: &[(&str, &str)] = &[
    (".html", "text/html"),
    (".css", "text/css"),
    (".js", "application/javascript"),
    (".json", "application/json"),
    (".png", "image/png"),
    (".jpg", "image/jpeg"),
    (".jpeg", "image/jpeg"),
    (".gif", "image/gif"),
    (".svg", "image/svg+xml"),
    (".ico", "image/x-icon"),
    (".woff", "font/woff"),
    (".woff2", "font/woff2"),
    (".ttf", "font/ttf"),
    (".otf", "font/otf"),
    (".node", "application/octet-stream"),
];

fn get_mime_type(path: &str) -> &str {
    let ext = path.rsplit('.').next().unwrap_or("");
    let ext = format!(".{}", ext);
    MIME_TYPES
        .iter()
        .find(|(k, _)| *k == ext)
        .map(|(_, v)| *v)
        .unwrap_or("application/octet-stream")
}

pub fn register_protocol(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = crate::utils::paths::get_data_dir();
    let assets_dir = data_dir.join("assets");
    let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../static");

    let handles = HashMap::from([
        ("assets".to_string(), assets_dir),
        ("static".to_string(), static_dir),
    ]);

    app.register_asynchronous_uri_scheme_protocol("vesktop", move |_app, request, responder| {
        let url = request.uri().to_string();
        let path = url.strip_prefix("vesktop://").unwrap_or("");

        let (scheme, file_path) = path.split_once('/').unwrap_or(("", path));
        let base_dir = handles.get(scheme);

        if base_dir.is_none() {
            return responder.respond(
                tauri::http::Response::builder()
                    .status(404)
                    .body(b"Unknown scheme".to_vec())
                    .unwrap(),
            );
        }

        let base_dir = base_dir.unwrap();
        let full_path = base_dir.join(file_path);

        if !full_path.exists() || !full_path.is_file() {
            return responder.respond(
                tauri::http::Response::builder()
                    .status(404)
                    .body(b"Not found".to_vec())
                    .unwrap(),
            );
        }

        let content = match std::fs::read(&full_path) {
            Ok(c) => c,
            Err(_) => {
                return responder.respond(
                    tauri::http::Response::builder()
                        .status(500)
                        .body(b"Read error".to_vec())
                        .unwrap(),
                )
            }
        };

        let mime_type = get_mime_type(file_path);
        responder.respond(
            tauri::http::Response::builder()
                .header(CONTENT_TYPE, mime_type)
                .header("Access-Control-Allow-Origin", "*")
                .status(200)
                .body(content)
                .unwrap(),
        );
    })?;

    Ok(())
}
