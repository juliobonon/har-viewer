mod app;
mod components;
mod parsers;

use app::App;

fn main() {
    dioxus_desktop::launch(App);
}
