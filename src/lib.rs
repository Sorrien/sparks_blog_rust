mod utils;

use std::str::FromStr;

use chrono::prelude::*;
//use html_node::{html, text, typed::elements::*, unsafe_text};
//use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/* #[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub extern "C" fn greet() {
    alert("Hello, {{project-name}}!");
} */

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
/*     let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    /*     let location = window.location().href().expect("window should have a url");
    window
        .alert_with_message(&location)
        .expect("window should alert!"); */

    // Manufacture the element we're gonna append
    let page_div = document.create_element("div")?;
    page_div.set_id("app");

    let location = window.location().pathname().expect("window should have a url");
    //window
    //    .alert_with_message(&location)
    //    .expect("window should alert!");

    if location.to_ascii_lowercase().ends_with("/blog") {
        page_div.set_inner_html(&get_blog_page_body().await);
    } else {
        page_div.set_inner_html(&get_index_page_body());
    }
    body.append_child(&page_div)?; */

/*     let nav_link_active = document.get_elements_by_class_name("nav-link active").get_with_index(0).unwrap();
    nav_link_active.set_class_name("navlink");
    nav_link_active.remove_attribute("aria-current").unwrap();

    let nav_links = document.get_elements_by_class_name("nav-link");
    for i in 0..nav_links.length() {
        let nav_link = nav_links.get_with_index(i).unwrap();
        let href = nav_link.get_attribute("href").unwrap();
        if location == href || (location == "/" && href == ""){
            nav_link.set_class_name("navlink active");
            nav_link.set_attribute("aria-current", "page").unwrap();
        }
    }  */

    Ok(())
}   

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}