use custom_elements::{inject_style, CustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Node, Text};

struct MyWebComponent {
    label_node: Text,
}

impl MyWebComponent {
    fn new() -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let label_node = document.create_text_node("friend");
        Self { label_node }
    }

    fn view(&self) -> Node {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el = document.create_element("button").unwrap();
        el.append_child(&self.label_node).unwrap();

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
        inject_style(&this, "p { color: var(--wild-vanilla-color, green); }");
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        // define public attributes
        &["label"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        match name.as_str() { 
            "label" => {
                self.label_node
                    .set_data(&new_value.unwrap_or_else(|| "label".to_string()))
            },
            _ => {
                print!("Invalid attribute {} passed", name);
                log(format!("Invalid attribute {} passed", name).as_str());
            }
        }
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        // log("connected");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        // log("disconnected");
    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {
        // log("adopted");
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn register() -> Result<(), JsValue> {
    // define the Custom Element
    MyWebComponent::define("wild-button");

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}