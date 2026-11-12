//! Carried over from the original guide.html.

use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{CopyBlock, Description};

#[component]
pub fn Guide() -> impl IntoView {
    view! {
        <Title text="Copse guide"/>
        <Description text="How to use Copse: committing, history, branches, merging, undo, remotes and GitHub, and the interactive screen."/>
        <section class="page-head">
            <div class="wrap">
                <h1>
                    "Guide"
                </h1>
                <p>
                    "Everything you need for everyday work with Copse. If you know git, most of it will look familiar; the differences are noted as they come up."
                </p>
            </div>
        </section>
        <section class="page-body">
            <div class="wrap guide-row">
                <Contents/>
                <div class="guide">
                    <section id="start">
                        <SectionHeading id="start" title="Getting started"/>
                        <p>
                            "Start a new repository in the current folder, or clone one from any git server or another Copse repository:"
                        </p>
                        <CopyBlock>
                            "copse init\ncopse clone https://github.com/you/project.git"
                        </CopyBlock>
                        <p>
                            "Copse keeps its history in a "
                            <code>
                                ".copse"
                            </code>
                            " folder. Your name and email come from "
                            <code>
                                "copse config user.name"
                            </code>
                            " and "
                            <code>
                                "user.email"
                            </code>
                            ", or from git's settings if you have them."
                        </p>
                    </section>
                    <section id="commit">
                        <SectionHeading id="commit" title="Committing"/>
                        <p>
                            "There's no staging area. "
                            <code>
                                "status"
                            </code>
                            " shows what changed, and "
                            <code>
                                "commit"
                            </code>
                            " takes all of it:"
                        </p>
                        <CopyBlock>
                            "copse status\ncopse commit -m \"Fix the login page\""
                        </CopyBlock>
                        <p>
                            "To commit only some files, name them; everything else stays uncommitted:"
                        </p>
                        <CopyBlock>
                            "copse commit -m \"Update the docs\" docs/ README.md"
                        </CopyBlock>
                        <p>
                            "Forgot something? "
                            <code>
                                "copse commit --amend"
                            </code>
                            " replaces the last commit with your current files, keeping its message unless you give a new one."
                        </p>
                    </section>
                    <section id="pick">
                        <SectionHeading id="pick" title="Picking changes"/>
                        <p>
                            <code>
                                "copse commit -p"
                            </code>
                            " goes through your changes one at a time and asks about each: "
                            <code>
                                "y"
                            </code>
                            " to commit it, "
                            <code>
                                "n"
                            </code>
                            " to leave it, "
                            <code>
                                "s"
                            </code>
                            " to split it into smaller pieces, "
                            <code>
                                "e"
                            </code>
                            " to edit it, "
                            <code>
                                "q"
                            </code>
                            " to stop. Your files aren't touched; whatever you don't pick stays as an uncommitted change."
                        </p>
                        <p>
                            "In the interactive screen, "
                            <code>
                                "enter"
                            </code>
                            " on a file does the same, and lets you pick single lines."
                        </p>
                    </section>
                    <section id="history">
                        <SectionHeading id="history" title="Looking at history"/>
                        <CopyBlock>
                            "copse log --oneline --graph     # the history, with branches drawn\ncopse show                      # the latest commit and what it changed\ncopse diff                      # your uncommitted changes\ncopse show HEAD~2:src/app.rs    # a file as it was two commits ago"
                        </CopyBlock>
                        <p>
                            <code>
                                "log"
                            </code>
                            " takes git's "
                            <code>
                                "--format"
                            </code>
                            " and "
                            <code>
                                "--pretty"
                            </code>
                            " options, and "
                            <code>
                                "copse blame"
                            </code>
                            " shows who last changed each line."
                        </p>
                    </section>
                    <section id="branches">
                        <SectionHeading id="branches" title="Branches and merging"/>
                        <CopyBlock>
                            "copse branch feature        # make a branch\ncopse checkout feature      # switch to it\ncopse checkout main\ncopse merge feature         # bring its work into main"
                        </CopyBlock>
                        <p>
                            "Merges follow renamed files and moved folders the way git does. If both sides changed the same lines, Copse marks the conflict in the file; fix it and run "
                            <code>
                                "copse commit"
                            </code>
                            ", or give up with "
                            <code>
                                "copse merge --abort"
                            </code>
                            "."
                        </p>
                        <p>
                            <code>
                                "copse branch -d feature"
                            </code>
                            " deletes a branch once its work is merged, and tells you how to bring it back."
                        </p>
                    </section>
                    <section id="rebase">
                        <SectionHeading id="rebase" title="Rebasing"/>
                        <p>
                            <code>
                                "copse rebase main"
                            </code>
                            " replays your branch's commits on top of "
                            <code>
                                "main"
                            </code>
                            ". With "
                            <code>
                                "-i"
                            </code>
                            " you choose what happens to each commit: reorder, reword, squash, edit or drop. After a conflict, fix it and run "
                            <code>
                                "copse rebase --continue"
                            </code>
                            "."
                        </p>
                        <p>
                            <code>
                                "copse cherry-pick"
                            </code>
                            " copies single commits onto the current branch, and "
                            <code>
                                "copse revert"
                            </code>
                            " adds a commit that undoes one."
                        </p>
                    </section>
                    <section id="undo">
                        <SectionHeading id="undo" title="Undo"/>
                        <p>
                            "Copse records every move your branch makes. "
                            <code>
                                "copse undo"
                            </code>
                            " reverses the last one, whether it was a commit, merge, rebase, pull or reset. Run it again to redo."
                        </p>
                        <CopyBlock>
                            "copse undo\ncopse reflog                # every recorded move, newest first"
                        </CopyBlock>
                        <p>
                            "Anything that would throw work away keeps a backup first. "
                            <code>
                                "copse restore file.txt"
                            </code>
                            " discards your changes to a file, and "
                            <code>
                                "copse reset --hard"
                            </code>
                            " moves the branch, but both stash your uncommitted work before they do."
                        </p>
                    </section>
                    <section id="stash">
                        <SectionHeading id="stash" title="Putting work aside"/>
                        <CopyBlock>
                            "copse stash                 # put your changes aside\ncopse stash list\ncopse stash pop             # bring them back"
                        </CopyBlock>
                        <p>
                            "A stash can be brought back after you've made new commits, or on another branch: it's a handy way to move unfinished work."
                        </p>
                    </section>
                    <section id="remotes">
                        <SectionHeading id="remotes" title="Remotes and GitHub"/>
                        <p>
                            "Copse works directly with GitHub, GitLab, Bitbucket and any git server, over HTTPS or SSH. To your teammates it looks like any other git client."
                        </p>
                        <CopyBlock>
                            "copse remote add origin git@github.com:you/project.git\ncopse push                  # send your work\ncopse pull                  # get what others pushed\ncopse pull --rebase         # or put your commits on top of theirs"
                        </CopyBlock>
                        <p>
                            "Signing in uses git's saved logins, or your SSH key. For GitHub over HTTPS, the password is a personal access token. Tags, signed commits, submodules and Git LFS all work, and so do shallow ("
                            <code>
                                "--depth"
                            </code>
                            ") and partial ("
                            <code>
                                "--filter=blob:none"
                            </code>
                            ") clones."
                        </p>
                    </section>
                    <section id="screen">
                        <SectionHeading id="screen" title="The interactive screen"/>
                        <p>
                            "Run "
                            <code>
                                "copse"
                            </code>
                            " on its own for a full-screen view with four tabs:"
                        </p>
                        <table>
                            <tr>
                                <th>
                                    "Screen"
                                </th>
                                <th>
                                    "What you can do"
                                </th>
                            </tr>
                            <tr>
                                <td>
                                    "1 Changes"
                                </td>
                                <td>
                                    "Pick files or single lines, commit, discard, stash"
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    "2 History"
                                </td>
                                <td>
                                    "Browse the commit graph; check out, revert, cherry-pick"
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    "3 Branches"
                                </td>
                                <td>
                                    "Switch, create, delete and merge branches"
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    "4 Undo"
                                </td>
                                <td>
                                    "Step back to any earlier point"
                                </td>
                            </tr>
                        </table>
                        <p>
                            <code>
                                "F"
                            </code>
                            ", "
                            <code>
                                "L"
                            </code>
                            " and "
                            <code>
                                "P"
                            </code>
                            " fetch, pull and push in the background, and "
                            <code>
                                "?"
                            </code>
                            " shows every key."
                        </p>
                    </section>
                    <section id="git">
                        <SectionHeading id="git" title="Working with git"/>
                        <p>
                            "Everything Copse pushes is ordinary git: commits keep their git ids, so history stays identical on the server. If you use an editor or tool that reads "
                            <code>
                                ".git"
                            </code>
                            " directly, "
                            <code>
                                "copse git-view on"
                            </code>
                            " keeps a read-only "
                            <code>
                                ".git"
                            </code>
                            " view up to date for it."
                        </p>
                    </section>
                    <section id="more">
                        <SectionHeading id="more" title="Everything else"/>
                        <p>
                            "Worktrees, sparse checkouts, hooks, aliases, bisect, plumbing commands, the storage format and how Copse is tested are all in the "
                            <a href="https://github.com/Copse-SCM/copse/blob/main/docs/guide.md">
                                "full reference"
                            </a>
                            ". "
                            <code>
                                "copse help"
                            </code>
                            " and "
                            <code>
                                "copse COMMAND --help"
                            </code>
                            " list every option."
                        </p>
                    </section>
                </div>
            </div>
        </section>
    }
}

