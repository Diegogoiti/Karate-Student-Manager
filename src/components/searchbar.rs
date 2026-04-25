use dioxus::prelude::*;
use crate::my_app::Columnas;






#[component]
pub fn SearchBar(on_input: EventHandler<(Columnas,String)>) -> Element {
    let mut search_text = use_signal(|| "".to_string());
    let mut selected_param = use_signal(|| Columnas::Nombre);

    

    let mut notificar = move || {
        on_input.call((selected_param.cloned(), search_text.cloned()));
    };

    rsx! {
        div { class: "flex flex-row space-x-2 p-4 bg-white rounded-xl shadow-md border border-gray-200",
            // Dropdown para el parámetro
            select {
                class: "p-2 rounded bg-gray-50 border border-gray-300 text-gray-700",
                onchange: move |evt| {
                    match evt.value().as_str() {
                        "Nombre" => selected_param.set(Columnas::Nombre),
                        "id" => selected_param.set(Columnas::Id),
                        "Representante" => selected_param.set(Columnas::Representante),
                        "Teléfono" => selected_param.set(Columnas::Telefono),
                        _ => {}
                    }
                    notificar();
                },
                option { value: "Nombre", "Nombre" }
                option { value: "id", "id" }
                option { value: "Representante", "Representante" }
                option { value: "Teléfono", "Teléfono" }
            }

            // Input de texto
            input {
                class: "flex-1 p-2 rounded border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none",
                placeholder: "Buscar alumno...",
                value: "{search_text}",
                oninput: move |evt| {
                    search_text.set(evt.value());
                    notificar();
                },
            }
        }
    }
}