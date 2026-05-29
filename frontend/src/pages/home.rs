use crate::api::products_api::fetch_products;
use crate::components::error_message::ErrorMessage;
use crate::components::loading::Loading;
use crate::components::product_card::ProductCard;
use crate::components::search_bar::SearchBar;
use crate::components::product_filter::ProductFilter;
use crate::models::product::ProductFilters;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let (search, set_search) = create_signal(String::new());
    let (category_id, set_category_id) = create_signal(String::new());
    let (min_price, set_min_price) = create_signal(String::new());
    let (max_price, set_max_price) = create_signal(String::new());
    let (condition, set_condition) = create_signal(String::from("all"));
    let (location, set_location) = create_signal(String::new());
    let (status, set_status) = create_signal(String::from("all"));
    let (sort, set_sort) = create_signal(String::from("newest"));

    let (filter_trigger, set_filter_trigger) = create_signal(0);

    let get_current_filters = move || {
        ProductFilters {
            search: search.get(),
            category_id: category_id.get(),
            min_price: min_price.get(),
            max_price: max_price.get(),
            condition: condition.get(),
            location: location.get(),
            status: status.get(),
            sort: sort.get(),
        }
    };

    let products = create_resource(
        move || filter_trigger.get(), 
        move |_| async move { 
            let filters = get_current_filters();
            fetch_products(&filters).await 
        }
    );

    let on_search = create_action(move |_: &()| {
        set_filter_trigger.update(|n| *n += 1);
        async move {}
    });

    let on_apply = create_action(move |_: &()| {
        set_filter_trigger.update(|n| *n += 1);
        async move {}
    });

    let on_clear = create_action(move |_: &()| {
        set_search.set(String::new());
        set_category_id.set(String::new());
        set_min_price.set(String::new());
        set_max_price.set(String::new());
        set_condition.set(String::from("all"));
        set_location.set(String::new());
        set_status.set(String::from("all"));
        set_sort.set(String::from("newest"));
        set_filter_trigger.update(|n| *n += 1);
        async move {}
    });

    view! {
        <div class="home-container">
            <aside class="sidebar">
                <ProductFilter
                    category_id=category_id
                    set_category_id=set_category_id
                    min_price=min_price
                    set_min_price=set_min_price
                    max_price=max_price
                    set_max_price=set_max_price
                    condition=condition
                    set_condition=set_condition
                    location=location
                    set_location=set_location
                    status=status
                    set_status=set_status
                    sort=sort
                    set_sort=set_sort
                    on_apply=on_apply
                    on_clear=on_clear
                />
            </aside>
            <main style="flex: 1; min-width: 0;">
                <SearchBar
                    search_query=search
                    set_search_query=set_search
                    on_search=on_search
                />
                
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                    <h2 style="margin: 0; font-size: 1.5rem; color: var(--text-color);">"Productos destacados"</h2>
                </div>
                
                <Suspense fallback=move || view! { <Loading/> }>
                    {move || {
                        products.get().map(|result| match result {
                            Ok(data) => {
                                if data.is_empty() {
                                    view! { 
                                        <div style="text-align: center; padding: 50px; background: white; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                                            <h3 style="color: #6b7280;">"No se encontraron productos"</h3>
                                            <p style="color: #9ca3af;">"Prueba a cambiar los filtros o realizar otra búsqueda."</p>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="grid" style="grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 20px;">
                                            {data.into_iter().map(|p| view! { <ProductCard product=p/> }).collect_view()}
                                        </div>
                                    }.into_view()
                                }
                            }
                            Err(e) => view! { <ErrorMessage message=e/> }.into_view()
                        })
                    }}
                </Suspense>
            </main>
        </div>
    }
}
