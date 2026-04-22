
use std::collections::HashSet;

use crate::models::{Alumno, Database}; 

pub struct MyApp {
    pub alumnos: Vec<Alumno>,
    pub seleccionados: HashSet<usize>,
    database: Database,
}

impl MyApp {
    //debo hacer que aqui haga la llamada a la db para obtener los datos de los alumnos, y luego pasarlos a la tabla
    pub fn new() -> Self {
        let database = Database::new("./database/database.db").unwrap();
        let alumnos = database.fetch_all().unwrap();
        Self {
            alumnos: alumnos ,
            seleccionados: HashSet::new(),
            database,
        }
    }


pub fn toggle_seleccion(&mut self, id: usize) {
    if self.seleccionados.contains(&id) {
        self.seleccionados.remove(&id);
    } else {
        self.seleccionados.insert(id);
    }
}


pub fn toggle_all(&mut self) {
    if !self.seleccionados.is_empty() {
        self.seleccionados.clear();
    } else {
        self.seleccionados = self.alumnos.iter().map(|a| a.id).collect();
    }
}

}