#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct TabsProps<'a> {
    pub titles: &'a [&'a str],
    pub children: Element<'a>,
}

pub fn Tabs<'a>(cx: Scope<'a, TabsProps<'a>>) -> Element {
    let current_tab = use_state(&cx, || 0 as u32);

    let titles = cx.props.titles.iter().enumerate().map(|(i, title)| {
        let current = if *current_tab.get() == i as u32 {
            "page"
        } else {
            "false"
        };

        rsx! {
            button {
                aria_current: "{current}",
                key: "tab-{i}",
                class: "secondary",
                onclick: move |_| {
                    current_tab.set(i as u32);
                },
                "{title}"
            }
        }
    });

    // let child = cx.props.children[*current_tab.get() as usize];

    let child = match &cx.props.children {
        Some(VNode::Fragment(el)) => Some(&el.children[*current_tab.get() as usize]),
        child => {
            println!("{:?}", child);
            panic!("Tabs: children must be an array of VNodes");
        }
    };

    // let children = cx.props.children.iter().map(|child| {
    //     let child = Some(child);
    //     rsx! {
    //         div {
    //             "test"
    //             child
    //         }
    //     }
    // });

    cx.render(rsx! {
        div {
            class: "tabs",
            div {
                class: "tabs-header",
                titles
            }
            div {
                class: "tabs-body",
                child
            }
        }
    })
}
