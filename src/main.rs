use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use muda::{
    accelerator::{Accelerator, Code, Modifiers},
    Menu, MenuEvent, MenuItem, Submenu,
};
use std::process::exit;


mod content;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

// Define custom menu item IDs to identify the selected item later.
const NEW_MENU_ITEM_ID: &str = "new_menu_item";
const EXIT_MENU_ITEM_ID: &str = "exit_menu_item";
const OTHER_MENU_ITEM_ID_1: &str = "scary_menu_item_1";
const OTHER_MENU_ITEM_ID_2: &str = "scary_menu_item_2";

// The Dioxus application component.
#[component]
fn App() -> Element {
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1 { "Monaco Editor in Dioxus" }
        h2 { "with a custom menu using muda" }
        h3 { "(check devtools console and terminal when clicking buttons)" }
        crate::content::Hero {}
    }
}

fn main() {
    

    // Start Monaco asset server
    std::thread::spawn(|| {
    
    std::env::set_var("RUST_LOG", "warp=warn"); //tame warp logging noise

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Serve directly from ./assets/min without the "monaco" prefix
        let monaco = warp::fs::dir("./assets");
            //.with(warp::log("monaco-server")); // Optional logging
        
        println!("Starting Monaco server on http://localhost:3030");
        println!("Serving files from: ./assets/min");
        
        warp::serve(monaco)
            .run(([127, 0, 0, 1], 3030))
            .await;
        });
    });

    // Give server time to start
    std::thread::sleep(std::time::Duration::from_millis(100));

    // 1. Define the custom menu using muda structs.
    let new_menu_item = MenuItem::with_id(
        NEW_MENU_ITEM_ID,
        "New",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyN)),
    );

    let exit_menu_item = MenuItem::with_id(
        EXIT_MENU_ITEM_ID,
        "Exit",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyQ)),
    );
    let other_menu_item_1 = MenuItem::with_id(
        OTHER_MENU_ITEM_ID_1,
        "Scary Option 1",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::Digit1)),
    );
    let other_menu_item_2 = MenuItem::with_id(
        OTHER_MENU_ITEM_ID_2,
        "Scary Option 2",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::Digit2)),
    );

    let file_menu_items = [&new_menu_item as &dyn muda::IsMenuItem, &exit_menu_item];
    let file_menu = Submenu::with_items("&File", true, &file_menu_items)
        .expect("Failed to create 'File' submenu");
    let other_menu_items = [&other_menu_item_1 as &dyn muda::IsMenuItem, &other_menu_item_2];
    let other_menu = Submenu::with_items("&Other", true, &other_menu_items)
        .expect("Failed to create 'Other' submenu");

    let main_menu_items = [&file_menu as &dyn muda::IsMenuItem, &other_menu];
    let main_menu = Menu::with_items(&main_menu_items).expect("Failed to create main menu");
    
    // Pass the menu to the desktop configuration.
    // In your main() function, update the desktop_config:
    let desktop_config = Config::new()
        .with_window(
            WindowBuilder::new()
            .with_title("Monaco in Dioxus Sample")
            .with_inner_size(LogicalSize::new(800.0, 800.0))
            .with_min_inner_size(LogicalSize::new(600.0, 700.0))  // Minimum size
        )
        .with_menu(Some(main_menu))
        .with_custom_head(format!(r#"
            <link rel="stylesheet" href="http://localhost:3030/main.css" onload="console.log('CSS loaded successfully')" onerror="console.log('CSS failed to load')">
            <style>
                body {{
                    margin: 0;
                    padding: 20px;
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
                    background-color: #2d2d2d;
                    color: white;
                }}
                
                #monaco-editor-container {{
                    position: relative;
                    overflow: hidden;
                    border: 1px solid #444;
                }}
                
                .monaco-editor, .monaco-editor-background, .monaco-editor .margin {{
                    background-color: #1e1e1e !important;
                }}
            </style>
            <script>
                const script = document.createElement('script');
                script.src = 'http://localhost:3030/min/vs/loader.js';
                script.onload = function() {{
                    require.config({{ paths: {{ 'vs': 'http://localhost:3030/min/vs' }} }});
                    require(['vs/editor/editor.main'], function() {{
                        console.log('Local Monaco loaded!');
                        window.monaco_preloaded = true;
                    }});
                }};
                document.head.appendChild(script);
            </script>
        "#));

    // Use the `desktop()` builder and `launch(App)`.
    dioxus::LaunchBuilder::desktop()
        .with_cfg(desktop_config)
        .launch(App);

    // Listen for menu events on a separate thread using the receiver.
    let menu_channel = MenuEvent::receiver();
    std::thread::spawn(move || {
        for event in menu_channel {
            match event.id.as_ref() {
                NEW_MENU_ITEM_ID => {
                    println!("New menu item clicked!");
                }
                EXIT_MENU_ITEM_ID => {
                    println!("Exiting application...");
                    exit(0);
                }
                _ => {}
            }
        }
    });
}
