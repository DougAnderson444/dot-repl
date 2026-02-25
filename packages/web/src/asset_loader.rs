use crate::storage::{fnv1a_hex, WebStorage};
use dioxus::logger::tracing;
use dot_repl_ui::PlatformStorage as _;
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
/// 2. **Stale cached files are refreshed on the next page load** – when a
///    new deploy ships updated `.dot` files we must overwrite the copy in
///    `LocalStorage` so the user sees fresh content after a reload / revisit.
///
/// We achieve this by storing a *server content hash* alongside every file
/// (under `{filename}\0__server_hash`).  On each cold load we:
///
/// - Always fetch the current server content.
/// - Compare its hash to the previously stored server hash.
/// - If the hashes differ ⟹ the server has a new version: overwrite storage
///   and update the server-hash sentinel.
/// - If the hashes match ⟹ nothing changed server-side: keep whatever the
///   user has stored (they may have edited it) and leave it alone.
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

    for filename in filenames {
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
        let stored_server_hash = storage.load_server_hash(&filename);

        let server_updated = stored_server_hash
            .as_deref()
            .map(|h| h != server_hash)
            // No sentinel yet means this is the very first load — always store.
            .unwrap_or(true);

        if server_updated {
            // The server has a new (or brand-new) version of this file.
            // Overwrite LocalStorage so the user sees fresh content on this
            // and every subsequent cold load — until the server ships another
            // change.  In-session edits live in the running signal and are
            // unaffected.
            match storage.save(&filename, content.as_bytes()) {
                Ok(_) => {
                    storage.save_server_hash(&filename, &server_hash);
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
            // Server content is unchanged.  Leave LocalStorage alone so any
            // edits the user made in a previous session are preserved.
            tracing::debug!(
                "Skipping {}: server content unchanged (hash {})",
                filename,
                &server_hash[..8]
            );
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
