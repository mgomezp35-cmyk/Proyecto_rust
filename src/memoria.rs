// memoria.rs — Un administrador de memoria muy simple
// La "RAM" es un arreglo de 1 KiB dividido en 64 bloques de 16 bytes.
use crate::println;

pub const TAM_BLOQUE: usize = 16;
pub const NUM_BLOQUES: usize = 64;

pub struct Memoria {
    datos: [u8; TAM_BLOQUE * NUM_BLOQUES], // la memoria en si
    dueno: [u8; NUM_BLOQUES],              // mapa: '.' = libre, letra = proceso dueno
    largo: [usize; NUM_BLOQUES],           // en el primer bloque: cuantos bloques ocupa
}

impl Memoria {
    pub fn new() -> Self {
        Memoria {
            datos: [0; TAM_BLOQUE * NUM_BLOQUES],
            dueno: [b'.'; NUM_BLOQUES],
            largo: [0; NUM_BLOQUES],
        }
    }

    /// Reserva memoria con PRIMER AJUSTE: el primer grupo de bloques libres seguidos.
    pub fn reservar(&mut self, nombre: u8, bytes: usize) -> Option<usize> {
        let n = (bytes + TAM_BLOQUE - 1) / TAM_BLOQUE; // bloques necesarios (redondeo hacia arriba)
        let mut inicio = 0;
        while inicio + n <= NUM_BLOQUES {
            if self.dueno[inicio..inicio + n].iter().all(|&d| d == b'.') {
                for i in inicio..inicio + n {
                    self.dueno[i] = nombre;
                }
                self.largo[inicio] = n;
                println!(
                    "{} pide {} bytes -> bloques {}..{} (desperdicio interno: {} bytes)",
                    nombre as char, bytes, inicio, inicio + n - 1, n * TAM_BLOQUE - bytes
                );
                return Some(inicio);
            }
            inicio += 1;
        }
        println!("{} pide {} bytes -> NO HAY ESPACIO CONTIGUO", nombre as char, bytes);
        None
    }

    /// Libera los bloques que empiezan en `inicio`.
    pub fn liberar(&mut self, inicio: usize) {
        let n = self.largo[inicio];
        let nombre = self.dueno[inicio];
        for i in inicio..inicio + n {
            self.dueno[i] = b'.';
        }
        self.largo[inicio] = 0;
        println!("{} libera {} bloques", nombre as char, n);
    }

    /// Escribe datos dentro de la region asignada. PROTECCION: no deja salirse del limite.
    pub fn escribir(&mut self, inicio: usize, desp: usize, datos: &[u8]) -> Result<(), &'static str> {
        let limite = self.largo[inicio] * TAM_BLOQUE;
        if desp + datos.len() > limite {
            return Err("fuera de limites");
        }
        let base = inicio * TAM_BLOQUE + desp; // direccion "fisica" = base + desplazamiento
        self.datos[base..base + datos.len()].copy_from_slice(datos);
        Ok(())
    }

    /// Lee `len` bytes de la region asignada.
    pub fn leer(&self, inicio: usize, desp: usize, len: usize) -> &[u8] {
        let base = inicio * TAM_BLOQUE + desp;
        &self.datos[base..base + len]
    }

    /// Muestra el mapa de memoria y cuanto espacio libre hay.
    pub fn mostrar(&self) {
        let mut libres = 0;
        let mut hueco = 0;
        let mut mayor = 0;
        for &d in self.dueno.iter() {
            if d == b'.' {
                libres += 1;
                hueco += 1;
                if hueco > mayor {
                    mayor = hueco;
                }
            } else {
                hueco = 0;
            }
        }
        println!("[{}]", core::str::from_utf8(&self.dueno).unwrap());
        println!("  libres: {} bloques ({} bytes), hueco mayor: {} bloques", libres, libres * TAM_BLOQUE, mayor);
    }
}
