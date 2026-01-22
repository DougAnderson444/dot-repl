pub mod error;
pub use error::Error;

pub mod graphvism_wrapper;
pub mod storage;

use dioxus::prelude::*;
use dot_repl_ui::{GVizProvider, StorageProvider};
use graphvizm::Graphvizm;

#[component]
pub fn DesktopApp(path: String, children: Element) -> Element {
    let storage = storage::GitStorage::new(path).unwrap();
    let storage_provider = StorageProvider::new(storage.clone());

    // provide storage in context for all child elements
    use_context_provider(|| storage_provider);

    // signal that will be saved to the context as None, until GViz is loaded
    let gviz_signal = use_signal::<Option<GVizProvider>>(|| None);
    let mut gviz_signal = use_context_provider(|| gviz_signal);

    // Create the Graphvizm instance once
    use_hook(|| {
        if let Ok(gviz) = Graphvizm::new() {
            // set the signal
            gviz_signal.set(Some(GVizProvider::new(
                graphvism_wrapper::GraphvizmWrapper::from(gviz),
            )));
        }
    });

    rsx! {
        // Global app resources

        {children}

    }
}
