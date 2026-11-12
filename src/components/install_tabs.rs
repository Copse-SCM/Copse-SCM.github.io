use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

use super::CopyBlock;

const TABS: [(&str, &str); 3] = [
    ("homebrew", "Homebrew"),
    ("download", "Download"),
    ("source", "From source"),
];

/// The three ways to install, as tabs. Left and right arrow keys move
/// between tabs, as people expect from a tab list.
#[component]
pub fn InstallTabs() -> impl IntoView {
    let (active, set_active) = signal(0usize);

    let on_key = move |ev: KeyboardEvent| {
        let n = TABS.len();
        let next = match ev.key().as_str() {
            "ArrowRight" => (active.get() + 1) % n,
            "ArrowLeft" => (active.get() + n - 1) % n,
            "Home" => 0,
            "End" => n - 1,
            _ => return,
        };
        ev.prevent_default();
        set_active.set(next);
        if let Some(tab) = document().get_element_by_id(&format!("tab-{}", TABS[next].0)) {
            let _ = tab.unchecked_into::<web_sys::HtmlElement>().focus();
        }
    };

    let tabs = TABS
        .iter()
        .enumerate()
        .map(|(i, (id, label))| {
            let selected = move || active.get() == i;
            view! {
                <button
                    type="button"
                    role="tab"
                    id=format!("tab-{id}")
                    aria-controls=format!("panel-{id}")
                    aria-selected=move || selected().to_string()
                    tabindex=move || if selected() { "0" } else { "-1" }
                    on:click=move |_| set_active.set(i)
                >
                    {*label}
                </button>
            }
        })
        .collect_view();

    let panel = move |i: usize, body: AnyView| {
        let id = TABS[i].0;
        view! {
            <div
                class="tab-panel"
                role="tabpanel"
                id=format!("panel-{id}")
                aria-labelledby=format!("tab-{id}")
                hidden=move || active.get() != i
            >
                {body}
            </div>
        }
    };

    view! {
        <div class="install-tabs">
            <div class="tab-list" role="tablist" aria-label="Ways to install" on:keydown=on_key>
                {tabs}
            </div>
            {panel(0, view! {
                <p>"macOS and Linux. Upgrades come with " <code>"brew upgrade"</code> "."</p>
                <CopyBlock>"brew install copse-scm/copse/copse"</CopyBlock>
                <p class="note">"For the daily build of main: " <code>"copse-scm/copse/copse-nightly"</code> "."</p>
            }.into_any())}
            {panel(1, view! {
                <p>
                    "For macOS (Apple Silicon or Intel) or Linux (x86_64), from the "
                    <a href="/releases">"latest release"</a> "."
                </p>
                <CopyBlock>"tar xzf copse-*.tar.gz\nsudo mv copse-*/copse /usr/local/bin/"</CopyBlock>
            }.into_any())}
            {panel(2, view! {
                <p>"With " <a href="https://rustup.rs">"Rust"</a> " installed. Builds in a minute or two."</p>
                <CopyBlock>"cargo install --git https://github.com/Copse-SCM/copse"</CopyBlock>
            }.into_any())}
        </div>
    }
}
