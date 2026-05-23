use leptos::*;

#[component]
fn Calculator() -> impl IntoView {
    // Signals para los dos inputs numéricos y el resultado
    let (num1, set_num1) = create_signal(String::new());
    let (num2, set_num2) = create_signal(String::new());
    let (resultado, set_resultado) = create_signal(String::from("0"));

    // Función de ayuda para parsear los inputs de manera segura
    let get_numbers = move || -> Option<(f64, f64)> {
        let n1 = num1.get().parse::<f64>().ok()?;
        let n2 = num2.get().parse::<f64>().ok()?;
        Some((n1, n2))
    };

    // Handlers para cada operación
    let suma = move |_| {
        if let Some((n1, n2)) = get_numbers() {
            set_resultado.set((n1 + n2).to_string());
        } else {
            set_resultado.set("Entrada inválida".to_string());
        }
    };

    let resta = move |_| {
        if let Some((n1, n2)) = get_numbers() {
            set_resultado.set((n1 - n2).to_string());
        } else {
            set_resultado.set("Entrada inválida".to_string());
        }
    };

    let multiplicacion = move |_| {
        if let Some((n1, n2)) = get_numbers() {
            set_resultado.set((n1 * n2).to_string());
        } else {
            set_resultado.set("Entrada inválida".to_string());
        }
    };

    let division = move |_| {
        if let Some((n1, n2)) = get_numbers() {
            if n2 == 0.0 {
                set_resultado.set("Error: División por cero".to_string());
            } else {
                set_resultado.set((n1 / n2).to_string());
            }
        } else {
            set_resultado.set("Entrada inválida".to_string());
        }
    };

    view! {
        <div style="font-family: sans-serif; max-width: 400px; margin: 2rem auto; padding: 2rem; border: 1px solid #ccc; border-radius: 8px; text-align: center;">
            <h2>"Calculadora Leptos"</h2>
            
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
                <button on:click=suma style="flex: 1; padding: 0.5rem; cursor: pointer;">"+"</button>
                <button on:click=resta style="flex: 1; padding: 0.5rem; cursor: pointer;">"-"</button>
                <button on:click=multiplicacion style="flex: 1; padding: 0.5rem; cursor: pointer;">"*"</button>
                <button on:click=division style="flex: 1; padding: 0.5rem; cursor: pointer;">"/"</button>
            </div>

            <div style="padding: 1rem; background-color: #f0f0f0; border-radius: 4px;">
                <strong>"Resultado: "</strong> 
                <span>{resultado}</span>
            </div>
        </div>
    }
}

fn main() {
    // Configura el panic hook para mostrar errores en la consola del navegador
    console_error_panic_hook::set_once();
    
    // Monta la aplicación en el body
    leptos::mount_to_body(|| view! { <Calculator/> })
}
