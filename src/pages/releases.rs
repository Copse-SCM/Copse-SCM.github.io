//! Carried over from the original releases.html.

use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{CopyBlock, Downloads};
use crate::release;

#[component]
pub fn Releases() -> impl IntoView {
    view! {
        <Title text="Copse releases"/>
        <Meta name="description" content="Copse releases and nightly builds, with downloads and what changed."/>
        <section class="page-head">
            <div class="wrap">
                <h1>
                    "Releases"
                </h1>
                <p>
                    "Versions are dates: "
                    <code>
                        "2026.11.0"
                    </code>
                    " is the first release of November 2026, "
                    <code>
                        "2026.11.1"
                    </code>
                    " the next. Each release has builds for macOS (Apple Silicon and Intel) and Linux (x86_64), and a "
                    <code>
                        "SHA256SUMS"
                    </code>
                    " file to check them."
                </p>
            </div>
        </section>
        <section class="page-body">
            <div class="wrap">
                <article class="release">
                    <header>
                        <h2>
                            "Nightly"
                        </h2>
                        <span class="tag dev">
                            "Daily build of main"
                        </span>
                    </header>
                    <p class="muted">
                        "Built every morning (06:17 UTC) when "
                        <code>
                            "main"
                        </code>
                        " has changed, named like "
                        <code>
                            "2026.11.09-dev+abc1234"
                        </code>
                        ": the date and the commit. It has everything new, and may have rough edges."
                    </p>
                    <ul class="downloads">
                        <li>
                            <a href="https://github.com/Copse-SCM/copse/releases/tag/nightly">
                                <svg aria-hidden="true" class="icon small" viewBox="0 0 24 24">
                                    <path d="M12 4v11"/>
                                    <path d="M7 10l5 5 5-5"/>
                                    <path d="M5 20h14"/>
                                </svg>
                                "Downloads for today's nightly"
                            </a>
                        </li>
                    </ul>
                    <CopyBlock>
                        "brew install copse-scm/copse/copse-nightly"
                    </CopyBlock>
                </article>
                <article class="release">
                    <header>
                        <h2>{release::VERSION}</h2>
                        <span>
                            <span class="tag">
                                "Latest"
                            </span>
                            " "
                            <span class="muted">
                                {release::DATE}
                            </span>
                        </span>
                    </header>
                    <p>
                        "The first release."
                    </p>
                    <ul>
                        <li>
                            <strong>
                                "Everyday version control:"
                            </strong>
                            " commit without a staging area, pick changes or single lines with "
                            <code>
                                "commit -p"
                            </code>
                            ", branch, merge, rebase, cherry-pick, revert, stash, tag."
                        </li>
                        <li>
                            <strong>
                                "Undo for everything:"
                            </strong>
                            " "
                            <code>
                                "copse undo"
                            </code>
                            " reverses the last commit, merge, rebase, pull or reset."
                        </li>
                        <li>
                            <strong>
                                "Works with git servers:"
                            </strong>
                            " GitHub, GitLab and any git host over HTTPS or SSH, plus older cgit servers; shallow and partial clones; signed commits, submodules and Git LFS."
                        </li>
                        <li>
                            <strong>
                                "Merges like git's:"
                            </strong>
                            " renames and moved folders are followed, with "
                            <code>
                                "-X"
                            </code>
                            " options and octopus merges; 2,000 randomized stress rounds match git exactly."
                        </li>
                        <li>
                            <strong>
                                "A built-in terminal interface:"
                            </strong>
                            " run "
                            <code>
                                "copse"
                            </code>
                            " on its own."
                        </li>
                        <li>
                            <strong>
                                "Faster than git"
                            </strong>
                            " at status, commits and checkouts."
                        </li>
                    </ul>
                    <Downloads checksums=true/>
                    <p class="muted" style="margin-top:14px">
                        "Full notes on "
                        <a href=release::notes_url()>
                            "GitHub"
                        </a>
                        "."
                    </p>
                </article>
                <p class="muted">
                    "Every release and its notes are also on "
                    <a href="https://github.com/Copse-SCM/copse/releases">
                        "GitHub"
                    </a>
                    "."
                </p>
            </div>
        </section>
    }
}
