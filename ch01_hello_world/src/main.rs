use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let html = "<p>This HTML will be injected.</p>";
    log::info!("Hello world!!");
    leptos::mount::mount_to_body(move || {
        view! {
            <div class="container">
                <div class="text-danger" inner_html=html />
                <div class="text-primary">"Hello World!!!"</div>
            </div>
        }
    });
}
