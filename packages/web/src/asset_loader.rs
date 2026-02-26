use crate::storage::{fnv1a_hex, WebStorage};
use dioxus::logger::tracing;
use dot_repl_ui::PlatformStorage as _;
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};

/// Pre-populate LocalStorage with DOT files from static assets.
///
/// ## Freshness strategy
///
/// We want two things to be true simultaneously:
///
/// 1. **Users keep their in-session edits** – while the page is open the
///    running `dot_input` signal holds the user's work in memory; we never
///    touch that here.
/// 2. **Server content is the source of truth on reload** – when the page
///    reloads (or on first visit), we sync LocalStorage with the server's
///    manifest. Any file on the server overwrites the local copy if it
///    differs (resetting any persistent edits from a previous session).
///    Files no longer in the manifest are removed from storage.
///
/// This ensures "The user has until the page reloads in order to do something
/// with their changes" (e.g. download or copy them).
pub async fn preload_dot_files(storage: &WebStorage, dots_folder: &str) -> Result<usize, String> {
    tracing::info!("Fetching DOT files manifest from {}...", dots_folder);

    let manifest_url = format!("{}/manifest.json", dots_folder);

    let filenames: Vec<String> = match fetch_json(&manifest_url).await {
        Ok(files) => files,
        Err(e) => {
            tracing::warn!("Failed to fetch manifest: {}, no files to preload", e);
            return Ok(0);
        }
    };

    tracing::info!("Found {} DOT files in manifest", filenames.len());

    let mut loaded_count = 0;
    let mut manifest_filenames = HashSet::new();

    for filename in &filenames {
        manifest_filenames.insert(filename.clone());
        let url = format!("{}/{}", dots_folder, filename);
        tracing::info!("Fetching DOT url {}", url);

        let content = match fetch_string(&url).await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Failed to fetch {}: {}", url, e);
                continue;
            }
        };

        let server_hash = fnv1a_hex(content.as_bytes());
        let current_local_content = storage.load(filename).ok();
        
        let needs_update = current_local_content
            .as_ref()
            .map(|c| c != content.as_bytes())
            .unwrap_or(true);

        if needs_update {
            // The server content is different from what's in LocalStorage.
            // Overwrite LocalStorage so the user sees fresh content from the server.
            match storage.save(filename, content.as_bytes()) {
                Ok(_) => {
                    storage.save_server_hash(filename, &server_hash);
                    tracing::info!(
                        "Updated {} from server ({} bytes, hash {})",
                        filename,
                        content.len(),
                        &server_hash[..8]
                    );
                    loaded_count += 1;
                }
                Err(e) => tracing::warn!("Failed to save {}: {}", filename, e),
            }
        } else {
            // Content is already identical.  Update/save the server hash sentinel
            // just to be sure this file is marked as "server tracked".
            storage.save_server_hash(filename, &server_hash);
            tracing::debug!("Skipping {}: already matches server", filename);
        }
    }

    // Cleanup: Remove any files that were previously tracked as "from server" 
    // but are no longer in the current manifest.
    let tracked_keys = storage.get_all_server_tracked_keys();
    for key in tracked_keys {
        if !manifest_filenames.contains(&key) {
            tracing::info!("Removing stale file no longer in manifest: {}", key);
            let _ = storage.delete(&key);
            storage.delete_server_hash(&key);
        }
    }

    tracing::info!("Preloaded {} DOT files into LocalStorage", loaded_count);
    Ok(loaded_count)
}

/// Fetch JSON array from a URL using web-sys fetch API
async fn fetch_json(url: &str) -> Result<Vec<String>, String> {
    let window = window().ok_or("No window object")?;

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "Response is not a Response object")?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let json = JsFuture::from(
        resp.json()
            .map_err(|e| format!("Failed to get json: {:?}", e))?,
    )
    .await
    .map_err(|e| format!("Failed to parse JSON: {:?}", e))?;

    serde_wasm_bindgen::from_value(json).map_err(|e| format!("Failed to deserialize: {:?}", e))
}

// /// Fetch binary data from a URL using web-sys fetch API
// async fn fetch_binary(url: &str) -> Result<Vec<u8>, String> {
//     let window = window().ok_or("No window object")?;
//
//     let opts = RequestInit::new();
//     opts.set_method("GET");
//     opts.set_mode(RequestMode::Cors);
//
//     let request = Request::new_with_str_and_init(url, &opts)
//         .map_err(|e| format!("Failed to create request: {:?}", e))?;
//
//     let resp_value = JsFuture::from(window.fetch_with_request(&request))
//         .await
//         .map_err(|e| format!("Fetch failed: {:?}", e))?;
//
//     let resp: Response = resp_value
//         .dyn_into()
//         .map_err(|_| "Response is not a Response object")?;
//
//     if !resp.ok() {
//         return Err(format!("HTTP error: {}", resp.status()));
//     }
//
//     let array_buffer = JsFuture::from(
//         resp.array_buffer()
//             .map_err(|e| format!("Failed to get array buffer: {:?}", e))?,
//     )
//     .await
//     .map_err(|e| format!("Failed to read array buffer: {:?}", e))?;
//
//     let uint8_array = js_sys::Uint8Array::new(&array_buffer);
//     Ok(uint8_array.to_vec())
// }

// Fetch String data from a URL using web-sys fetch API
async fn fetch_string(url: &str) -> Result<String, String> {
    let window = window().ok_or("No window object")?;
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;
    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "Response is not a Response object")?;
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    let text = JsFuture::from(
        resp.text()
            .map_err(|e| format!("Failed to get text: {:?}", e))?,
    )
    .await
    .map_err(|e| format!("Failed to read text: {:?}", e))?;
    text.as_string()
        .ok_or("Failed to convert text to string".to_string())
}
