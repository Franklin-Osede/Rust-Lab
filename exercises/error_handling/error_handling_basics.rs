//! 🦀 Error Handling Basics - Bug Spotting Exercise
//! 
//! Este ejercicio demuestra conceptos de manejo de errores en Rust
//! con bugs intencionales para practicar debugging.

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

/// Estructura que representa un archivo de configuración
#[derive(Debug)]
struct Config {
    port: u16,
    host: String,
    timeout: u64,
    // BUG INTENCIONAL: Option<String> en lugar de String para debug
    debug_level: Option<String>,
}

impl Config {
    /// Crea una nueva configuración
    fn new(port: u16, host: String, timeout: u64) -> Self {
        Self {
            port,
            host,
            timeout,
            debug_level: None,
        }
    }
    
    /// BUG INTENCIONAL: Método que puede fallar sin manejo de errores
    fn set_debug_level(&mut self, level: &str) {
        // BUG: No valida que el nivel sea válido
        self.debug_level = Some(level.to_string());
    }
    
    /// BUG INTENCIONAL: Método que puede causar panic
    fn get_debug_level(&self) -> &str {
        // BUG: Unwrap sin verificar si es Some
        self.debug_level.as_ref().unwrap()
    }
}

/// Función que demuestra problemas con unwrap()
fn demonstrate_unwrap_bugs() {
    println!("🔍 Demostrando bugs con unwrap()...");
    
    // BUG: Usar unwrap() sin verificar
    let config = Config::new(8080, "localhost".to_string(), 30);
    
    // ESTE CÓDIGO CAUSARÁ PANIC:
    // println!("Debug level: {}", config.get_debug_level());
    
    // BUG: Intentar parsear un número inválido
    let invalid_number = "not_a_number";
    // ESTE CÓDIGO CAUSARÁ PANIC:
    // let parsed: i32 = invalid_number.parse().unwrap();
    
    println!("Configuración creada: {:?}", config);
}

/// Función que demuestra problemas con expect()
fn demonstrate_expect_bugs() {
    println!("\n🔍 Demostrando bugs con expect()...");
    
    // BUG: Usar expect() con mensaje genérico
    let result = "maybe_a_number".parse::<i32>();
    match result {
        Ok(value) => println!("Número parseado: {}", value),
        Err(e) => {
            // BUG: Solo imprimir el error sin manejo
            println!("Error: {}", e);
            // ESTE CÓDIGO CAUSARÍA PANIC:
            // let _ = result.expect("Número inválido");
        }
    }
}

/// Función que demuestra problemas con Result
fn demonstrate_result_bugs() {
    println!("\n🔍 Demostrando bugs con Result...");
    
    // BUG: No manejar el Result correctamente
    let file_result = File::open("archivo_inexistente.txt");
    
    // ESTE CÓDIGO CAUSARÍA PANIC:
    // let _file = file_result.unwrap();
    
    // BUG: Manejo de error incompleto
    match file_result {
        Ok(_) => println!("Archivo abierto correctamente"),
        Err(e) => {
            // BUG: Solo imprimir el error sin recuperación
            println!("Error al abrir archivo: {}", e);
        }
    }
}

/// Función que demuestra problemas con Option
fn demonstrate_option_bugs() {
    println!("\n🔍 Demostrando bugs con Option...");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // BUG: Usar unwrap() sin verificar
    // ESTE CÓDIGO CAUSARÍA PANIC SI EL VECTOR ESTÁ VACÍO:
    // let first = numbers.pop().unwrap();
    
    // BUG: No verificar si el índice existe
    let index = 10;
    // ESTE CÓDIGO CAUSARÍA PANIC:
    // let value = numbers[index];
    
    println!("Vector: {:?}", numbers);
}

/// Función que demuestra problemas con propagación de errores
fn demonstrate_error_propagation_bugs() -> Result<String, Box<dyn std::error::Error>> {
    println!("\n🔍 Demostrando bugs de propagación de errores...");
    
    // BUG: Función que puede fallar pero no maneja todos los casos
    let content = read_file_content("config.txt")?;
    
    // BUG: Asumir que el archivo siempre tiene contenido
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        // BUG: No manejar el caso de archivo vacío
        return Ok("Archivo vacío".to_string());
    }
    
    // BUG: Asumir que la primera línea siempre es válida
    let first_line = lines[0];
    let port: u16 = first_line.parse()?;
    
    Ok(format!("Puerto configurado: {}", port))
}

/// Función auxiliar que lee contenido de archivo
fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Función que demuestra problemas con custom errors
fn demonstrate_custom_error_bugs() {
    println!("\n🔍 Demostrando bugs con custom errors...");
    
    // BUG: No definir un tipo de error personalizado
    let result = validate_port("8080");
    match result {
        Ok(port) => println!("Puerto válido: {}", port),
        Err(e) => {
            // BUG: No manejar diferentes tipos de error
            println!("Error: {}", e);
        }
    }
    
    // BUG: Intentar validar puerto inválido
    let invalid_result = validate_port("99999");
    match invalid_result {
        Ok(port) => println!("Puerto válido: {}", port),
        Err(e) => {
            // BUG: No recuperación del error
            println!("Error: {}", e);
        }
    }
}

/// Función que valida un puerto (con bugs)
fn validate_port(port_str: &str) -> Result<u16, String> {
    // BUG: No validar formato antes de parsear
    let port: u16 = port_str.parse()
        .map_err(|_| "Puerto inválido".to_string())?;
    
    // BUG: Validación incompleta
    if port == 0 {
        return Err("Puerto no puede ser 0".to_string());
    }
    
    // BUG: No validar rango máximo
    Ok(port)
}

/// Función que demuestra problemas con panic recovery
fn demonstrate_panic_recovery_bugs() {
    println!("\n🔍 Demostrando bugs con panic recovery...");
    
    // BUG: No usar std::panic::catch_unwind para funciones que pueden panic
    let result = std::panic::catch_unwind(|| {
        // Código que puede causar panic
        let numbers = vec![1, 2, 3];
        numbers[10] // Esto causará panic
    });
    
    match result {
        Ok(_) => println!("Operación exitosa"),
        Err(_) => {
            // BUG: No manejar el panic apropiadamente
            println!("Panic capturado, pero no se maneja correctamente");
        }
    }
}

fn main() {
    println!("🦀 Rust Lab - Error Handling Bug Spotting");
    println!("{}", "=".repeat(50));
    
    // Ejecutar demostraciones (algunas compilarán, otras no)
    demonstrate_unwrap_bugs();
    demonstrate_expect_bugs();
    demonstrate_result_bugs();
    demonstrate_option_bugs();
    
    // Estas funciones pueden fallar
    if let Err(e) = demonstrate_error_propagation_bugs() {
        println!("Error en propagación: {}", e);
    }
    
    demonstrate_custom_error_bugs();
    demonstrate_panic_recovery_bugs();
    
    println!("\n✅ Ejercicio completado. Revisa los comentarios para entender los bugs.");
}

