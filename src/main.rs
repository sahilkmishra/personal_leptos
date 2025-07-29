use leptos::prelude::*;
use leptos::mount::{mount_to_body};

#[component]
fn App(increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
    <div class="container">

        <h1>"Welcome to Leptos"</h1>
        <h2><i>"On Github Pages"</i></h2>

        <button
            on:click= move |_| {
                set_count(count() + increment)
            }
        >
            "Click me: "
            {count}
        </button>
    </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App increment=5 />
        }
    })
}
