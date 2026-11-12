use leptos::prelude::*;

/// Sets the page's description. `index.html` already has a description tag
/// (so link previews, which don't run the app, still get one); this updates
/// that tag rather than adding a second, which would leave two that disagree.
#[component]
pub fn Description(text: &'static str) -> impl IntoView {
    let doc = document();
    let tag = doc
        .query_selector("meta[name=\"description\"]")
        .ok()
        .flatten()
        .or_else(|| {
            let tag = doc.create_element("meta").ok()?;
            tag.set_attribute("name", "description").ok()?;
            doc.head()?.append_child(&tag).ok()?;
            Some(tag)
        });
    if let Some(tag) = tag {
        let _ = tag.set_attribute("content", text);
    }
}
