#![no_std]
#![no_main]
mod memoria;
mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // evita que el compilador renombre la funcion
pub extern "C" fn _start() -> ! {
    println!("so1_kernel UMG - Asignacion de memoria simple");
    let mut m = memoria::Memoria::new();
    m.mostrar();

    // 1. Tres procesos piden memoria
    let a = m.reservar(b'A', 40).unwrap();
    let b = m.reservar(b'B', 100).unwrap();
    let c = m.reservar(b'C', 20).unwrap();
    m.mostrar();

    // 2. Escribir y leer
    m.escribir(a, 0, b"Hola UMG").unwrap();
    println!("A contiene: {}", core::str::from_utf8(m.leer(a, 0, 8)).unwrap());

    // 3. Proteccion: C intenta escribir fuera de su region
    match m.escribir(c, 30, b"desborde!") {
        Ok(_) => println!("Escritura en C aceptada"),
        Err(e) => println!("Escritura en C rechazada: {}", e),
    }

    // 4. Liberar B deja un hueco; D lo reutiliza (primer ajuste)
    m.liberar(b);
    m.mostrar();
    let _d = m.reservar(b'D', 50);
    let _e = m.reservar(b'E', 700);
    m.mostrar();

    // 5. Fragmentacion externa: hay espacio total, pero no contiguo
    let _f = m.reservar(b'F', 150);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
