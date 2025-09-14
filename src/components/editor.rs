use dioxus::prelude::*;
use dioxus_document as document;
use serde_json;

#[component]
pub fn MonacoEditor(initial_value: String) -> Element {
    let editor_text = use_signal(|| initial_value.clone());
    let mut editor_ready = use_signal(|| false);

    // Function to get current editor content
    let get_editor_content = move |_: Event<MouseData>| {
        println!("Button click detected");
        let mut editor_text_clone = editor_text.clone();
        spawn(async move {
            println!("Attempting to get editor content...");
            
            // First verify editor exists and is ready
            let check_js = r#"
                if (typeof window.monaco_editor === 'undefined') {
                    console.log('Editor not initialized');
                    return 'not_initialized';
                }
                if (!window.monaco_editor) {
                    console.log('Editor object is null');
                    return 'null';
                }
                if (typeof window.monaco_editor.getValue !== 'function') {
                    console.log('getValue not available');
                    return 'no_getValue';
                }
                console.log('Editor is ready');
                return 'ready';
            "#;
            
            match document::eval(check_js).await {
                Ok(status) => {
                    println!("Editor status check: {}", status);
                    if status.as_str() != Some("ready") {
                        println!("Editor not ready, status: {}", status);
                        return;
                    }
                }
                Err(e) => {
                    println!("Editor check failed: {}", e);
                    return;
                }
            }

            // Now try to get content
            let get_js = r#"
                try {
                    if (!window.monaco_editor) {
                        console.error('Editor missing in getValue');
                        return null;
                    }
                    console.log('Editor object:', window.monaco_editor);
                    const model = window.monaco_editor.getModel();
                    console.log('Editor model:', model);
                    const content = window.monaco_editor.getValue();
                    console.log('Raw content:', content);
                    return content || null;
                } catch (e) {
                    console.error('Error getting content:', e);
                    return String(e);
                }
            "#;
            
            match document::eval(get_js).await {
                Ok(content) => {
                    println!("Content result: {}", content);
                    if let Some(text) = content.as_str() {
                        println!("Got text: {}", text);
                        editor_text_clone.set(text.to_string());
                    }
                },
                Err(e) => println!("Get failed: {}", e)
            }
        });
    };

    // Cleanup when component is dropped
    let _cleanup = use_drop(move || {
        spawn(async move {
            let _ = document::eval(
                r#"
                try {
                    if (window.monaco_editor) {
                        window.monaco_editor.dispose();
                        window.monaco_editor = null;
                    }
                } catch (e) {
                    console.error('Dispose error', e);
                }
                "#,
            )
            .await;
        });
    });

    rsx! {
        div {
            // Editor container
            div {
                id: "monaco-editor-container",
                style: "height: 45vh; min-height: 300px; width: 100%; border: 1px solid #444; overflow: hidden; position: relative;",
                onmounted: move |_| {
                    let initial_value_json = serde_json::to_string(&initial_value).unwrap_or_else(|_| "\"\"".to_string());

                    spawn(async move {
                        let init_js = format!(r#"
                            console.log('Initializing Monaco Editor...');

                            (function initMonaco() {{
                                function createEditor() {{
                                    const container = document.getElementById('monaco-editor-container');
                                    if (!container) {{
                                        console.error('Container not found!');
                                        return;
                                    }}

                                    try {{
                                        window.monaco_editor = monaco.editor.create(container, {{
                                            value: {initial_value_json},
                                            language: 'javascript',
                                            theme: 'vs-dark',
                                            automaticLayout: true,
                                            minimap: {{ enabled: false }},
                                            scrollBeyondLastLine: false,
                                            fontSize: 14,
                                            wordWrap: 'on',
                                            lineNumbers: 'on',
                                            glyphMargin: false,
                                            folding: false,
                                            lineDecorationsWidth: 10,
                                            lineNumbersMinChars: 0
                                        }});

                                        console.log('Monaco Editor created successfully!');
                                        window.monaco_ready = true;
                                        
                                        // Set up change listener
                                        window.monaco_editor.onDidChangeModelContent(() => {{
                                            console.log('Editor content changed to:', window.monaco_editor.getValue());
                                        }});

                                        // Layout after a brief delay
                                        setTimeout(() => {{
                                            if (window.monaco_editor) {{
                                                window.monaco_editor.layout();
                                            }}
                                        }}, 100);
                                    }} catch (error) {{
                                        console.error('Error creating Monaco editor:', error);
                                    }}
                                }}

                                if (typeof monaco !== 'undefined' && monaco.editor) {{
                                    console.log('Monaco already loaded');
                                    createEditor();
                                    return;
                                }}

                                // Load Monaco from CDN
                                console.log('Loading Monaco from CDN...');
                                const cdnScript = document.createElement('script');
                                cdnScript.src = 'https://cdn.jsdelivr.net/npm/monaco-editor@0.44.0/min/vs/loader.js';
                                cdnScript.onload = function() {{
                                    console.log('Monaco loader loaded');
                                    require.config({{
                                        paths: {{ 'vs': 'https://cdn.jsdelivr.net/npm/monaco-editor@0.44.0/min/vs' }}
                                    }});
                                    require(['vs/editor/editor.main'], function() {{
                                        console.log('Monaco main loaded');
                                        createEditor();
                                    }});
                                }};
                                cdnScript.onerror = function() {{
                                    console.error('Failed to load Monaco from CDN');
                                }};
                                document.head.appendChild(cdnScript);
                            }})();
                        "#);

                        if let Err(e) = document::eval(&init_js).await {
                            println!("Failed to init Monaco: {:?}", e);
                        }

                        // More thorough readiness check
                        spawn(async move {
                            for i in 0..50 {
                                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                                let check_js = r#"
                                    if (typeof window.monaco_editor === 'undefined') return 'not_initialized';
                                    if (!window.monaco_editor) return 'null';
                                    if (typeof window.monaco_editor.getValue !== 'function') return 'not_ready';
                                    return 'ready';
                                "#;
                                match document::eval(check_js).await {
                                    Ok(status) if status.as_str() == Some("ready") => {
                                        println!("Monaco editor fully ready after {} attempts", i + 1);
                                        editor_ready.set(true);
                                        break;
                                    }
                                    Ok(status) => println!("Editor not ready yet: {}", status),
                                    Err(e) => println!("Check failed: {}", e)
                                }
                            }
                        });
                    });
                }
            }

            // Controls
            div {
                style: "margin-top: 10px; display: flex; gap: 10px; flex-wrap: wrap;",
                
                button {
                    disabled: !editor_ready(),
                    onclick: get_editor_content,
                    style: "padding: 8px 16px; background-color: #0078d4; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    "Get Current Code"
                }

                button {
                    disabled: !editor_ready(),
                    onclick: move |_| {
                        println!("🔄 Set content button clicked");
                        let mut editor_text_clone = editor_text.clone();
                        spawn(async move {
                            println!("📝 Setting new content...");
                            let new_content = "console.log('New content from Rust!');";
                            let js = format!(r#"
                                if (window.monaco_editor) {{
                                    console.log('Setting new content...');
                                    window.monaco_editor.setValue("{}");
                                    console.log('Content set successfully');
                                    true
                                }} else {{
                                    console.warn('Editor not found');
                                    false
                                }}"#, new_content);
                            if let Ok(_) = document::eval(&js).await {
                                editor_text_clone.set(new_content.to_string());
                                println!("Set new content in editor");
                            }
                        });
                    },
                    style: "padding: 8px 16px; background-color: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    "Set Test Content"
                }

                button {
                    onclick: move |_| {
                        println!("Current stored content: {}", editor_text());
                    },
                    style: "padding: 8px 16px; background-color: #6f42c1; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    "Print Stored Content"
                }
            }

            // Live Content section
            div {
                style: "margin-top: 20px; border: 1px solid #444; padding: 10px; background-color: #333; border-radius: 4px;",
                h3 { 
                    style: "margin: 0 0 10px 0; color: #fff;",
                    "Stored Content (Length: {editor_text().len()} chars)" 
                }
                if editor_ready() {
                    pre { 
                        style: "margin: 0; color: #ccc; white-space: pre-wrap; font-family: 'Courier New', monospace; font-size: 12px; max-height: 200px; overflow-y: auto;",
                        "{editor_text()}" 
                    }
                } else {
                    div {
                        style: "color: #888; font-style: italic;",
                        "Monaco Editor loading..."
                    }
                }
            }

            // Debug section
            div {
                style: "margin-top: 10px; padding: 10px; background-color: #222; border-radius: 4px; font-size: 12px;",
                p { 
                    style: "margin: 0; color: #888;",
                    "💡 Click 'Get Current Code' to extract content from Monaco editor"
                }
                p { 
                    style: "margin: 5px 0 0 0; color: #888;",
                    "🔧 Use 'Print Stored Content' to see what's in the Rust signal"
                }
            }
        }
    }
}