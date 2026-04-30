//! se encarga de dibujar las vistas segun las rutas seleccionadas
//! contiene las funciones con el codigo especifico de cada vista

use std::string;

use crate::components::datatable::DataTable;
use crate::components::filter::Filter;
use crate::components::searchbar::SearchBar;
use crate::my_app::{self, Columnas};
use crate::models::{Alumno, Cintas};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut estado = use_context::<Signal<my_app::MyApp>>();
    let todos_seleccionados = !estado.read().alumnos.is_empty()
        && estado
            .read()
            .alumnos
            .iter()
            .all(|a| estado.read().seleccionados.contains(&a.id));
    let texto_boton = if todos_seleccionados {
        "Deseleccionar todos"
    } else {
        "Seleccionar todos"
    };
    let alumnos_lista = use_signal(|| estado.read().alumnos.clone());

    rsx! {
        div { class: "flex flex-col h-full space-y-4 ",
            div { class: "relative flex items-center justify-center py-2",
                h2 { class: "text-3xl font-bold text-gray-800 text-center", "Consultar" }
                button {
                    class: "absolute right-0 px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 transition-colors text-sm",
                    onclick: move |_| {
                        estado.write().toggle_all(alumnos_lista.read().clone());
                    },
                    "{texto_boton}"
                }
            }

            DataTable { alumnos_lista, estado }

            // Pie de tabla con resumen
            div { class: "text-gray-500 text-xs",
                "Mostrando {estado.read().alumnos.len()} alumnos registrados"
            }
        }
    }
}

#[component]
pub fn Buscar() -> Element {
    let mut estado = use_context::<Signal<my_app::MyApp>>();

    let mut filtro = use_signal(|| (my_app::Columnas::Nombre, String::new()));
    let alumnos_filtrados = use_signal(|| estado.read().alumnos.clone());
    let todos_seleccionados = !alumnos_filtrados.read().is_empty()
        && alumnos_filtrados
            .read()
            .iter()
            .all(|a| estado.read().seleccionados.contains(&a.id));
    let texto_boton = if todos_seleccionados {
        "Deseleccionar todos"
    } else {
        "Seleccionar todos"
    };

    {
        let filtro = filtro.clone();
        let estado = estado.clone();
        let mut alumnos_filtrados = alumnos_filtrados.clone();
        use_effect(move || {
            let app = estado.read();
            alumnos_filtrados.set(app.buscar_alumnos(filtro.read().0, &filtro.read().1));
        });
    }

    rsx! {
        div { class: "flex flex-col h-full space-y-4",
            div { class: "relative flex items-center justify-center py-2",
                h2 { class: "text-3xl font-bold text-gray-800 text-center", "Buscar" }
                button {
                    class: "absolute right-0 px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 transition-colors text-sm",
                    onclick: move |_| {
                        estado.write().toggle_all(alumnos_filtrados.read().clone());
                    },
                    "{texto_boton}"
                }
            }

            SearchBar {
                on_input: move |data| filtro.set(data),
                options: vec![
                    ("Id".to_string(), my_app::Columnas::Id),
                    ("Nombre".to_string(), my_app::Columnas::Nombre),
                    ("Representante".to_string(), my_app::Columnas::Representante),
                    ("Teléfono".to_string(), my_app::Columnas::Telefono),
                ],
                placeholder: "Buscar alumno...".to_string(),
                initial_param: my_app::Columnas::Nombre,
            }
            DataTable { alumnos_lista: alumnos_filtrados, estado }
        }

    }
}

#[component]
pub fn Filtrar() -> Element {
    let mut estado = use_context::<Signal<my_app::MyApp>>();

    // Iniciamos con Cinta por defecto para que coincida con el componente Filter
    let mut filtro = use_signal(|| (my_app::Columnas::Cinta, "Blanca".to_string(), false));
    let mut alumnos_filtrados = use_signal(|| estado.read().alumnos.clone());

    let todos_seleccionados = !alumnos_filtrados.read().is_empty()
        && alumnos_filtrados
            .read()
            .iter()
            .all(|a| estado.read().seleccionados.contains(&a.id));

    let texto_boton = if todos_seleccionados {
        "Deseleccionar todos"
    } else {
        "Seleccionar todos"
    };

    // Lógica de filtrado reactiva
    use_effect(move || {
        let app = estado.read();
        let (columna, valor, solo_rallita) = filtro.read().clone();

        let resultado = match columna {
            Columnas::Cinta => app.filtrar_cinta(valor, solo_rallita),
            Columnas::Edad => app.filtrar_edad(valor),
            _ => app.buscar_alumnos(columna, &valor),
        };

        alumnos_filtrados.set(resultado);
    });

    rsx! {
        div { class: "flex flex-col h-full space-y-4",
            div { class: "relative flex items-center justify-center py-2",
                h2 { class: "text-3xl font-bold text-gray-800 text-center", "Filtrar" }
                button {
                    class: "absolute right-0 px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 transition-colors text-sm",
                    onclick: move |_| {
                        estado.write().toggle_all(alumnos_filtrados.read().clone());
                    },
                    "{texto_boton}"
                }
            }

            Filter {
                on_input: move |data| filtro.set(data),
                options: vec![
                    ("Cinta".to_string(), Columnas::Cinta),
                    ("Edad".to_string(), Columnas::Edad),
                ],
                placeholder: "Filtrar alumnos...".to_string(),
                initial_param: my_app::Columnas::Cinta,
            }

            DataTable { alumnos_lista: alumnos_filtrados, estado }
        }
    }
}


