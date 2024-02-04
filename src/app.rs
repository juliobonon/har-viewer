use crate::components::request_tables::request_tables::RequestTables;
use crate::parsers::har_parser::har_parser::{parse_har, Request};
use clap::Parser;
use dioxus::prelude::*;
use serde_json::Value;
use std::{collections::VecDeque, fs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the har file to read
    #[arg(short, long)]
    file: String,
}

pub fn App(cx: Scope) -> Element {
    let args: Args = Args::parse();
    let input = fs::read_to_string(args.file).unwrap();
    let v: Value = serde_json::from_str(&input).unwrap();
    let entries: VecDeque<Request> = parse_har(v);
    let default_entries = entries.clone();

    let filtered_entries = use_state(cx, || entries.clone());
    let search_query = use_state(cx, || String::new());

    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
        div { class: "m-4",
            h1 { class: "text-3xl font-bold text-center mb-6", "Har Viewer" }
            h2 { class: "text-xl font-bold text-center mb-6", "A simple way to see traffic" }
            div { class: "p-4 mb-6 bg-white shadow rounded overflow-x-auto",
                input {
                    class: "w-full p-2 border border-gray-300 rounded",
                    placeholder: "Search",
                    value: search_query.get().as_str(),
                    oninput: move |event| {
                        let query = event.value.clone();
                        search_query.set(query);
                        let entries = entries.clone();
                        let query = search_query.get().clone();
                        let filtered = entries
                            .into_iter()
                            .filter(|entry| entry.url.contains(query.as_str()))
                            .collect::<VecDeque<Request>>();
                        filtered_entries.set(filtered);
                    },  
                    onreset: move |_| {
                        filtered_entries.set(default_entries.clone());
                    }
                }
            }
            div { class: "p-4 mb-6 bg-white shadow rounded overflow-x-auto",
                table { class: "table-auto w-full",
                    thead {
                        tr { class: "text-xs text-gray-500 text-left",
                            th { class: "pl-2 pb-1 font-medium", "Expand" }
                            th { class: "pl-6 pb-3 font-medium", "Method" }
                            th { class: "pl-6 pb-3 font-medium", "Url"}
                            th { class: "pl-6 pb-3 font-medium", "Status"}
                        }
                    }
                    tbody {
                        RequestTables { entries: filtered_entries.get().clone() }
                    }
                }
            }
        }
    })
}
