use leptos::*;
use std::cmp::min;

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    #[prop(default = 100)] max: u16,
    progress: F
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (stacks, set_stacks) = create_signal(0);
    let units = move || count() + stacks() * 64;
    view! {
        <ProgressBar max=27 progress=stacks />
        <button
            on:click=move |_| {
                set_stacks.update(|n| *n = min(*n+1, 27));
                if stacks() == 27 {
                    set_count(0);
                }
            }
        >
            "Stacks: "
            {stacks}
        </button>
        <br />
        <br />
        <ProgressBar max=64 progress=count />
        <button
            on:click=move |_| {
                set_count.update(|n| *n = min(*n+16, 64));
                if count() == 64 {
                    set_stacks.update(|n| *n = min(*n+1, 27));
                    set_count(0);
                }
            }
            class:red=move || count() % 2 == 1
            disabled=move || stacks() == 27
        >
            "Qty: "
            {count}
        </button>
        <br />
        <p>
            "Your total amount is "
            {units}
        </p>
        <ProgressBar max=1728 progress=units />
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