#[component]
pub fn Agregar() -> Element {
    // 1. Signals para manejar el estado del formulario
    let mut nombre = use_signal(|| "".to_string());
    let mut fecha_nac = use_signal(|| "".to_string());
    let mut rango = use_signal(|| 10u32); // Por defecto "Blanca" (valor 10)[cite: 2]
    let mut representante = use_signal(|| "".to_string());
    let mut contacto = use_signal(|| "".to_string());
    let mut rallita = use_signal(|| false);

    rsx! {


div { class: "flex flex-col h-screen max-w-2xl mx-auto p-4",

        div { class: "flex flex-col h-full space-y-6 max-w-2xl mx-auto",
            // Encabezado
            div { class: "text-center py-4",
                h2 { class: "text-3xl font-bold text-gray-800", "Registrar Nuevo Alumno" }
                p { class: "text-gray-500", "Ingresa los datos personales y de grado del karateka." }
            }

            // Contenedor del Formulario[cite: 6, 8]
            div { class: "flex-1 flex flex-col justify-around bg-white p-8 rounded-2xl shadow-xl ",
                
                // Campo: Nombre
                div { class: "flex flex-col space-y-1",
                    label { class: "text-sm font-semibold text-gray-600", "Nombre Completo" }
                    input {
                        r#type: "text",
                        class: "p-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none",
                        placeholder: "Ej. Ichiro Suzuki",
                        oninput: move |e| nombre.set(e.value())
                    }
                }

                div { class: "grid grid-cols-2 gap-4",
                    // Campo: Fecha de Nacimiento
                    div { class: "flex flex-col space-y-1",
                        label { class: "text-sm font-semibold text-gray-600", "Fecha de Nacimiento" }
                        input {
                            r#type: "date",
                            class: "p-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none",
                            oninput: move |e| fecha_nac.set(e.value())
                        }
                    }

                    // Campo: Grado / Cinta[cite: 2]
                    div { class: "flex flex-col",
                        label { class: "text-sm font-semibold text-gray-600", "Grado (Kyu)" }
                        select {
                            class: "p-2 rounded-lg border border-gray-300 bg-gray-50",
                            onchange: move |e| {
                                if let Ok(val) = e.value().parse::<u32>() {
                                    rango.set(val);
                                }
                            },
                            {Cintas::all_variants().iter().map(|cinta| rsx! {
                                option { value: "{cinta.valor()}", "{cinta.label()}" }
                            })}
                        }
                    }
                }

                // Campo: Representante
                div { class: "flex flex-col",
                    label { class: "text-sm font-semibold text-gray-600", "Representante" }
                    input {
                        r#type: "text",
                        class: "p-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none",
                        placeholder: "Nombre del padre o tutor",
                        oninput: move |e| representante.set(e.value())
                    }
                }

                // Campo: Contacto y Rallita
                div { class: "grid grid-cols-2 gap-4 ",
                    div { class: "flex flex-col space-y-1",
                        label { class: "text-sm font-semibold text-gray-600", "Teléfono de Contacto" }
                        input {
                            r#type: "tel",
                            class: "p-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none",
                            placeholder: "0412-0000000",
                            oninput: move |e| contacto.set(e.value())
                        }
                    }

                    // Checkbox de Rallita[cite: 6]
                    label { class: "flex items-center space-x-3 p-3 bg-gray-50 rounded-lg border border-gray-200 cursor-pointer hover:bg-gray-100 transition-colors",
                        input {
                            r#type: "checkbox",
                            class: "w-5 h-5 text-blue-600 rounded border-gray-300 focus:ring-blue-500",
                            onchange: move |_| rallita.set(!rallita.cloned())
                        }
                        span { class: "text-sm font-medium text-gray-700", "Grado con Rallita" }
                    }
                }

                // Botón de Acción
                button {
                    class: "w-full mt-4 py-3 bg-blue-600 text-white font-bold rounded-xl hover:bg-blue-700 shadow-lg shadow-blue-200 transition-all active:scale-[0.98]",
                    onclick: move |_| {
                        println!("Guardando: {} - Cinta: {}", nombre.read(), rango.read());
                        // Aquí conectarás con app_state.write().database.save(...) más adelante
                    },
                    "Añadir Al Dojo"
                }
            }

            // Pie de página con recordatorio del código de conducta[cite: 2]
            div { class: "text-center text-gray-400 text-[10px] tracking-widest uppercase",
                "BudoDB • Gestión de Disciplina"
            }
        }
    }
    }
}

#[component]
pub fn Editar() -> Element {
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800", "Editar Alumno" }
            p { class: "text-gray-600",
                "Aquí podrás editar la información de los alumnos existentes."
            }

            // Un pequeño indicador de que la vista cargó
            div { class: "p-10 border-2 border-dashed border-gray-300 rounded-xl text-center",
                "Formulario de edición de alumno (Próximamente)"
            }
        }
    }
}

#[component]
pub fn Eliminar() -> Element {
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800", "Eliminar Alumno" }
            p { class: "text-gray-600", "Aquí podrás eliminar alumnos del sistema." }

            // Un pequeño indicador de que la vista cargó
            div { class: "p-10 border-2 border-dashed border-gray-300 rounded-xl text-center",
                "Funcionalidad de eliminación de alumno (Próximamente)"
            }
        }
    }
}
