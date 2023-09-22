use leptos::*;

mod app;
mod static_list;
// use crate::app::App;
use crate::app::DynamicApp;
use crate::static_list::StaticList;

fn main() {
    // mount_to_body(|| view! { <App /> });
    mount_to_body(|| view! { <DynamicApp />});
    mount_to_body(|| {
        let values = vec![1, 2, 3, 4, 5];
        view! { <StaticList values= &values /> }
    });
}
