use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{CopyBlock, CopyLine, InstallTabs, Terminal};
use crate::release;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Copse: fast version control that works with git"/>
        <Meta name="description" content="Copse is a fast version control system that works with GitHub, GitLab and any git server, with undo for everything and a built-in terminal interface."/>
        <Hero/>
        <Features/>
        <FromGit/>
        <History/>
        <Install/>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="wrap hero-row">
                <div class="hero-text">
                    <p class="kicker">"Version control, written in Rust"</p>
                    <h1>"Fast version control" <br/> "that works with git."</h1>
                    <p class="lede">
                        "Clone from GitHub, commit, branch, merge and push, and your teammates who use git \
                         won't notice a thing. Copse has no staging area, an undo for everything, and a \
                         terminal interface built in."
                    </p>
                    <CopyLine command="brew install copse-scm/copse/copse"/>
                    <p class="actions">
                        <a class="button" href="/install">"Install Copse"</a>
                        <a class="button quiet" href="/guide">"Read the guide"</a>
                    </p>
                    <p class="fine">
                        "Latest release: " <a href="/releases">{release::VERSION}</a>
                        " · macOS and Linux · Apache-2.0"
                    </p>
                </div>
                <Terminal/>
            </div>
        </section>
    }
}

#[component]
fn Features() -> impl IntoView {
    view! {
        <section class="features">
            <div class="wrap">
                <h2>"What's different"</h2>
                <div class="grid">
                    <article>
                        <svg class="icon" viewBox="0 0 24 24"><path d="M13 3L5 13h6l-1 8 8-10h-6z"/></svg>
                        <h3>"Faster than git"</h3>
                        <SpeedBars/>
                    </article>
                    <article>
                        <svg class="icon" viewBox="0 0 24 24">
                            <circle cx="12" cy="12" r="3"/><path d="M12 3v6"/><path d="M12 15v6"/>
                        </svg>
                        <h3>"No staging area"</h3>
                        <p>
                            <code>"commit"</code>
                            " takes everything, the paths you name, or the changes you pick one at a time with "
                            <code>"-p"</code> "."
                        </p>
                    </article>
                    <article>
                        <svg class="icon" viewBox="0 0 24 24">
                            <path d="M9 14L4 9l5-5"/><path d="M4 9h10a6 6 0 0 1 0 12h-3"/>
                        </svg>
                        <h3>"Undo for everything"</h3>
                        <p>
                            <code>"copse undo"</code>
                            " reverses the last commit, merge, rebase, pull or reset. Anything that would \
                             throw work away keeps a backup first."
                        </p>
                    </article>
                    <article>
                        <svg class="icon" viewBox="0 0 24 24">
                            <circle cx="6" cy="6" r="2.2"/><circle cx="6" cy="18" r="2.2"/>
                            <circle cx="18" cy="8" r="2.2"/><path d="M6 8.2v7.6"/><path d="M18 10.2c0 4-6 3-11 6"/>
                        </svg>
                        <h3>"Works with git"</h3>
                        <p>
                            "GitHub, GitLab, and any git server over HTTPS or SSH, with signed commits, tags, \
                             submodules and LFS. Merges give the same results as git."
                        </p>
                    </article>
                </div>
            </div>
        </section>
    }
}

/// How many times faster than git, from the benchmark on a 50,000-file
/// repository: (what, at least, at most).
const SPEED: &[(&str, f32, f32)] = &[("status", 3.0, 4.0), ("commit", 2.5, 6.0), ("checkout", 4.0, 4.0)];
const SPEED_SCALE: f32 = 6.0;

fn times(x: f32) -> String {
    if x.fract() == 0.0 { format!("{x:.0}") } else { format!("{x}") }
}

