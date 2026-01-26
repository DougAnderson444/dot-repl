use dioxus::logger::tracing;
use dioxus::prelude::*;
use dot_repl_ui::PlatformStorage;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, RequestMode, Response};

/// Pre-populate LocalStorage with DOT files from static assets
/// Uses Dioxus asset! macro to get the correct folder path with hash
/// Fetches manifest.json to discover available files
pub async fn preload_dot_files(
    storage: &impl PlatformStorage,
    dots_folder: &str,
) -> Result<usize, String> {
    tracing::info!("Fetching DOT files manifest from {}...", dots_folder);

    // Fetch the manifest.json to discover available files
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
        // Skip if already exists in storage (user may have edited)
        if storage.exists(&filename) {
            tracing::debug!("Skipping {}: already exists in storage", filename);
            continue;
        }

        // Fetch from /assets/dots/ directory
        let url = format!("{}/{}", dots_folder, filename);

        tracing::info!("Fetching DOT url {}", url);

        match fetch_string(&url).await {
            Ok(content) => {
                // Check if we should update: either doesn't exist or content differs
                let should_update = if storage.exists(&filename) {
                    // Load existing and compare
                    match storage.load(&filename) {
                        Ok(existing) => existing != content.as_bytes(),
                        Err(_) => true, // Error reading, update it
                    }
                } else {
                    true // Doesn't exist, load it
                };

                if should_update {
                    if let Err(e) = storage.save(&filename, content.as_bytes()) {
                        tracing::warn!("Failed to save {}: {}", filename, e);
                    } else {
                        tracing::info!("Loaded {} ({} bytes)", filename, content.len());
                        loaded_count += 1;
                    }
                } else {
                    tracing::debug!("Skipping {}: unchanged in storage", filename);
                }
            }
            Err(e) => {
                tracing::warn!("Failed to fetch {}: {}", url, e);
            }
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
