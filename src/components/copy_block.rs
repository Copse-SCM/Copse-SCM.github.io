use leptos::html::Code;
use leptos::prelude::*;
use std::time::Duration;

/// Puts `text` on the clipboard, then shows "Copied" through `set_copied`
/// for a moment.
pub fn copy_text(text: String, set_copied: WriteSignal<bool>) {
    let Some(window) = web_sys::window() else { return };
    let promise = window.navigator().clipboard().write_text(&text);
    leptos::task::spawn_local(async move {
        if wasm_bindgen_futures::JsFuture::from(promise).await.is_ok() {
            set_copied.set(true);
            set_timeout(move || set_copied.set(false), Duration::from_millis(1400));
        }
    });
}

/// A code block with a Copy button. Trailing `# comments` are left out of
/// what gets copied, so the commands paste straight into a terminal.
#[component]
pub fn CopyBlock(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let code = NodeRef::<Code>::new();
    let (copied, set_copied) = signal(false);

    let copy = move |_| {
        if let Some(el) = code.get() {
            copy_text(strip_comments(&el.inner_text()), set_copied);
        }
    };

    view! {
        <pre class=format!("copyable {class}")>
            <code node_ref=code>{children()}</code>
            <button class="copy" type="button" on:click=copy>
                {move || if copied.get() { "Copied" } else { "Copy" }}
            </button>
        </pre>
    }
}

/// One command on a single line, with a Copy button that is always showing.
#[component]
pub fn CopyLine(command: &'static str) -> impl IntoView {
    let (copied, set_copied) = signal(false);
    view! {
        <div class="copy-line">
            <code>{command}</code>
            <button
                class="copy-line-button"
                type="button"
                aria-label=move || if copied.get() { "Copied" } else { "Copy the command" }
                on:click=move |_| copy_text(command.to_string(), set_copied)
            >
                {move || if copied.get() { "Copied" } else { "Copy" }}
            </button>
        </div>
    }
}

/// Drops whitespace followed by `#` and the rest of the line, from each line.
fn strip_comments(text: &str) -> String {
    text.lines()
        .map(|line| {
            line.char_indices()
                .find(|&(i, c)| c == '#' && i > 0 && line[..i].ends_with(char::is_whitespace))
                .map(|(i, _)| line[..i].trim_end())
                .unwrap_or(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
