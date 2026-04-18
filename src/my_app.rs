
use std::collections::HashSet;
use crate::models::Alumno; 

pub struct MyApp {
    pub alumnos: Vec<Alumno>,
    pub seleccionados: HashSet<usize>,
}

impl MyApp {
    //debo hacer que aqui haga la llamada a la db para obtener los datos de los alumnos, y luego pasarlos a la tabla
    pub fn new(alumnos: Vec<Alumno>) -> Self {
        Self {
            alumnos,
            seleccionados: HashSet::new(),
        }
    }


pub fn toggle_seleccion(&mut self, id: usize) {
    if let Some(alumno) = self.alumnos.iter_mut().find(|a| a.id == id) {
        // Invertimos el estado del alumno
        alumno.seleccionado = !alumno.seleccionado;
        
        // Sincronizamos con el HashSet
        if alumno.seleccionado {
            self.seleccionados.insert(alumno.id as usize);
        } else {
            self.seleccionados.remove(&(alumno.id as usize));
        }
    }
}

pub fn toggle_all(&mut self) {
    if !self.seleccionados.is_empty() {
        // 1. Limpiamos el Set
        self.seleccionados.clear();
        // 2. IMPORTANTE: Actualizamos el booleano en cada alumno
        for alumno in self.alumnos.iter_mut() {
            alumno.seleccionado = false;
        }
    } else {
        // 1. Llenamos el Set con los IDs (o índices)
        for alumno in self.alumnos.iter_mut() {
            self.seleccionados.insert(alumno.id as usize);
            // 2. IMPORTANTE: Marcamos como seleccionado
            alumno.seleccionado = true;
        }
    }
}
}