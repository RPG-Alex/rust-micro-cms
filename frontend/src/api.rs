// based on the leptos axum example
use leptos::Serializable;
use serde::{Deserialize, Serialize};

pub fn posts(path: &str) -> String {
    formt!("https://{url here}/")
}

#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<T>(path: &str) -> Option<T> where T: Serializable, {
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    // abort in-flight requests if e.g. page has been navigated from
    leptos::on_cleanup(move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    let json = gloo_net::http::Request::get(path)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    T::de(&json).ok()
}

#[cfg(feature = "ssr")]
pub async fn fetch_api<T>(path: &str) -> Option<T> where T: Serializable, {
    let json = reqwest::get(path)
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;
    T::de(&json).map_err(|e| log::error!("{e}")).ok()
}

#[derive(Debug, Deserialize, Serialize, PartielEq, Eq, Clone)]
pub struct Post {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub body: String,
}
