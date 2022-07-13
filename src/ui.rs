#![allow(non_snake_case)]
use dioxus::prelude::*;
use zeropool_client::{relayer_info, fetch_transactions};


pub fn Main(cx: Scope) -> Element {
    let print_info = move |_| {
        cx.spawn(async move {
            let info = relayer_info("https://kovan.testnet.relayer.v2.zeropool.network/").await;
            println!("{:?}", info);
        });
    };

    let print_txs = move |_| {
        cx.spawn(async move {
            let txs = fetch_transactions("https://kovan.testnet.relayer.v2.zeropool.network/", 0, 10).await;
            println!("{:?}", txs);
        });
    };

    cx.render(rsx! {
        div {
            button {
                onclick: move |_| zeropool_client::test_prover(),
                "test"
            }
            button {
                onclick: print_info,
                "relayer info"
            }
            button {
                onclick: print_txs,
                "txs"
            }
        }
    })
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        style { [include_str!("./pico.classless.css")] }
        Main { }
    ))
}

