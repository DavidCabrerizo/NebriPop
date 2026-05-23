use leptos::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Operation {
    id: u64,
    left_value: f64,
    operator: String,
    right_value: f64,
    result: f64,
}

#[derive(Serialize, Debug)]
struct NewOperation {
    left_value: f64,
    operator: String,
    right_value: f64,
    result: f64,
}

#[component]
fn Calculator() -> impl IntoView {
    let (num1, set_num1) = create_signal(String::new());
    let (num2, set_num2) = create_signal(String::new());
    let (resultado, set_resultado) = create_signal(String::from("0"));
    let (error_msg, set_error_msg) = create_signal(String::new());
    
    // Resource to fetch history
    let fetch_history = create_resource(
        || (),
        |_| async move {
            let client = Client::new();
            match client.get("http://127.0.0.1:3000/operations").send().await {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<Vec<Operation>>().await {
                        data
                    } else {
                        vec![]
                    }
                },
                Err(_) => vec![],
            }
        },
    );

    let get_numbers = move || -> Option<(f64, f64)> {
        let n1 = num1.get().parse::<f64>().ok()?;
        let n2 = num2.get().parse::<f64>().ok()?;
        Some((n1, n2))
    };

    let send_operation = move |left: f64, op: &str, right: f64, res: f64| {
        let payload = NewOperation {
            left_value: left,
            operator: op.to_string(),
            right_value: right,
            result: res,
        };
        
        leptos::logging::log!("Enviando operación al backend: {:?}", payload);
        
        spawn_local(async move {
            let client = Client::new();
            match client.post("http://127.0.0.1:3000/operations")
                .json(&payload)
                .send()
                .await 
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        leptos::logging::log!("Operación guardada con éxito");
                        set_error_msg.set(String::new());
                        fetch_history.refetch(); // Actualiza la lista explícitamente
                    } else {
                        set_error_msg.set("Error desde el servidor al guardar".to_string());
                    }
                },
                Err(e) => {
                    leptos::logging::log!("Error de reqwest: {:?}", e);
                    set_error_msg.set("Error de conexión con el backend".to_string());
                }
            }
        });
    };

    let calculate = move |op: &str| {
        set_error_msg.set(String::new());
        if let Some((n1, n2)) = get_numbers() {
            if op == "/" && n2 == 0.0 {
                set_resultado.set("Error: División por cero".to_string());
                return;
            }
            
            let res = match op {
                "+" => n1 + n2,
                "-" => n1 - n2,
                "*" => n1 * n2,
                "/" => n1 / n2,
                _ => 0.0,
            };
            
            set_resultado.set(res.to_string());
            send_operation(n1, op, n2, res);
        } else {
            set_resultado.set("Entrada inválida".to_string());
        }
    };

    view! {
        <div style="font-family: sans-serif; max-width: 600px; margin: 2rem auto; padding: 2rem; border: 1px solid #ccc; border-radius: 8px;">
            <h2 style="text-align: center;">"Calculadora Full-Stack (Leptos + Axum)"</h2>
            
            <div style="margin-bottom: 1rem;">
                <input 
                    type="number" 
                    placeholder="Número 1"
                    on:input=move |ev| set_num1.set(event_target_value(&ev))
                    prop:value=num1
                    style="margin-bottom: 0.5rem; padding: 0.5rem; width: 100%; box-sizing: border-box;"
                />
                <input 
                    type="number" 
                    placeholder="Número 2"
                    on:input=move |ev| set_num2.set(event_target_value(&ev))
                    prop:value=num2
                    style="margin-bottom: 0.5rem; padding: 0.5rem; width: 100%; box-sizing: border-box;"
                />
            </div>

            <div style="margin-bottom: 1.5rem; display: flex; justify-content: space-between; gap: 0.5rem;">
                <button on:click=move |_| calculate("+") style="flex: 1; padding: 0.5rem; cursor: pointer;">"+"</button>
                <button on:click=move |_| calculate("-") style="flex: 1; padding: 0.5rem; cursor: pointer;">"-"</button>
                <button on:click=move |_| calculate("*") style="flex: 1; padding: 0.5rem; cursor: pointer;">"*"</button>
                <button on:click=move |_| calculate("/") style="flex: 1; padding: 0.5rem; cursor: pointer;">"/"</button>
            </div>

            <div style="padding: 1rem; background-color: #f0f0f0; border-radius: 4px; margin-bottom: 1rem;">
                <strong>"Resultado: "</strong> 
                <span>{resultado}</span>
            </div>

            <p style="color: red;">{error_msg}</p>

            <hr/>
            <h3>"Historial de Operaciones"</h3>
            <Suspense fallback=move || view! { <p>"Cargando historial..."</p> }>
                {move || {
                    fetch_history.get().map(|ops| {
                        if ops.is_empty() {
                            view! { <p>"No hay operaciones guardadas."</p> }.into_view()
                        } else {
                            view! {
                                <ul>
                                    {ops.into_iter().map(|op| {
                                        view! {
                                            <li>
                                                {format!("ID {}: {} {} {} = {}", op.id, op.left_value, op.operator, op.right_value, op.result)}
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            }.into_view()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <Calculator/> })
}
