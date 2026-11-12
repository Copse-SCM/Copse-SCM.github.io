use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::release::{self, DOWNLOADS, Target};

// Works out what kind of computer the visitor is on. Chrome and Edge can say
// whether a Mac is Apple Silicon or Intel; Safari and Firefox can't, so for
// them a Mac stays just "mac".
#[wasm_bindgen(inline_js = r#"
export async function detectSystem() {
  const uad = navigator.userAgentData;
  const ua = navigator.userAgent.toLowerCase();
  const platform = ((uad && uad.platform) || navigator.platform || "").toLowerCase();
  let arch = "";
  if (uad && uad.getHighEntropyValues) {
    try { arch = (await uad.getHighEntropyValues(["architecture"])).architecture || ""; } catch (e) {}
  }
  // iPads ask for desktop sites as a Mac, but have a touch screen.
  if ((platform.includes("mac") || ua.includes("mac os x")) && navigator.maxTouchPoints < 2) {
    return arch === "arm" ? "mac-arm" : arch === "x86" ? "mac-intel" : "mac";
  }
  if ((platform.includes("linux") || ua.includes("linux")) && !ua.includes("android")) {
    return arch === "arm" || ua.includes("aarch64") ? "linux-arm" : "linux-x86";
  }
  return "other";
}
"#)]
extern "C" {
    #[wasm_bindgen(js_name = detectSystem)]
    async fn detect_system() -> JsValue;
}

/// The visitor's computer, as far as the browser will say.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum System {
    Known(Target),
    /// A Mac, but the browser won't say which chip.
    Mac,
    /// Linux on ARM: there's no build for it yet.
    LinuxArm,
    Other,
}

impl System {
    fn from_name(name: &str) -> Self {
        match name {
            "mac-arm" => Self::Known(Target::MacArm),
            "mac-intel" => Self::Known(Target::MacIntel),
            "linux-x86" => Self::Known(Target::LinuxX86),
            "mac" => Self::Mac,
            "linux-arm" => Self::LinuxArm,
            _ => Self::Other,
        }
    }

    fn matches(self, target: Target) -> bool {
        match self {
            Self::Known(t) => t == target,
            Self::Mac => matches!(target, Target::MacArm | Target::MacIntel),
            _ => false,
        }
    }
}

/// The latest release's downloads. The one for the visitor's computer is
/// marked; for a Mac whose chip we can't tell, both Mac builds are marked,
/// with a tip for finding out which.
#[component]
pub fn Downloads(#[prop(optional)] checksums: bool) -> impl IntoView {
    let (system, set_system) = signal(System::Other);
    leptos::task::spawn_local(async move {
        let name = detect_system().await.as_string().unwrap_or_default();
        set_system.set(System::from_name(&name));
    });

    let items = DOWNLOADS
        .iter()
        .map(|d| {
            let yours = move || system.get().matches(d.target);
            let icon = match d.target {
                Target::LinuxX86 => view! {
                    <svg class="icon small" viewBox="0 0 24 24">
                        <rect x="4" y="4" width="16" height="16" rx="2"/><path d="M8 9l3 3-3 3"/><path d="M13 15h3"/>
                    </svg>
                }
                .into_any(),
                _ => view! {
                    <svg class="icon small" viewBox="0 0 24 24">
                        <rect x="3" y="5" width="18" height="12" rx="1.5"/><path d="M8 21h8"/><path d="M12 17v4"/>
                    </svg>
                }
                .into_any(),
            };
            view! {
                <li class:yours=yours>
                    <a href=release::download_url(d)>
                        {icon}
                        {d.label}
                        <Show when=yours>
                            <span class="tag">
                                {move || if system.get() == System::Mac { "A Mac" } else { "Your computer" }}
                            </span>
                        </Show>
                        <span class="size">{d.size}</span>
                    </a>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul class="downloads">
            {items}
            {checksums.then(|| view! {
                <li>
                    <a href=release::checksums_url()>
                        <svg class="icon small" viewBox="0 0 24 24">
                            <path d="M12 3l7 3v5c0 4.5-3 8-7 10-4-2-7-5.5-7-10V6z"/><path d="M9 12l2 2 4-4"/>
                        </svg>
                        "SHA256SUMS"
                        <span class="size">"checksums"</span>
                    </a>
                </li>
            })}
        </ul>
        {move || match system.get() {
            System::Mac => Some(view! {
                <p class="download-hint">
                    "Not sure which Mac you have? Open the Apple menu and choose "
                    <strong>"About This Mac"</strong>
                    ". If it lists a " <strong>"Chip"</strong> ", it's Apple Silicon; if it lists a "
                    <strong>"Processor"</strong> ", it's Intel."
                </p>
            }.into_any()),
            System::LinuxArm => Some(view! {
                <p class="download-hint">
                    "There's no ready-made build for Linux on ARM yet. You can "
                    <a href="/install">"build it from source"</a> " instead."
                </p>
            }.into_any()),
            _ => None,
        }}
    }
}
