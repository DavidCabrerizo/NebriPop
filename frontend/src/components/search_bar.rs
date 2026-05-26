use leptos::*;

#[component]
pub fn SearchBar(
    search_query: ReadSignal<String>,
    set_search_query: WriteSignal<String>,
    on_search: Action<(), ()>,
) -> impl IntoView {
    view! {
        <div class="search-bar" style="display: flex; gap: 10px; width: 100%; margin-bottom: 20px;">
            <input
                type="text"
                placeholder="Buscar productos, marcas, modelos..."
                prop:value=search_query
                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                on:keypress=move |ev| {
                    if ev.key() == "Enter" {
                        on_search.dispatch(());
                    }
                }
                style="flex: 1; padding: 12px 20px; font-size: 16px; border: 2px solid var(--primary-color); border-radius: 24px; outline: none;"
            />
            <button class="btn" on:click=move |_| on_search.dispatch(()) style="border-radius: 24px; padding: 0 24px; font-size: 16px;">
                "Buscar"
            </button>
        </div>
    }
}
