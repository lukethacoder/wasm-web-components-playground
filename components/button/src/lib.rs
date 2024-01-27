use custom_elements::{inject_style, CustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Node, Text, Element};

struct MyWebComponent {
    label_node: Text,
    button_node: Element,
}

impl MyWebComponent {
    fn new() -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let label_node = document.create_text_node("friend");
        let button_node = document.create_element("button").unwrap();
        Self { label_node, button_node }
    }

    fn view(&self) -> Element {
        self.button_node.append_child(&self.label_node).unwrap();
        self.button_node.clone().unchecked_into()
    }

    // methods added here are NOT added to the WC when compiled
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

        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
            log("custom click event here from rust");          
        });

        // Result<(), JsValue>
        let _listener = self.button_node.add_event_listener_with_callback(
            "click",
            closure.as_ref().unchecked_ref()
        );
        closure.forget();
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