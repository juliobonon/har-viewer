pub mod request_tables {
    use crate::parsers::har_parser::har_parser::Request;
    use dioxus::prelude::*;
    use std::collections::VecDeque;

    #[derive(PartialEq, Props)]
    struct RequestTableProps {
        entry: Request,
    }

    #[derive(PartialEq, Props)]
    struct StatusCellProps {
        status: String,
    }

    pub fn StatusCell(cx: Scope<StatusCellProps>) -> Element {
        // this component should differ the background color based on the status
        let class_text = match cx.props.status.as_str() {
            "200" => "bg-green-500",
            "404" => "bg-yellow-500",
            "500" => "bg-red-500",
            _ => "bg-yellow-500",
        };
        render!(
            td {
                class: "py-4 px-4 py-2 px-2 text-white {class_text} rounded-full text-sm",
                style: "text-align: center;",
                cx.props.status.clone()
            }
        )
    }

    pub fn RequestTable(cx: Scope<RequestTableProps>) -> Element {
        let mut request: Request = cx.props.entry.clone();
        let expand = cx.use_hook(|| false);

        if request.url.len() > 100 {
            request.url.truncate(100);
            request.url.push_str("...");
        }

        cx.render(rsx! {
            tr { class: "text-xs bg-gray-50",
                td { class: "py-4 px-6 font-medium",
                    button {
                        class: "text-blue-500", style: "cursor: pointer;",
                        onclick: move |_| {
                            *expand = !(*expand);
                            cx.needs_update();
                        },
                        "Expand" 
                    }
                }
                td { class: "py-3 px-3 font-medium", request.method }
                td { class: "flex py-3 font-medium", request.url }
                StatusCell { status: request.status }
            }
            if *expand {
                rsx! {
                    div { class: "p-2 mb-2 bg-white shadow rounded overflow-x-auto", style: "max-width: 1000px;",
                        thead {
                            tr { class: "text-xs text-gray-500 text-left",
                                th { class: "pl-2 pb-1 font-medium", "Headers" }
                            }
                        }
                        rsx! {
                            for header in request.headers {
                                tr { class: "text-xs bg-gray-50",
                                    td { class: "py-2 px-4 font-medium", header }
                                }
                            }
                        }
                        tr { class: "text-xs text-gray-500 text-left",
                            th { class: "pl-2 pb-1 font-medium", "Content" }
                        }
                        tr { class: "text-xs bg-gray-50",
                            td { class: "py-2 px-4 font-medium", style: "word-wrap: break-word;", request.content }
                        }
                    }
                }
            }
        })
    }

    #[derive(PartialEq, Props)]
    struct AppState {
        entries: VecDeque<Request>,
        search_query: String,
    }

    #[derive(PartialEq, Props)]
    pub struct RequestTablesProps {
        entries: VecDeque<Request>,
    }

    pub fn RequestTables(cx: Scope<RequestTablesProps>) -> Element {
        let request_tables: Vec<LazyNodes<'_, '_>> = cx
            .props
            .entries
            .clone()
            .into_iter()
            .map(|entry: Request| {
                rsx! { RequestTable { entry: entry } }
            })
            .collect();

        render! {
            for request_table in request_tables {
                request_table
            }
        }
    }
}
