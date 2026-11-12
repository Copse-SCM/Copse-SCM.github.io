use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="Page not found · Copse"/>
        <section class="page-head">
            <div class="wrap">
                <h1>"Page not found"</h1>
                <p>
                    "There's nothing at this address. Try the "
                    <a href="/">"home page"</a>
                    " or the "
                    <a href="/guide">"guide"</a>
                    "."
                </p>
            </div>
        </section>
    }
}
