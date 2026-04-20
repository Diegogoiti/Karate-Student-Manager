import sqlite3
import random
from datetime import datetime, timedelta

def generar_fecha():
    # Genera fechas para alumnos entre 5 y 25 años
    start_date = datetime.now() - timedelta(days=25*365)
    end_date = datetime.now() - timedelta(days=5*365)
    random_date = start_date + (end_date - start_date) * random.random()
    return random_date.strftime("%Y-%m-%d")

def llenar_db():
    # Conexión a tu archivo (asegúrate de que la ruta sea correcta)
    try:
        conn = sqlite3.connect('./database/database.db')
    except sqlite3.Error as e:
        print(f"Error al conectar con la base de datos: {e}")
        

    cursor = conn.cursor()

    nombres = ["Juan", "Maria", "Pedro", "Ana", "Luis", "Carmen", "Diego", "Elena", "Carlos", "Sofia"]
    apellidos = ["Garcia", "Rodriguez", "Perez", "Martinez", "Lopez", "Sanchez", "Gonzalez", "Diaz"]
    rangos = [1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0] # Tus niveles de cinta

    print("Insertando 100 alumnos de prueba...")

    for i in range(100):
        nombre_completo = f"{random.choice(nombres)} {random.choice(apellidos)} {i}"
        fecha = generar_fecha()
        rango = int(random.choice(rangos) * 10)
        rep = f"Padre de {nombre_completo.split()[0]}"
        tlf = f"0414-{random.randint(1000000, 9999999)}"

        cursor.execute('''
            INSERT INTO alumnos (nombre, fecha_de_nacimiento, rango, representante, numero_contacto)
            VALUES (?, ?, ?, ?, ?)
        ''', (nombre_completo, fecha, rango, rep, tlf))

    conn.commit()
    conn.close()
    print("¡Listo! 100 alumnos agregados con éxito.")

if __name__ == "__main__":
    llenar_db()