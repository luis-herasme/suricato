use std::sync::atomic::{AtomicU64, Ordering};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Array, Uint32Array};
use web_sys::{HtmlImageElement, Response};

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn generate_id() -> u64 {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[inline]
pub fn to_bytes<T>(slice: &[T]) -> &[u8] {
    let len = slice.len() * std::mem::size_of::<T>();
    unsafe {
        return std::slice::from_raw_parts(slice.as_ptr() as *const u8, len);
    }
}

pub async fn fetch_image(url: &str) -> Result<HtmlImageElement, JsValue> {
    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_str(&url)).await?;

    let response: Response = response_value.dyn_into()?;
    let blob = JsFuture::from(response.blob()?).await?;
    let url = web_sys::Url::create_object_url_with_blob(&blob.dyn_into()?)?;

    let image = HtmlImageElement::new()?;
    image.set_src(&url);

    Ok(image)
}

pub async fn fetch_text(url: &str) -> Result<String, JsValue> {
    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_str(url)).await?;
    let response: Response = response_value.dyn_into()?;

    let text = JsFuture::from(response.text()?).await?;
    Ok(text.as_string().unwrap())
}

pub fn js_value_to_vec_u32(array: JsValue) -> Vec<u32> {
    let array = Uint32Array::new(&array);
    let mut output = vec![0; array.length() as usize];
    array.copy_to(&mut output[..]);
    output
}

pub fn js_array_to_vec_u32(array: JsValue) -> Vec<u32> {
    let array: Array = array.dyn_into().unwrap();
    array.iter().map(|v| v.as_f64().unwrap() as u32).collect()
}
