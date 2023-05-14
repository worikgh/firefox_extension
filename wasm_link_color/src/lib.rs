use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    type NodeList;

    #[wasm_bindgen(method, structural, js_namespace = NodeList)]
    fn item(this: &NodeList, index: u32) -> Option<HtmlElement>;

    #[wasm_bindgen(method, getter, structural, js_namespace = NodeList)]
    fn length(this: &NodeList) -> u32;

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);

    #[wasm_bindgen(js_namespace = window)]
    fn document() -> web_sys::Document;
}

#[wasm_bindgen]
pub fn change_links_color_to_green() {
    set_links_color("green");
}

// Helper function
fn set_links_color(color: &str) {
    let document = document();
    let list = document
        .query_selector_all("a")
        .expect("cannot query selector");
    for i in 0..list.length() {
        if let Some(elem) = list.item(i) {
            let style = elem.style();
            style
                .set_property("color", color)
                .expect("cannot set style");
        }
    }
}
