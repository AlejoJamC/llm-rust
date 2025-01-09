use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // Escuchar en el puerto 7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor HTTP escuchando en http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Nueva conexión establecida!");

        // Leer la solicitud HTTP
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        // Imprimir la solicitud HTTP en la consola
        println!("Solicitud:\n{}", String::from_utf8_lossy(&buffer[..]));

        // Crear la respuesta HTTP
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
        stream.write(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}