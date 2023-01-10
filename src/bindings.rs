use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/js/html2pdf.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn htmltopdf(element: &HtmlElement);
}
