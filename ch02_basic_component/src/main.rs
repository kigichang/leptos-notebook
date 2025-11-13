use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="container pt-5">
            <button type="button" class="btn btn-primary position-relative"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Click Me"
                <span class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-danger">
                    {move || if count.get() < 10 { format!("{}", count.get()) } else { "10+".to_string() } }
                <span class="visually-hidden">unread messages</span>
                </span>
            </button>
            <p>"Count: " {count} </p>
            <p>"Double Count: " {move || count.get() * 2 }</p>
        </div>
    }
}
