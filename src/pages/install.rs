//! Carried over from the original install.html.

use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{CopyBlock, Downloads};
use crate::release;

#[component]
pub fn Install() -> impl IntoView {
    view! {
        <Title text="Install Copse"/>
        <Meta name="description" content="Install Copse with Homebrew, from a release download, or from source."/>
        <section class="page-head">
            <div class="wrap">
                <h1>
                    "Install Copse"
                </h1>
                <p>
                    "Copse runs on macOS (Apple Silicon or Intel) and Linux (x86_64). Pick whichever way suits you; they all give you the same "
                    <code>
                        "copse"
                    </code>
                    " command."
                </p>
            </div>
        </section>
        <section class="page-body">
            <div class="wrap">
                <div class="cards">
                    <div class="card">
                        <h3>
                            <svg class="icon small" viewBox="0 0 24 24">
                                <path d="M5 9h14l-1.5 11h-11z"/>
                                <path d="M8 9V6a4 4 0 0 1 8 0v3"/>
                            </svg>
                            "Homebrew"
                        </h3>
                        <p class="muted">
                            "The easiest way, on macOS or Linux. Upgrades come with "
                            <code>
                                "brew upgrade"
                            </code>
                            "."
                        </p>
                        <CopyBlock>
                            "brew install copse-scm/copse/copse"
                        </CopyBlock>
                    </div>
                    <div class="card">
                        <h3>
                            <svg class="icon small" viewBox="0 0 24 24">
                                <path d="M12 4v11"/>
                                <path d="M7 10l5 5 5-5"/>
                                <path d="M5 20h14"/>
                            </svg>
                            "Download"
                        </h3>
                        <p class="muted">
                            "A ready-made binary from the latest release. Unpack it and put "
                            <code>
                                "copse"
                            </code>
                            " on your "
                            <code>
                                "PATH"
                            </code>
                            "."
                        </p>
                        <CopyBlock>
                            "tar xzf copse-*.tar.gz\nsudo mv copse-*/copse /usr/local/bin/"
                        </CopyBlock>
                    </div>
                    <div class="card">
                        <h3>
                            <svg class="icon small" viewBox="0 0 24 24">
                                <path d="M8 7l-5 5 5 5"/>
                                <path d="M16 7l5 5-5 5"/>
                                <path d="M13.5 5l-3 14"/>
                            </svg>
                            "From source"
                        </h3>
                        <p class="muted">
                            "With "
                            <a href="https://rustup.rs">
                                "Rust"
                            </a>
                            " installed. Builds in a minute or two."
                        </p>
                        <CopyBlock>
                            "cargo install --git https://github.com/Copse-SCM/copse"
                        </CopyBlock>
                    </div>
                </div>
                <h2>"Downloads for " {release::VERSION}</h2>
                <Downloads/>
                <p class="muted">
                    "Other versions are on the "
                    <a href="/releases">
                        "releases page"
                    </a>
                    "."
                </p>
                <h2>
                    "Check the download"
                </h2>
                <p>
                    "Each release has a "
                    <code>
                        "SHA256SUMS"
                    </code>
                    " file. Put it next to the archive and run:"
                </p>
                <CopyBlock>
                    "shasum -a 256 -c SHA256SUMS --ignore-missing"
                </CopyBlock>
                <div class="note-box">
                    <p>
                        <strong>
                            "On a Mac,"
                        </strong>
                        " a file downloaded in a browser is quarantined, and Copse isn't signed with an Apple developer certificate yet, so macOS won't open it until you clear that: "
                        <code>
                            "xattr -d com.apple.quarantine /usr/local/bin/copse"
                        </code>
                        ". Homebrew and "
                        <code>
                            "curl"
                        </code>
                        " downloads aren't affected."
                    </p>
                </div>
                <h2>
                    "Check it works"
                </h2>
                <CopyBlock>
                    "copse version"
                </CopyBlock>
                <p>
                    "Then try the "
                    <a href="/guide">
                        "guide"
                    </a>
                    ", or run "
                    <code>
                        "copse"
                    </code>
                    " on its own for the interactive screen."
                </p>
                <h2>
                    "The nightly build"
                </h2>
                <p>
                    "A build of the latest "
                    <code>
                        "main"
                    </code>
                    " is published each day it changes, named like "
                    <code>
                        "2026.11.09-dev+abc1234"
                    </code>
                    " (the date and the commit). It has everything new, and may have rough edges."
                </p>
                <CopyBlock>
                    "brew install copse-scm/copse/copse-nightly"
                </CopyBlock>
                <p class="muted">
                    <code>
                        "copse"
                    </code>
                    " and "
                    <code>
                        "copse-nightly"
                    </code>
                    " both provide the "
                    <code>
                        "copse"
                    </code>
                    " command, so install one or the other. The nightly is also on the "
                    <a href="https://github.com/Copse-SCM/copse/releases/tag/nightly">
                        "nightly release page"
                    </a>
                    "."
                </p>
                <h2>
                    "Upgrading and removing"
                </h2>
                <div class="table-scroll">
                    <table>
                        <tr>
                            <th>
                                "Installed with"
                            </th>
                            <th>
                                "Upgrade"
                            </th>
                            <th>
                                "Remove"
                            </th>
                        </tr>
                        <tr>
                            <td>
                                "Homebrew"
                            </td>
                            <td>
                                <code>
                                    "brew upgrade copse"
                                </code>
                            </td>
                            <td>
                                <code>
                                    "brew uninstall copse"
                                </code>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Download"
                            </td>
                            <td>
                                "Download the new version and replace the file"
                            </td>
                            <td>
                                <code>
                                    "sudo rm /usr/local/bin/copse"
                                </code>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Cargo"
                            </td>
                            <td>
                                "Run the install command again"
                            </td>
                            <td>
                                <code>
                                    "cargo uninstall copse"
                                </code>
                            </td>
                        </tr>
                    </table>
                </div>
                <p class="muted">
                    "Removing Copse doesn't touch your repositories: each keeps its history in its own "
                    <code>
                        ".copse"
                    </code>
                    " folder."
                </p>
            </div>
        </section>
    }
}
