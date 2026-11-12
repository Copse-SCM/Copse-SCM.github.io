//! The latest release. When a new one comes out, this is the only file to
//! change: the home page, the install page and the releases page all read it.

pub const VERSION: &str = "2026.11.0";
pub const DATE: &str = "November 8, 2026";

/// Which kind of computer a download is for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Target {
    MacArm,
    MacIntel,
    LinuxX86,
}

pub struct Download {
    pub target: Target,
    /// The Rust target name, as it appears in the file name.
    pub triple: &'static str,
    pub label: &'static str,
    pub size: &'static str,
}

pub const DOWNLOADS: &[Download] = &[
    Download {
        target: Target::MacArm,
        triple: "aarch64-apple-darwin",
        label: "macOS, Apple Silicon (M1 and later)",
        size: "3.7 MB",
    },
    Download {
        target: Target::MacIntel,
        triple: "x86_64-apple-darwin",
        label: "macOS, Intel",
        size: "4.1 MB",
    },
    Download {
        target: Target::LinuxX86,
        triple: "x86_64-unknown-linux-gnu",
        label: "Linux, x86_64",
        size: "4.3 MB",
    },
];

const RELEASES: &str = "https://github.com/Copse-SCM/copse/releases";

pub fn download_url(d: &Download) -> String {
    format!(
        "{RELEASES}/download/v{VERSION}/copse-{VERSION}-{}.tar.gz",
        d.triple
    )
}

pub fn checksums_url() -> String {
    format!("{RELEASES}/download/v{VERSION}/SHA256SUMS")
}

pub fn notes_url() -> String {
    format!("{RELEASES}/tag/v{VERSION}")
}
