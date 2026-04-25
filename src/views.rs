//! se encarga de dibujar las vistas segun las rutas seleccionadas
//! contiene las funciones con el codigo especifico de cada vista



use std::string;

use dioxus::prelude::*;
use crate::components::datatable::DataTable;
use crate::components::searchbar::SearchBar;
use crate::my_app;




#[component]
pub fn Home() -> Element {
    let mut estado = use_context::<Signal<my_app::MyApp>>();
    let todos_seleccionados = !estado.read().alumnos.is_empty()
        && estado.read().alumnos.iter().all(|a| estado.read().seleccionados.contains(&a.id));
    let texto_boton = if todos_seleccionados { "Deseleccionar todos" } else { "Seleccionar todos" };
    let mut alumnos_lista = use_signal(|| estado.read().alumnos.clone());

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
    let texto_boton = if todos_seleccionados { "Deseleccionar todos" } else { "Seleccionar todos" };

    {
        let filtro = filtro.clone();
        let estado = estado.clone();
        let mut alumnos_filtrados = alumnos_filtrados.clone();
        use_effect(move || {
            let app = estado.read();
            alumnos_filtrados.set(app.filtrar_alumnos(filtro.read().0, &filtro.read().1));
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

            SearchBar { on_input: move |data| filtro.set(data) }
            DataTable { alumnos_lista: alumnos_filtrados, estado }
        }

    }
}

#[component]
pub fn Filtrar() -> Element {
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800", "Filtrar Alumnos" }
            p { class: "text-gray-600", "Aquí podrás filtrar alumnos por curso, promedio o estado." }

            // Un pequeño indicador de que la vista cargó
            div { class: "p-10 border-2 border-dashed border-gray-300 rounded-xl text-center",
                "Funcionalidad de filtrado (Próximamente)"
            }
        }
    }
}

#[component]
pub fn Agregar() -> Element {
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800", "Agregar Alumno" }
            p { class: "text-gray-600", "Aquí podrás agregar nuevos alumnos al sistema." }

            // Un pequeño indicador de que la vista cargó
            div { class: "p-10 border-2 border-dashed border-gray-300 rounded-xl text-center",
                "Formulario de agregar alumno (Próximamente)"
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