/// The speed-ups as bars. Solid green runs to the slowest result; where there's
/// a range, a lighter green runs on to the fastest. The tick on each track is
/// git's own speed.
#[component]
fn SpeedBars() -> impl IntoView {
    let rows = SPEED
        .iter()
        .map(|&(what, lo, hi)| {
            let value = if lo == hi { format!("{}×", times(lo)) } else { format!("{}–{}×", times(lo), times(hi)) };
            let bar = format!("width: {}%", hi / SPEED_SCALE * 100.0);
            let solid = format!("width: {}%", lo / hi * 100.0);
            let tick = format!("left: {}%", 100.0 / SPEED_SCALE);
            view! {
                <div class="speed-row">
                    <code class="speed-what">{what}</code>
                    <span class="speed-track">
                        <span class="speed-git" style=tick></span>
                        <span class="speed-bar" style=bar><span class="speed-solid" style=solid></span></span>
                    </span>
                    <span class="speed-value">{value}</span>
                </div>
            }
        })
        .collect_view();

    view! {
        <figure class="speed" role="img"
            aria-label="Three to four times faster status, two and a half to six times faster commits, and four times faster checkouts than git, on a 50,000-file repository.">
            <div class="speed-rows" aria-hidden="true">{rows}</div>
            <figcaption aria-hidden="true">
                "Times faster than git (the tick) on a 50,000-file repository."
            </figcaption>
        </figure>
    }
}

/// Everyday jobs, as you'd do them in git and in Copse.
const FROM_GIT: &[(&str, &[&str], &[&str])] = &[
    (
        "Commit everything you changed",
        &["git add -A", "git commit -m \"Fix login\""],
        &["copse commit -m \"Fix login\""],
    ),
    (
        "Commit only some files",
        &["git add docs/ README.md", "git commit -m \"Update docs\""],
        &["copse commit -m \"Update docs\" docs/ README.md"],
    ),
    (
        "Pick changes one at a time",
        &["git add -p", "git commit -m \"Part one\""],
        &["copse commit -p -m \"Part one\""],
    ),
    (
        "Add your latest edits to the last commit",
        &["git add -A", "git commit --amend --no-edit"],
        &["copse commit --amend"],
    ),
    (
        "Take back a merge, rebase or pull",
        &["git reflog", "git reset --hard HEAD@{1}"],
        &["copse undo"],
    ),
];

#[component]
fn FromGit() -> impl IntoView {
    let commands = |lines: &'static [&'static str]| {
        lines.iter().map(|l| view! { <code>{*l}</code> }).collect_view()
    };
    let rows = FROM_GIT
        .iter()
        .map(|(task, git, copse)| {
            view! {
                <tr>
                    <th scope="row">{*task}</th>
                    <td class="git" data-label="git">{commands(git)}</td>
                    <td class="copse" data-label="Copse">{commands(copse)}</td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <section class="from-git">
            <div class="wrap">
                <h2>"Coming from git"</h2>
                <p class="section-lede">
                    "The commands will look familiar. Most everyday jobs just take fewer of them."
                </p>
                <table class="compare">
                    <thead>
                        <tr>
                            <th scope="col"><span class="visually-hidden">"To"</span></th>
                            <th scope="col">"In git"</th>
                            <th scope="col">"In Copse"</th>
                        </tr>
                    </thead>
                    <tbody>{rows}</tbody>
                </table>
                <p class="note">
                    "There's more on how the two differ in the " <a href="/guide">"guide"</a> "."
                </p>
            </div>
        </section>
    }
}

/// How the commit graph is shown: drawn in once it scrolls into view, or
/// all at once for people who've asked for less motion.
#[derive(Clone, Copy, PartialEq)]
enum Drawing {
    Still,
    Waiting,
    Drawn,
}

fn wants_less_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}

