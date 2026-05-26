use leptos::*;

use crate::api::categories_api::fetch_categories;

#[component]
pub fn ProductFilter(
    category_id: ReadSignal<String>,
    set_category_id: WriteSignal<String>,
    min_price: ReadSignal<String>,
    set_min_price: WriteSignal<String>,
    max_price: ReadSignal<String>,
    set_max_price: WriteSignal<String>,
    condition: ReadSignal<String>,
    set_condition: WriteSignal<String>,
    location: ReadSignal<String>,
    set_location: WriteSignal<String>,
    status: ReadSignal<String>,
    set_status: WriteSignal<String>,
    sort: ReadSignal<String>,
    set_sort: WriteSignal<String>,
    on_apply: Action<(), ()>,
    on_clear: Action<(), ()>,
) -> impl IntoView {
    let categories_resource = create_resource(|| (), |_| async move { fetch_categories().await });

    view! {
        <div class="product-filter" style="background: white; border-radius: 8px; padding: 20px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
            <h3 style="margin-top: 0; color: var(--text-color); border-bottom: 1px solid var(--border-color); padding-bottom: 10px; margin-bottom: 20px;">"Filtros"</h3>
            
            <div class="filter-group" style="margin-bottom: 15px;">
                <label style="display: block; font-weight: 500; margin-bottom: 5px; font-size: 0.9rem;">"Categoría"</label>
                <select
                    prop:value=category_id
                    on:change=move |ev| set_category_id.set(event_target_value(&ev))
                    style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 6px; outline: none;"
                >
                    <option value="">"Todas las categorías"</option>
                    <Suspense fallback=move || view! { <option>"Cargando..."</option> }>
                        {move || {
                            categories_resource.get().map(|res| match res {
                                Ok(categories) => {
                                    categories.into_iter().map(|c| {
                                        view! { <option value=c.id.to_string()>{c.name}</option> }
                                    }).collect_view()
                                }
                                Err(_) => view! { <option>"Error al cargar"</option> }.into_view()
                            })
                        }}
                    </Suspense>
                </select>
            </div>

            <div class="filter-group" style="margin-bottom: 15px;">
                <label style="display: block; font-weight: 500; margin-bottom: 5px; font-size: 0.9rem;">"Precio (€)"</label>
                <div class="price-range" style="display: flex; gap: 10px; align-items: center;">
                    <input
                        type="number"
                        placeholder="Mín"
                        prop:value=min_price
                        on:input=move |ev| set_min_price.set(event_target_value(&ev))
                        min="0"
                        style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 6px; outline: none;"
                    />
                    <span style="color: #6b7280;">"-"</span>
                    <input
                        type="number"
                        placeholder="Máx"
                        prop:value=max_price
                        on:input=move |ev| set_max_price.set(event_target_value(&ev))
                        min="0"
                        style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 6px; outline: none;"
                    />
                </div>
            </div>

            <div class="filter-group" style="margin-bottom: 15px;">
                <label style="display: block; font-weight: 500; margin-bottom: 5px; font-size: 0.9rem;">"Estado del producto"</label>
                <select
                    prop:value=condition
                    on:change=move |ev| set_condition.set(event_target_value(&ev))
                    style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 6px; outline: none;"
                >
                    <option value="all">"Cualquiera"</option>
                    <option value="new">"Nuevo"</option>
                    <option value="like_new">"Como nuevo"</option>
                    <option value="good">"Bueno"</option>
                    <option value="used">"Usado"</option>
                    <option value="damaged">"Dañado"</option>
                </select>
            </div>

            <div class="filter-group" style="margin-bottom: 15px;">
                <label style="display: block; font-weight: 500; margin-bottom: 5px; font-size: 0.9rem;">"Ubicación"</label>
                <input
                    type="text"
                    placeholder="Ej. Madrid"
                    prop:value=location
                    on:input=move |ev| set_location.set(event_target_value(&ev))
                    style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 6px; outline: none; box-sizing: border-box;"
                />
            </div>

            <div class="filter-group" style="margin-bottom: 15px;">
                <label style="display: block; font-weight: 500; margin-bottom: 5px; font-size: 0.9rem;">"Disponibilidad"</label>
                <select
                    prop:value=status
                    on:change=move |ev| set_status.set(event_target_value(&ev))
                    style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 6px; outline: none;"
                >
                    <option value="all">"Cualquiera"</option>
                    <option value="available">"Disponible"</option>
                    <option value="reserved">"Reservado"</option>
                    <option value="sold">"Vendido"</option>
                </select>
            </div>

            <div class="filter-group" style="margin-bottom: 25px;">
                <label style="display: block; font-weight: 500; margin-bottom: 5px; font-size: 0.9rem;">"Ordenar por"</label>
                <select
                    prop:value=sort
                    on:change=move |ev| set_sort.set(event_target_value(&ev))
                    style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 6px; outline: none;"
                >
                    <option value="newest">"Más recientes"</option>
                    <option value="price_asc">"Precio: Menor a Mayor"</option>
                    <option value="price_desc">"Precio: Mayor a Menor"</option>
                </select>
            </div>

            <div class="filter-actions" style="display: flex; flex-direction: column; gap: 10px;">
                <button class="btn" on:click=move |_| on_apply.dispatch(()) style="width: 100%; padding: 10px; font-size: 1rem; border-radius: 20px;">"Aplicar Filtros"</button>
                <button class="btn-secondary" on:click=move |_| on_clear.dispatch(()) style="width: 100%; padding: 10px; font-size: 1rem; border-radius: 20px; background-color: white; color: var(--primary-color); border: 2px solid var(--primary-color);">"Limpiar"</button>
            </div>
        </div>
    }
}
