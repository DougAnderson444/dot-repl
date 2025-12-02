//! This module contains the component that looks up the data by it's key,
//! and displays the graph.
//!
//! We have a system which allows user to take starter dot files and cusotmize
//! them, which are saved to their storage system.
//!
//! So when we look up a key,the following stepsneed to happen in this order:
//! 1. Check if the key exists in storage, if so, load it.
//! 2. If not, fall back to a default, if it exists.
//! 3. If the default static key ()& data) doesn't exists, create
//!    the data in dynamic storage. This allows users to creates a link to a
//!    ew file in their system and build out their hyperlinks.
//!
//! When the desktop app is used, this should be able to save the data to
//! local disk, such that it can be
//! - published to the website
//! - published to a data network like IPFS or similar in the future.
use dioxus::prelude::*;

use crate::{components::GraphEditor, StorageProvider};

#[component]
pub fn GraphView<R>(route: R, key_path: String) -> Element
where
    R: Routable + Clone + PartialEq,
{
    let mut dot_input = use_signal(String::new);
    let storage = use_context::<StorageProvider>();

    let p = key_path.to_string();
    let decoded = url_escape::decode(&p).to_string();

    info!("GraphView rendering for key_path: {}", decoded);

    use_effect(move || {
        // triggers when key_path changes
        let _ = use_route::<R>();
        let dot = storage
            .load(&decoded)
            .map(|data| String::from_utf8_lossy(&data).to_string())
            .unwrap_or_else(|_| "digraph { file -> not_found; }".to_string());

        info!("Loaded DOT data: {}", dot);
        dot_input.set(dot.clone());
    });

    rsx! {
        GraphEditor { dot_input }
    }
}
