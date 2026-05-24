use leptos::*;

#[component]
pub fn ErrorMessage(message: String) -> impl IntoView {
    view! {
        <div class="alert-error">
            <strong>"Error: "</strong> {message}
        </div>
    }
}
