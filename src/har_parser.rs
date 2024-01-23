pub mod har_parser {
    pub struct Request {
        pub url: String,
        pub method: String,
        pub status: String,
        pub headers: Vec<String>,
    }

    use serde_json::Value;
    use std::collections::VecDeque;

    pub fn parse_har(value: Value) -> VecDeque<Request> {
        let mut entries_to_return: VecDeque<Request> = Default::default();
        let entries = &value["log"]["entries"];

        for entry in entries.as_array().unwrap() {
            let method = &entry["request"]["method"];
            let request_info = &entry["request"];
            let response = &entry["response"];
            let url = &request_info["url"];
            let status = &response["status"];
            let status_text = &response["statusText"];
            let content_type = &response["content"]["mimeType"];
            let size = &response["content"]["size"];
            let headers = &response["headers"];
            let post_data = &request_info["postData"];
            let mut content_length = 0;

            match headers {
                Value::Array(headers) => {
                    for header in headers {
                        let name = &header["name"];
                        let value = &header["value"];
                        if name == "Content-Length" {
                            content_length = value.as_str().unwrap().parse::<i32>().unwrap();
                        }
                    }
                }
                _ => println!("Headers are not available!"),
            }

            let headers_str_vec = headers
                .as_array()
                .unwrap()
                .iter()
                .map(|header| {
                    let name = &header["name"];
                    let value = &header["value"];
                    format!("{}: {}", name, value)
                })
                .collect::<Vec<String>>();

            let request_struct = Request {
                url: url.to_string(),
                method: method.to_string(),
                status: status.to_string(),
                headers: headers_str_vec,
            };
            entries_to_return.push_front(request_struct)
        }

        entries_to_return
    }
}
