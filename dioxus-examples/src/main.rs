mod app;
mod pages;

use app::App;

fn main() {
    dioxus::launch(App);
}