/// The guide's sections, in order: (id, title).
const SECTIONS: &[(&str, &str)] = &[
    ("start", "Getting started"),
    ("commit", "Committing"),
    ("pick", "Picking changes"),
    ("history", "Looking at history"),
    ("branches", "Branches and merging"),
    ("rebase", "Rebasing"),
    ("undo", "Undo"),
    ("stash", "Putting work aside"),
    ("remotes", "Remotes and GitHub"),
    ("screen", "The interactive screen"),
    ("git", "Working with git"),
    ("more", "Everything else"),
];

/// How far down the window a heading must pass before its section counts as
/// the one being read.
const READING_LINE: f64 = 140.0;

/// Which section is being read: the last one whose heading has scrolled up
/// past the reading line, or the last one of all once the page is scrolled
/// to the bottom (short final sections never reach the line).
fn section_in_view() -> &'static str {
    let window = window();
    let doc = document();
    let scrolled = window.scroll_y().unwrap_or(0.0);
    let height = window
        .inner_height()
        .ok()
        .and_then(|h| h.as_f64())
        .unwrap_or(0.0);
    let page = doc
        .document_element()
        .map(|e| e.scroll_height() as f64)
        .unwrap_or(0.0);
    if scrolled > 0.0 && scrolled + height >= page - 2.0 {
        return SECTIONS[SECTIONS.len() - 1].0;
    }
    let mut current = SECTIONS[0].0;
    for (id, _) in SECTIONS {
        match doc.get_element_by_id(id) {
            Some(el) if el.get_bounding_client_rect().top() <= READING_LINE => current = id,
            _ => break,
        }
    }
    current
}

