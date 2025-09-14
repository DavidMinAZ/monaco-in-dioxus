use dioxus::prelude::*;
use crate::components::MonacoEditor; // Import from the `components` module

#[component]
pub fn Hero() -> Element {
    let initial_code = "console.log('Hello from Monaco Editor!');".to_string();
    rsx! {
        div {
            id: "hero",
            MonacoEditor { initial_value: initial_code }
        }
    }
}
