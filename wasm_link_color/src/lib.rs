use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
// use wasm_bindgen::prelude::*;
// use web_sys::{CssStyleDeclaration, Element, HtmlElement, Window};

// #[wasm_bindgen]
// extern "C" {
//     type NodeList;

//     #[wasm_bindgen(method, structural, js_namespace = NodeList)]
//     fn item(this: &NodeList, index: u32) -> Option<HtmlElement>;

//     #[wasm_bindgen(method, getter, structural, js_namespace = NodeList)]
//     fn length(this: &NodeList) -> u32;

//     #[wasm_bindgen(js_namespace = console)]
//     fn log(a: &str);

//     #[wasm_bindgen(js_namespace = window)]
//     fn window() -> Window;
// }

// #[wasm_bindgen]
// pub fn change_links_color_to_green() {
//     set_links_color("green");
// }

// // Helper function
// fn set_links_color(color: &str) {
//     let window = window();
//     let document = window.document().expect("cannot get document");
//     let list = document
//         .query_selector_all("a")
//         .expect("cannot query selector");
//     for i in 0..list.length() {
//         if let Some(elem) = list.item(i) {
//             let style = elem.style();
//             style
//                 .set_property("color", color)
//                 .expect("cannot set style");
//         }
//     }
// }
