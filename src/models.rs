use crate::utils;
use chrono::{Datelike, Local, NaiveDate};
use rusqlite;



#[derive(PartialEq, Clone)]
pub struct Alumno {
    pub id: usize,
    pub nombre: String,
    pub rango: u32,
    pub edad: i32,
    pub fecha_de_nacimiento: String,
    pub representante: String,
    pub numero_contacto: String,
    pub seleccionado: bool,
}

impl Alumno {
    // Un constructor sencillo
    pub fn new(
        nombre: &str,
        fecha_de_nacimiento: &str,
        rango: u32,
        representante: &str,
        numero_contacto: &str,
    ) -> Self {
        Self {
            id: 0,
            nombre: nombre.to_string(),
            fecha_de_nacimiento: fecha_de_nacimiento.to_string(),
            rango: rango as u32,
            representante: representante.to_string(),
            numero_contacto: numero_contacto.to_string(),
            edad: 1, //es provicional, despues lo calculare a partir de la fecha de nacimiento
            seleccionado: false,
        }
    }

    pub fn from_db(
        id: i32,
        nombre: &str,
        fecha_de_nacimiento: &str,
        rango: u32,
        representante: &str,
        numero_contacto: &str,
    ) -> Self {
        Self {
            id: id as usize,
            nombre: nombre.to_string(),
            fecha_de_nacimiento: fecha_de_nacimiento.to_string(),
            rango,
            representante: representante.to_string(),
            numero_contacto: numero_contacto.to_string(),
            edad: 1, //es provicional, despues lo calculare a partir de la fecha de nacimiento
            seleccionado: false, //esto no deberia estar aqui, se debe calcular directamente mediante el hashset porque contiene los ids de los seleccionados
        }
    }


    
    pub fn cinta(&self) -> String {
        //utils::obtener_nombre_cinta(self.rango)
        todo!("Implementar lógica de cinta basada en el rango")
    }
    pub fn rango_str(&self) -> String {
        /*let tiene_rayita = (self.rango - self.rango.floor()).abs() > 0.1;

        if !tiene_rayita && self.rango > 0.0 {
            format!("{} kyu", self.rango.floor() as i32)
        } else if tiene_rayita {
            format!("{} kyu b", self.rango.floor() as i32)
        } else {
            format!("{} Dan", (self.rango.abs() as i32) + 1)
        }*/
        todo!("Implementar lógica de rango_str basada en el rango tomando en cuenta que de ahora en adelante se usaran enteros :)")
    }

    pub fn edad(&self) -> String {
        let fecha_actual = Local::now().naive_local().date();
        let edad = if let Ok(nac) = NaiveDate::parse_from_str(&self.fecha_de_nacimiento, "%Y-%m-%d")
        {
            let mut años = fecha_actual.year() - nac.year();
            if fecha_actual.month() < nac.month()
                || (fecha_actual.month() == nac.month() && fecha_actual.day() < nac.day())
            {
                años -= 1;
            }
            años.to_string()
        } else {
            "??".to_string()
        };
        edad
    }
}
pub struct Database {
    connection: rusqlite::Connection,
}
impl Database {
    pub fn new(path: &str) -> rusqlite::Result<Self> {
        // 1. Intentamos crear el directorio y capturamos si hay un error real de sistema
        if let Err(e) = std::fs::create_dir_all("database") {
            eprintln!("Error creando carpeta database: {}", e);
        }

        // 2. Abrimos la conexión
        let connection = rusqlite::Connection::open(path)?;

        let db = Self { connection };

        // 3. ¡IMPORTANTE! Asegúrate de llamar aquí a la inicialización de tablas
        // Si no, aunque el archivo se cree, estará vacío y las consultas fallarán.
        db.inicializar_tablas()?;

        Ok(db)
    }

    fn inicializar_tablas(&self) -> rusqlite::Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS alumnos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            fecha_de_nacimiento TEXT NOT NULL,
            rango REAL NOT NULL,
            representante TEXT NOT NULL,
            numero_contacto TEXT NOT NULL
        )",
            [],
        )?;
        Ok(())
    }

    pub fn save(&self, alumno: &Alumno) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT INTO alumnos (nombre, fecha_de_nacimiento, rango, representante, numero_contacto) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                alumno.nombre,
                alumno.fecha_de_nacimiento,
                alumno.rango,
                alumno.representante,
                alumno.numero_contacto
            ],
        )?;
        Ok(())
    }

    pub fn fetch_all(&self) -> rusqlite::Result<Vec<Alumno>> {
        let mut stmt = self.connection.prepare(
        "SELECT id, nombre, fecha_de_nacimiento, rango, representante, numero_contacto FROM alumnos"
    )?;

        let alumno_iter = stmt.query_map([], |row| {
            // Corrección: Pasamos los valores directamente sin los nombres de los parámetros
            // y eliminamos el punto y coma final para que retorne el objeto Alumno
            Ok(Alumno::from_db(
                row.get(0)?,
                &row.get::<_, String>(1)?, // Usamos & porque from_db pide &str
                &row.get::<_, String>(2)?,
                row.get(3)?,
                &row.get::<_, String>(4)?,
                &row.get::<_, String>(5)?,
            ))
        })?; // Aquí faltaba el paréntesis de cierre

        let mut alumnos = Vec::new();
        for alumno in alumno_iter {
            alumnos.push(alumno?);
        }
        Ok(alumnos)
    }

    pub fn update(&self, alumno: &Alumno) -> rusqlite::Result<()> {
        self.connection.execute(
            "UPDATE alumnos SET nombre = ?1, fecha_de_nacimiento = ?2, rango = ?3, representante = ?4, numero_contacto = ?5 WHERE id = ?6",
            rusqlite::params![
                alumno.nombre,
                alumno.fecha_de_nacimiento,
                alumno.rango,
                alumno.representante,
                alumno.numero_contacto,
                alumno.id
            ],
        )?;
        Ok(())
    }

    pub fn delete(&self, alumno_id: i32) -> rusqlite::Result<()> {
        self.connection.execute(
            "DELETE FROM alumnos WHERE id = ?1",
            rusqlite::params![alumno_id],
        )?;
        Ok(())
    }

    pub fn get_alumno_by_id(&self, id: i32) -> rusqlite::Result<Alumno> {
    self.connection.query_row(
        "SELECT id, nombre, fecha_de_nacimiento, rango, representante, numero_contacto FROM alumnos WHERE id = ?1",
        rusqlite::params![id],
        |row| {
            Ok(Alumno::from_db(
                row.get(0)?,
                &row.get::<_, String>(1)?,
                &row.get::<_, String>(2)?,
                row.get(3)?,
                &row.get::<_, String>(4)?,
                &row.get::<_, String>(5)?,
            ))
        },
    )
}
}