#[component]
fn History() -> impl IntoView {
    let figure = NodeRef::<leptos::html::Figure>::new();
    let start = if wants_less_motion() { Drawing::Still } else { Drawing::Waiting };
    let (drawing, set_drawing) = signal(start);

    // The drawing starts at the bottom (the first commit), so wait until the
    // whole graph is on screen, or until its top is well up the screen on a
    // window too short to show all of it at once.
    let check = move || {
        if drawing.get_untracked() != Drawing::Waiting {
            return;
        }
        let Some(el) = figure.get_untracked() else { return };
        let height = window().inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);
        let rect = el.get_bounding_client_rect();
        if rect.bottom() <= height || rect.top() <= height * 0.3 {
            set_drawing.set(Drawing::Drawn);
        }
    };
    request_animation_frame(check);
    let scroll = window_event_listener(leptos::ev::scroll, move |_| check());
    on_cleanup(move || scroll.remove());

    // Each mark's delay, in seconds, so they appear as the lines reach them.
    let d = |secs: f32| format!("--d: {secs}s");

    view! {
        <section class="history">
            <div class="wrap history-row">
                <figure
                    class="graph"
                    class:waiting=move || drawing.get() == Drawing::Waiting
                    class:drawn=move || drawing.get() == Drawing::Drawn
                    node_ref=figure
                    aria-label="A commit graph: a main line with a feature branch that is merged back in"
                >
                    <svg viewBox="0 0 260 300" role="img">
                        <path class="line main" pathLength="1" style=d(0.0) d="M 70 270 L 70 30"/>
                        <path class="line branch" pathLength="1" style=d(0.35) d="M 70 220 C 70 195, 170 200, 170 175 L 170 115 C 170 90, 70 95, 70 70"/>
                        <circle class="dot" style=d(0.05) cx="70" cy="270" r="9"/>
                        <circle class="dot" style=d(0.25) cx="70" cy="220" r="9"/>
                        <circle class="dot soft" style=d(0.6) cx="170" cy="175" r="9"/>
                        <circle class="dot soft" style=d(0.8) cx="170" cy="115" r="9"/>
                        <circle class="dot" style=d(0.5) cx="70" cy="145" r="9"/>
                        <circle class="dot ring" style=d(1.1) cx="70" cy="70" r="11"/>
                        <circle class="dot" style=d(1.25) cx="70" cy="30" r="9"/>
                        <text style=d(0.1) x="95" y="274">"first commit"</text>
                        <text style=d(0.55) x="95" y="149">"fix login"</text>
                        <text style=d(0.85) x="192" y="179">"feature"</text>
                        <text style=d(1.15) x="95" y="74">"merge feature"</text>
                        <text style=d(1.3) x="95" y="34">"main"</text>
                    </svg>
                </figure>
                <div class="history-text">
                    <h2>"Every step can be taken back"</h2>
                    <p>
                        "Copse keeps a record of every move your branch makes: each commit, merge, rebase, \
                         pull and reset. " <code>"copse undo"</code>
                        " reverses the last one, and running it again redoes it. " <code>"copse reflog"</code>
                        " lists them all, and the Undo screen can go back to any of them."
                    </p>
                    <p>
                        "Run " <code>"copse"</code>
                        " on its own for a screen with your changes, the commit graph, your branches, and \
                         that undo timeline. Pick single lines to commit, and pull or push without leaving it."
                    </p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Install() -> impl IntoView {
    view! {
        <section class="install" id="install">
            <div class="wrap">
                <h2>"Install"</h2>
                <InstallTabs/>
                <p class="note">
                    <a href="/install">"More about installing"</a>
                    ": checking downloads, the nightly build, upgrading."
                </p>
                <h3 class="tour-title">"A quick tour"</h3>
                <CopyBlock class="tour">
                    "copse clone https://github.com/you/project.git\n"
                    "cd project\n"
                    "copse status                  " <span class="c">"# what changed, and where you stand against origin"</span> "\n"
                    "copse commit -m \"Fix login\"   " <span class="c">"# no staging step"</span> "\n"
                    "copse push\n"
                    "copse undo                    " <span class="c">"# changed your mind? undo the last move"</span>
                </CopyBlock>
            </div>
        </section>
    }
}
