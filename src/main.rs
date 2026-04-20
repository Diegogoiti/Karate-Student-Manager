mod models;
mod utils;
mod components;
mod views;
mod my_app;

use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder};
use crate::components::sidebar::Sidebar;
use crate::views::*;
use crate::models::Alumno;

const CSS: &str = include_str!("../assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Sidebar)]
    #[route("/")]
    Home {},
    #[route("/buscar")] // Aunque no tengan contenido aún, deben existir
    Buscar {},
    #[route("/filtrar")]
    Filtrar {},
    #[route("/agregar")]
    Agregar {},
    #[route("/editar")]
    Editar {},
    #[route("/eliminar")]
    Eliminar {},
}

fn main() {
    // 1. Configuramos la ventana (SIN el menú aquí)
    let window = WindowBuilder::new()
        .with_title("DYFIT Student Manager")
        .with_background_color((17, 24, 39, 255))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(800.0, 600.0))
        .with_visible(false); // RGBA de gray-900

    // 2. Creamos el config y AQUÍ le quitamos el menú
    let config = Config::default()
        .with_window(window)
        .with_menu(None); // <--- El .with_menu() va pegado al Config, no al WindowBuilder

    // 3. Lanzamos
    LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[component]
fn App() -> Element {
    let lista_alumnos = use_signal(|| vec![
        Alumno {
            id: 1,
            nombre: "Diego ".into(),
            rango: 1,
            edad: 17,
            fecha_de_nacimiento: "2009-02-15".into(),
            representante: "Madre de Diego".into(),
            numero_contacto: "0412-1234567".into(),
            seleccionado: false,
        },
        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },
                Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            rango: 1,
            edad: 12,
            fecha_de_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            numero_contacto: "0424-7654321".into(),
            seleccionado: false,
        },
    ]);
    use_effect(move || {
        // Muestra la ventana solo cuando el componente esté montado
        dioxus::desktop::window().set_visible(true);
    });

let estado_app = use_signal(|| {
        my_app::MyApp::new()
    });

    // 3. Proveemos el signal ya creado al contexto
    use_context_provider(|| estado_app);




    rsx! {
        
        style { "{CSS}" }
        Router::<Route> {}
    }
}