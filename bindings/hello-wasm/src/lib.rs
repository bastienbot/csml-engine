mod utils;

// use csmlrustmanager::{
//     data::CsmlData, start_conversation, user_close_all_conversations, Client, CsmlResult,
//     ErrorInfo, Warnings,
// };


use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn close_conversations() -> bool {
    // let json_client = cx.argument::<JsValue>(0)?;

    // match user_close_all_conversations(neon_serde::from_value(&mut cx, json_client)?) {
    //     Ok(_) => Ok(cx.boolean(true)),
    //     Err(err) => panic!(err),
    // }
    false
}
