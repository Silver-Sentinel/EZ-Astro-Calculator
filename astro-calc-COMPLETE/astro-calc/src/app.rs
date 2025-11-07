use dioxus::prelude::*;
use crate::components::{NatalTab, SynastryTab, TransitsTab};

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    NatalChart,
    Synastry,
    Transits,
}

#[component]
pub fn App() -> Element {
    let mut active_tab = use_signal(|| Tab::NatalChart);

    rsx! {
        style {
            {include_str!("../dioxus-migration/styles.css")}
        }
        
        div {
            class: "app-container",
            
            // Header (matching your current gradient design)
            header {
                class: "app-header",
                h1 { "☀️ Astrological Chart Calculator" }
                p { class: "subtitle", "Fagan-Bradley Sidereal • Placidus Houses" }
            }

            // Tab Navigation (matching your current design)
            nav {
                class: "tab-nav",
                
                button {
                    class: if *active_tab.read() == Tab::NatalChart { "tab-button active" } else { "tab-button" },
                    onclick: move |_| active_tab.set(Tab::NatalChart),
                    "📊 Natal Chart"
                }
                
                button {
                    class: if *active_tab.read() == Tab::Synastry { "tab-button active" } else { "tab-button" },
                    onclick: move |_| active_tab.set(Tab::Synastry),
                    "🎭 Synastry"
                }
                
                button {
                    class: if *active_tab.read() == Tab::Transits { "tab-button active" } else { "tab-button" },
                    onclick: move |_| active_tab.set(Tab::Transits),
                    "🔄 Transits"
                }
            }

            // Main Content Area
            main {
                class: "main-content",
                
                match *active_tab.read() {
                    Tab::NatalChart => rsx! { NatalTab {} },
                    Tab::Synastry => rsx! { SynastryTab {} },
                    Tab::Transits => rsx! { TransitsTab {} },
                }
            }

            // Footer (matching your current design)
            footer {
                class: "app-footer",
                "Made by Aquarian Rising | "
                a {
                    href: "https://youtube.com",
                    target: "_blank",
                    "YouTube Channel"
                }
            }
        }
    }
}
