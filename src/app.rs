use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::{NavigateOptions, StaticSegment, path};

use crate::components::{Footer, Header};
use crate::pages::{Guide, Home, Install, NotFound, Releases, Security};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // "Skip to content" moves keyboard focus into the page itself, so the next
    // Tab goes to the first link in the content, not back through the header.
    let main = NodeRef::<leptos::html::Main>::new();
    let skip = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        if let Some(main) = main.get() {
            let _ = main.focus();
        }
    };

    view! {
        <Router>
            <a class="skip-link" href="#content" on:click=skip>"Skip to content"</a>
            <Header/>
            <main id="content" tabindex="-1" node_ref=main>
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/install") view=Install/>
                    <Route path=path!("/guide") view=Guide/>
                    <Route path=path!("/releases") view=Releases/>
                    <Route path=path!("/security") view=Security/>
                    // The old site's addresses, so existing links keep working.
                    <Route path=StaticSegment("index.html") view=|| view! { <MovedTo path="/"/> }/>
                    <Route path=StaticSegment("install.html") view=|| view! { <MovedTo path="/install"/> }/>
                    <Route path=StaticSegment("guide.html") view=|| view! { <MovedTo path="/guide"/> }/>
                    <Route path=StaticSegment("releases.html") view=|| view! { <MovedTo path="/releases"/> }/>
                    <Route path=StaticSegment("security.html") view=|| view! { <MovedTo path="/security"/> }/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}

/// Sends an old address (like /guide.html#undo) to its new one (/guide#undo).
/// It keeps any #section and ?query, and replaces the old address in the
/// browser's history, so Back doesn't land on it and bounce forward again.
#[component]
fn MovedTo(path: &'static str) -> impl IntoView {
    let location = use_location();
    // The router gives the query without its "?" (and the hash with its "#"),
    // so put back whichever mark is missing.
    let mark = |text: String, mark: char| {
        if text.is_empty() || text.starts_with(mark) {
            text
        } else {
            format!("{mark}{text}")
        }
    };
    let target = format!(
        "{path}{}{}",
        mark(location.search.get_untracked(), '?'),
        mark(location.hash.get_untracked(), '#')
    );
    let navigate = use_navigate();
    request_animation_frame(move || {
        navigate(
            &target,
            NavigateOptions {
                replace: true,
                ..Default::default()
            },
        );
        // Once the new page is drawn, go to its #section.
        request_animation_frame(|| request_animation_frame(crate::scroll_to_hash));
    });
}
