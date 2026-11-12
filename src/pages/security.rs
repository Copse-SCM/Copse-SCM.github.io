//! Carried over from the original security.html.

use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{CopyBlock, Description};

#[component]
pub fn Security() -> impl IntoView {
    view! {
        <Title text="Copse security"/>
        <Description text="How Copse keeps your work and your machine safe, how to check what you download, and how to report a security problem."/>
        <section class="page-head">
            <div class="wrap">
                <h1>
                    "Security"
                </h1>
                <p>
                    "Copse treats every repository, server and remote it talks to as untrusted, and it's built so that nothing you've committed is ever lost by accident. Here's what that means, how to check what you download, and how to tell us about a problem."
                </p>
            </div>
        </section>
        <section class="page-body">
            <div class="wrap">
                <div class="note-box">
                    <p>
                        <strong>
                            "Found a security problem?"
                        </strong>
                        " Please don't open a public issue. Report it privately with the "
                        <strong>
                            "Report a vulnerability"
                        </strong>
                        " button on the repository's "
                        <a href="https://github.com/Copse-SCM/copse/security">
                            "Security tab"
                        </a>
                        ", so it can be fixed before it's public. You'll get a reply within a week."
                    </p>
                </div>
                <div class="cards">
                    <div class="card">
                        <h3>
                            <svg aria-hidden="true" class="icon small" viewBox="0 0 24 24">
                                <path d="M12 3l7 3v5c0 4.5-3 8-7 10-4-2-7-5.5-7-10V6z"/>
                                <path d="M9 12l2 2 4-4"/>
                            </svg>
                            "Everything is checked"
                        </h3>
                        <p class="muted">
                            "Every object Copse receives is hashed again before it's kept, and every pack's checksum is verified. Anything whose contents don't match its id is refused."
                        </p>
                    </div>
                    <div class="card">
                        <h3>
                            <svg aria-hidden="true" class="icon small" viewBox="0 0 24 24">
                                <rect x="4" y="10" width="16" height="10" rx="1.5"/>
                                <path d="M8 10V7a4 4 0 0 1 8 0v3"/>
                            </svg>
                            "Hostile repositories"
                        </h3>
                        <p class="muted">
                            "A crafted repository can't make a checkout write outside your project, into "
                            <code>
                                ".copse"
                            </code>
                            ", or through a symlink, and addresses can't smuggle options into ssh. The test suite attacks Copse with exactly these."
                        </p>
                    </div>
                    <div class="card">
                        <h3>
                            <svg aria-hidden="true" class="icon small" viewBox="0 0 24 24">
                                <path d="M9 14L4 9l5-5"/>
                                <path d="M4 9h10a6 6 0 0 1 0 12h-3"/>
                            </svg>
                            "Nothing lost"
                        </h3>
                        <p class="muted">
                            "Every move of a branch is recorded, so "
                            <code>
                                "copse undo"
                            </code>
                            " can put it back. Anything that would throw work away, such as "
                            <code>
                                "reset --hard"
                            </code>
                            " or "
                            <code>
                                "restore"
                            </code>
                            ", keeps a backup first."
                        </p>
                    </div>
                </div>
                <h2>
                    "What counts as a security problem"
                </h2>
                <ul>
                    <li>
                        "A repository, pack or server reply that makes Copse write outside the working folder, into "
                        <code>
                            ".copse"
                        </code>
                        ", or through a symlink."
                    </li>
                    <li>
                        "Objects accepted whose contents don't match their ids."
                    </li>
                    <li>
                        "Command or option injection through URLs, remote names, submodule addresses or file names."
                    </li>
                    <li>
                        "Credentials shown, logged or sent somewhere they shouldn't go."
                    </li>
                    <li>
                        "A crash or hang caused by data from a server or someone else's repository."
                    </li>
                </ul>
                <p>
                    "Helpful things to include in a report: what an attacker could do, the version ("
                    <code>
                        "copse version"
                    </code>
                    "), and steps or a repository that shows it."
                </p>
                <h2>
                    "Signing in"
                </h2>
                <p>
                    "Copse uses git's own credential helpers, so a login you've saved for git (for example in the macOS Keychain) works for Copse too, and Copse doesn't keep a password store of its own. Over SSH it uses your SSH key, as git does. It asks for a login only when the server needs one, and it never prints a password."
                </p>
                <p class="muted">
                    "A login written into an address ("
                    <code>
                        "https://user:token@host/…"
                    </code>
                    ") works, but it's kept in "
                    <code>
                        ".copse/remotes"
                    </code>
                    ", so a credential helper is the safer choice."
                </p>
                <h2>
                    "Signed commits and tags"
                </h2>
                <p>
                    "Sign with an SSH key or GPG, using the same settings as git. Signatures are stored the way git stores them, so git verifies Copse's, Copse verifies git's, and GitHub shows them as Verified."
                </p>
                <CopyBlock>
                    "copse config --global user.signingKey ~/.ssh/id_ed25519\ncopse config --global gpg.format ssh\ncopse commit -S -m \"Signed\"\ncopse verify-commit HEAD"
                </CopyBlock>
                <p>
                    <code>
                        "copse tag -s"
                    </code>
                    " makes a signed tag and "
                    <code>
                        "copse verify-tag"
                    </code>
                    " checks one. Set "
                    <code>
                        "gpg.ssh.allowedSignersFile"
                    </code>
                    " and verifying also checks that the key belongs to the commit's author."
                </p>
                <h2>
                    "Checking your copy of a repository"
                </h2>
                <CopyBlock>
                    "copse fsck"
                </CopyBlock>
                <p>
                    "Checks that every object is intact, and that everything your branches, tags, stashes and undo history need is there."
                </p>
                <h2>
                    "Checking what you download"
                </h2>
                <p>
                    "Each release comes with a "
                    <code>
                        "SHA256SUMS"
                    </code>
                    " file. Put it next to the archive and run:"
                </p>
                <CopyBlock>
                    "shasum -a 256 -c SHA256SUMS --ignore-missing"
                </CopyBlock>
                <p>
                    "Releases are built from the public source by GitHub Actions, where every change is also scanned by CodeQL and the dependencies are checked against known vulnerabilities ("
                    <code>
                        "cargo audit"
                    </code>
                    ")."
                </p>
                <p class="muted">
                    "Copse isn't signed with an Apple developer certificate yet, so macOS quarantines a copy downloaded in a browser; see "
                    <a href="/install">
                        "Install"
                    </a>
                    ". Homebrew and "
                    <code>
                        "curl"
                    </code>
                    " downloads aren't affected."
                </p>
                <h2>
                    "Supported versions"
                </h2>
                <p>
                    "Until 1.0, fixes go into the latest release only. The "
                    <a href="/releases">
                        "nightly build"
                    </a>
                    " gets them first."
                </p>
            </div>
        </section>
    }
}
