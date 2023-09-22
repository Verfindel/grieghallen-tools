use leptos::*;

#[component]
pub fn StaticList<'a>(values: &'a Vec<i32>) -> impl IntoView {
    // let otherValues = vec![1, 2, 3, 4, 5];
    view! {
        <p>{values.clone()}</p>
        <ul>
            {values.iter().map(|v| view! { <li>{*v}</li> }).collect_view()}
        </ul>
    }
}
