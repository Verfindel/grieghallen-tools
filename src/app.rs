use leptos::*;

// #[component]
// pub fn App() -> impl IntoView {
//     let (count, set_count) = create_signal(0);

//     view! {
//         <button
//             on:click=move |_| {
//                 set_count.update(|n| *n += 1);
//             }
//         >"Click me: "
//             {count}
//         </button>
//     }
// }

/// A modular progress bar.
#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// The current progress of the bar.
    progress: F,
) -> impl IntoView {
    view! {
    <progress
        max=max
        value=progress
    />}
}

#[component]
pub fn DynamicApp() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let double_count = move || count() * 2;
    view! {
        <button
            on:click=move |_| {
                if count() == 50 {
                    set_count.update(|n| *n = 0);
                    return;
                }
                set_count.update(|n| *n += 1);
            }
            class:red=move || count() % 2 == 1
            class:blue=move || count() % 2 == 0
        >"Click me"
        </button>

        <p>"Count: " {count}</p>
        <p>"Double count: " {double_count}</p>

        <ProgressBar max=50 progress=count/>
        <br />
        <ProgressBar max=100 progress=double_count/>
    }
}
