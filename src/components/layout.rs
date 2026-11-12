use leptos::prelude::*;
use leptos_router::components::A;

use super::CopyLine;
use crate::release;

pub const GITHUB: &str = "https://github.com/Copse-SCM/copse";

/// The bar at the top of every page. `A` marks the current page with
/// `aria-current="page"`, which the stylesheet underlines.
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="top">
            <div class="wrap top-row">
                <A href="/" attr:class="brand">
                    <img src="/images/logo.svg" alt="" width="34" height="34"/>
                    <span>"Copse"</span>
                </A>
                <nav>
                    <A href="/install">"Install"</A>
                    <A href="/guide">"Guide"</A>
                    <A href="/releases">"Releases"</A>
                    <A href="/security">"Security"</A>
                    <a href=GITHUB>"GitHub"</a>
                </nav>
            </div>
        </header>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="wrap foot-grid">
                <div class="foot-about">
                    <a href="/" class="foot-brand">
                        <img src="/images/logo.svg" alt="" width="24" height="24"/>
                        <span>"Copse"</span>
                    </a>
                    <p>"Fast version control that works with git."</p>
                    <p>
                        "Free software under the "
                        <a href="https://github.com/Copse-SCM/copse/blob/main/LICENSE">"Apache License 2.0"</a>
                        "."
                    </p>
                </div>
                <div class="foot-get">
                    <p class="foot-label">"Get Copse"</p>
                    <CopyLine command="brew install copse-scm/copse/copse"/>
                    <p>
                        "Latest release: " <a href="/releases">{release::VERSION}</a>
                        ", " {release::DATE} ". Or try the "
                        <a href="https://github.com/Copse-SCM/copse/releases/tag/nightly">"nightly build"</a> "."
                    </p>
                </div>
                <nav class="foot-links" aria-label="Site">
                    <p class="foot-label">"The site"</p>
                    <a href="/install">"Install"</a>
                    <a href="/guide">"Guide"</a>
                    <a href="/releases">"Releases"</a>
                    <a href="/security">"Security"</a>
                    <a href=GITHUB>"GitHub"</a>
                </nav>
            </div>
        </footer>
    }
}