/// The contents list, with the section being read marked.
#[component]
fn Contents() -> impl IntoView {
    let (current, set_current) = signal(SECTIONS[0].0);
    let update = move || {
        let now = section_in_view();
        if current.get_untracked() != now {
            set_current.set(now);
        }
    };
    // Once the page is drawn, then on every scroll and resize. Both listeners
    // are removed when the guide is left.
    request_animation_frame(update);
    let scroll = window_event_listener(ev::scroll, move |_| update());
    let resize = window_event_listener(ev::resize, move |_| update());
    on_cleanup(move || {
        scroll.remove();
        resize.remove();
    });

    let links = SECTIONS
        .iter()
        .map(|&(id, title)| {
            let here = move || current.get() == id;
            view! {
                <a href=format!("#{id}") class:here=here aria-current=move || here().then_some("location")>
                    {title}
                </a>
            }
        })
        .collect_view();

    view! {
        <nav class="toc" aria-label="Contents">
            <p>"Contents"</p>
            {links}
        </nav>
    }
}

/// A section heading with a # link beside it, for sharing a link straight to
/// that section.
#[component]
fn SectionHeading(id: &'static str, title: &'static str) -> impl IntoView {
    view! {
        <h2>
            {title}
            <a class="anchor" href=format!("#{id}") aria-label=format!("Link to “{title}”")>"#"</a>
        </h2>
    }
}
