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
    let desktop_config = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_title("Monaco in Dioxus Sample")
                .with_inner_size(LogicalSize::new(800.0, 600.0)),
        )
        .with_menu(Some(main_menu));

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
