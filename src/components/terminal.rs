use leptos::prelude::*;

/// One line of the terminal session.
enum Line {
    /// A command typed at the prompt.
    Cmd(&'static str),
    /// Something Copse printed.
    Out(&'static str),
    /// A file in `status`: its one-letter state, then its path.
    File(&'static str, &'static str),
    Gap,
}
use Line::*;

/// Real output from Copse (a 2026.11 nightly), run in a fresh repository.
const SESSION: &[Line] = &[
    Cmd("copse status"),
    Out("On branch main"),
    File("M", "src/app.rs"),
    File("A", "src/login.rs"),
    Out("1 added, 1 modified, 0 deleted"),
    Gap,
    Cmd("copse commit -m \"Fix login\""),
    Out("[main 8b09c48a44] Fix login"),
    Out("  1 added, 1 modified, 0 deleted"),
    Gap,
    Cmd("copse undo"),
    Out("Undid `commit: Fix login` (just now)."),
    Out("  main is back at cfc918f64f First commit"),
    Out("  Run `copse undo` again to redo it."),
];

/// A terminal window showing Copse at work. It's a picture, so screen
/// readers get a one-line summary instead of every line.
#[component]
pub fn Terminal() -> impl IntoView {
    let lines = SESSION
        .iter()
        .enumerate()
        .map(|(i, line)| {
            // Each line fades in a moment after the one before.
            let style = format!("--i: {i}");
            match *line {
                Cmd(c) => view! {
                    <span class="t-line t-cmd" style=style>
                        <span class="t-prompt">"$ "</span>{c}
                    </span>
                }
                .into_any(),
                Out(o) => view! { <span class="t-line t-out" style=style>{o}</span> }.into_any(),
                File(state, path) => view! {
                    <span class="t-line t-out" style=style>
                        "  "<span class="t-state">{state}</span>" "{path}
                    </span>
                }
                .into_any(),
                Gap => view! { <span class="t-line t-gap" style=style>" "</span> }.into_any(),
            }
        })
        .collect_view();

    view! {
        <figure class="terminal" role="img"
            aria-label="A terminal: copse status lists two changed files, copse commit commits them, and copse undo takes the commit back.">
            <div class="terminal-bar" aria-hidden="true">
                <span class="terminal-dots"><i></i><i></i><i></i></span>
                <span class="terminal-title">"~/project"</span>
            </div>
            <pre class="terminal-body" aria-hidden="true"><code>{lines}</code></pre>
        </figure>
    }
}
