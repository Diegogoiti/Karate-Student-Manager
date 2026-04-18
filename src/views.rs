// src/views/home.rs
use dioxus::prelude::*;
use crate::components::datatable::DataTable;
use crate::components::searchbar::SearchBar;
use crate::my_app;




#[component]
pub fn Home() -> Element {
    let mut estado = use_context::<Signal<my_app::MyApp>>();
    let hay_seleccion = !estado.read().seleccionados.is_empty();
    let texto_boton = if hay_seleccion { "Deseleccionar todos" } else { "Seleccionar todos" };

    rsx! {
        div { class: "flex flex-col h-full space-y-4 ",
            div { class: "relative flex items-center justify-center py-2",
                h2 { class: "text-3xl font-bold text-gray-800 text-center", "Consultar" }
                button {
            class: "absolute right-0 px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 transition-colors text-sm",
            onclick: move |_| {
                estado.write().toggle_all();
            },
            "{texto_boton}"
        }}

            DataTable {  estado: estado }
            
        
            
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
     let hay_seleccion = !estado.read().seleccionados.is_empty();
    let texto_boton = if hay_seleccion { "Deseleccionar todos" } else { "Seleccionar todos" };
    rsx! {
        div { class: "flex flex-col h-full space-y-4",
            div { class: "relative flex items-center justify-center py-2",
                h2 { class: "text-3xl font-bold text-gray-800 text-center", "Buscar" }
                button {
            class: "absolute right-0 px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 transition-colors text-sm",
            onclick: move |_| {
                estado.write().toggle_all();
            },
            "{texto_boton}"
        }}
            

           
            SearchBar {  }
            DataTable { estado: estado }
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