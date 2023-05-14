use wasm_bindgen::prelude::*;
use web_sys::Document;

#[wasm_bindgen]
extern "C" {
    // Create a binding to the `window.document` property
    #[wasm_bindgen(js_namespace = window)]
    fn document() -> Document;
}

#[wasm_bindgen]
pub fn modify_dom_element() {
    // Access the document object
    let document = document();

    // Get an element by its ID
    if let Some(element) = document.get_element_by_id("my-element") {
        // Change the inner HTML content of the element
        element.set_inner_html("Hello from Rust!");
    }
}
