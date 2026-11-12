use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Redirect, Route, Router, Routes};
use leptos_router::{StaticSegment, path};

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
                    <Route path=StaticSegment("index.html") view=|| view! { <Redirect path="/"/> }/>
                    <Route path=StaticSegment("install.html") view=|| view! { <Redirect path="/install"/> }/>
                    <Route path=StaticSegment("guide.html") view=|| view! { <Redirect path="/guide"/> }/>
                    <Route path=StaticSegment("releases.html") view=|| view! { <Redirect path="/releases"/> }/>
                    <Route path=StaticSegment("security.html") view=|| view! { <Redirect path="/security"/> }/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
