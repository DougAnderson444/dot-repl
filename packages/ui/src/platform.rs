//! Platform specific utilities

/// Spawn for tokio
#[cfg(not(target_arch = "wasm32"))]
pub async fn sleep(millis: std::time::Duration) {
    use tokio::time::sleep as tokio_sleep;

    tokio_sleep(millis).await;
}

/// Spawn for browser wasm32
#[cfg(target_arch = "wasm32")]
pub async fn sleep(millis: std::time::Duration) {
    use gloo_timers::future::TimeoutFuture;
    use wasm_bindgen_futures::spawn_local;

    TimeoutFuture::new(millis.as_millis() as u32).await;
}
