use wasm_bindgen::{JsCast, JsValue};

mod body;
mod client;
/// TODO
#[cfg(feature = "multipart")]
pub mod multipart;
mod request;
mod response;

pub use self::body::Body;
pub use self::client::{Client, ClientBuilder};
pub use self::request::{Request, RequestBuilder};
pub use self::response::Response;

async fn promise<T>(promise: js_sys::Promise) -> Result<(T, JsValue), crate::error::BoxError>
where
    T: JsCast,
{
    use wasm_bindgen_futures::JsFuture;

    let js_val = JsFuture::from(promise).await.map_err(crate::error::wasm)?;

    js_val
        .clone()
        .dyn_into::<T>()
        .map(|t| (t, js_val))
        .map_err(|_js_val| "promise resolved to unexpected type".into())
}
