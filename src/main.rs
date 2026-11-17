mod app;
mod components;
mod data;
mod pages;

use app::App;

pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
