#![allow(non_snake_case)]
use dioxus::prelude::*;
use zeropool_client::{fetch_transactions, relayer_info};

use crate::tabs::Tabs;

pub fn Main(cx: Scope) -> Element {
    let print_info = move |_| {
        cx.spawn(async move {
            let info = relayer_info("https://kovan.testnet.relayer.v2.zeropool.network/").await;
            println!("{:?}", info);
        });
    };

    let print_txs = move |_| {
        cx.spawn(async move {
            let txs =
                fetch_transactions("https://kovan.testnet.relayer.v2.zeropool.network/", 0, 10)
                    .await;
            println!("{:?}", txs);
        });
    };

    cx.render(rsx! {
        main {
            class: "container",
            Tabs {
                titles: &["Main", "Tests"],
                div {
                    button {
                        "todo"
                    }
                }
                div {
                    button {
                        onclick: print_info,
                        "Print info"
                    }
                    button {
                        onclick: print_txs,
                        "Print txs"
                    }
                    button {
                        onclick: move |_| zeropool_client::test_prover(),
                        "test"
                    }
                }

            }

        }
    })
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        style { [include_str!("./pico.css")] }
        style { [include_str!("./custom.css")] }
        Main { }
    ))
}
