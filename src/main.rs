use dioxus::prelude::*;
use serde_json::Value;
use std::{collections::VecDeque, fs};

mod har_parser;
use clap::Parser;
use har_parser::har_parser::{parse_har, Request};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the har file to read
    #[arg(short, long)]
    file: String,
}

fn App(cx: Scope) -> Element {
    let args: Args = Args::parse();
    let input = fs::read_to_string(args.file).unwrap();
    let v: Value = serde_json::from_str(&input).unwrap();
    let entries: VecDeque<Request> = parse_har(v);

    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
        h1 { "Har Viewer" }
        div { class: "p-4 mb-6 bg-white shadow rounded overflow-x-auto",
            table { class: "table-auto w-full",
            thead {
                tr { class: "text-xs text-gray-500 text-left",
                    th { class: "pl-6 pb-3 font-medium", "Method" }
                    th { class: "pl-6 pb-3 font-medium", "Url"}
                    th { class: "pl-6 pb-3 font-medium", "Status"}
                }
            }
            rsx! {
                tbody {
                        for entry in entries {
                            rsx!(
                                tr { class: "text-xs bg-gray-50",
                                    td { class: "py-5 px-6 font-medium", entry.method }
                                    td { class: "flex py-3 font-medium", entry.url }
                                    td { class: "inline-block py-1 px-2 text-white bg-green-500 rounded-full", entry.status }
                                    td {
                                        class: "inline-block py-1 px-2 text-white bg-black rounded-full",
                                        button { onclick: move |_| {}, "Info" }
                                    }
                                }
                            )
                        }
                    }
                }
            }
        }
    })
}

fn main() {
    dioxus_desktop::launch(App);
}
