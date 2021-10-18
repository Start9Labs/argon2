mod utils;

use js_sys::Error;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn verify(encoded: &str, password: &str) -> Result<(), JsValue> {
    if argon2::verify_encoded(encoded, password.as_bytes())
        .map_err(|e| Error::new(&e.to_string()))
        .map_err(JsValue::from)?
    {
        Ok(())
    } else {
        Err(JsValue::from(Error::new("invalid password")))
    }
}
