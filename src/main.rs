use leptos::*;

mod app;
use crate::app::App;

fn main() {
    mount_to_body(|| view! { <App /> })
}
