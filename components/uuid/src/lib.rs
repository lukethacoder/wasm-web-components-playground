use custom_elements::{inject_style, CustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Node, Text};
use uuid::Uuid;

struct MyWebComponent {
    uuid_node: Text,
}

impl MyWebComponent {
    fn new() -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let uuid_node = document.create_text_node(&Uuid::new_v4().to_string());

        Self { uuid_node }
    }

    fn view(&self) -> Node {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el: web_sys::Element = document.create_element("p").unwrap();
        let t1 = document.create_text_node("Unique UUID: ");
        el.append_child(&t1).unwrap();
        el.append_child(&self.uuid_node).unwrap();

        el.unchecked_into()
    }
}

impl Default for MyWebComponent {
    fn default() -> Self {
        Self::new()
    }
}



impl CustomElement for MyWebComponent {
    // fn shadow() -> bool {
    //     false
    // }

    fn inject_children(&mut self, this: &HtmlElement) {
        inject_style(&this, "p { color: var(--wild-uuid-color, red); }");
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        // define public attributes
        &[]
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        log("connected");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        log("disconnected");
    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {
        log("adopted");
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn register() -> Result<(), JsValue> {
    // define the Custom Element
    MyWebComponent::define("wild-uuid");

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}