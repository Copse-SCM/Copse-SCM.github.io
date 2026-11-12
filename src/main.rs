mod app;
mod components;
mod pages;
mod release;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
    // The router draws the page a frame after mounting, so wait for it.
    leptos::prelude::request_animation_frame(scroll_to_hash);
}

/// The page is drawn after the browser has already looked for `#section` in
/// the address, so a link straight to /guide#undo needs a nudge on first load.
/// Links followed inside the site are handled by the router.
pub(crate) fn scroll_to_hash() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(hash) = window.location().hash() else {
        return;
    };
    let Some(id) = hash.strip_prefix('#').filter(|id| !id.is_empty()) else {
        return;
    };
    if let Some(el) = window.document().and_then(|d| d.get_element_by_id(id)) {
        el.scroll_into_view();
    }
}
