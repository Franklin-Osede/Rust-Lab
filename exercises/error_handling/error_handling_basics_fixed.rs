//! 🦀 Error Handling Basics - SOLUCIÓN CORREGIDA
//! 
//! Esta es la versión corregida del ejercicio anterior,
//! mostrando las mejores prácticas de manejo de errores en Rust.

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

/// Estructura que representa un archivo de configuración
#[derive(Debug, Clone)]
struct Config {
    port: u16,
    host: String,
    timeout: u64,
    debug_level: String,
}

impl Config {
    /// Crea una nueva configuración
    fn new(port: u16, host: String, timeout: u64) -> Self {
        Self {
            port,
            host,
            timeout,
            debug_level: "info".to_string(),
        }
    }
    
    /// CORREGIDO: Método que valida el nivel de debug
    fn set_debug_level(&mut self, level: &str) -> Result<(), String> {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        
        if valid_levels.contains(&level) {
            self.debug_level = level.to_string();
            Ok(())
        } else {
            Err(format!("Nivel de debug inválido: {}. Niveles válidos: {:?}", level, valid_levels))
        }
    }
    
    /// CORREGIDO: Método que retorna el nivel de debug de forma segura
    fn get_debug_level(&self) -> &str {
        &self.debug_level
    }
}

/// Función que demuestra manejo correcto de Result
fn demonstrate_result_correct() {
    println!("✅ Demostrando manejo correcto de Result...");
    
    // CORREGIDO: Usar match en lugar de unwrap()
    let config = Config::new(8080, "localhost".to_string(), 30);
    
    // Manejo seguro del nivel de debug
    match config.set_debug_level("debug") {
        Ok(_) => println!("Nivel de debug configurado correctamente"),
        Err(e) => println!("Error al configurar debug: {}", e),
    }
    
    println!("Configuración: {:?}", config);
    println!("Debug level: {}", config.get_debug_level());
}

/// Función que demuestra manejo correcto de parseo
fn demonstrate_parsing_correct() {
    println!("\n✅ Demostrando manejo correcto de parseo...");
    
    let inputs = vec!["123", "not_a_number", "456", "invalid"];
    
    for input in inputs {
        match input.parse::<i32>() {
            Ok(value) => println!("'{}' parseado correctamente: {}", input, value),
            Err(e) => println!("Error al parsear '{}': {}", input, e),
        }
    }
}

/// Función que demuestra manejo correcto de archivos
fn demonstrate_file_handling_correct() {
    println!("\n✅ Demostrando manejo correcto de archivos...");
    
    // CORREGIDO: Manejo completo de Result
    match read_config_file("config.txt") {
        Ok(config) => {
            println!("Archivo leído correctamente:");
            println!("{}", config);
        }
        Err(e) => {
            println!("Error al leer archivo: {}", e);
            println!("Usando configuración por defecto...");
            let default_config = "port=8080\nhost=localhost\ntimeout=30";
            println!("Configuración por defecto: {}", default_config);
        }
    }
}

/// Función que lee archivo de configuración con manejo de errores
fn read_config_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Función que demuestra manejo correcto de Option
fn demonstrate_option_correct() {
    println!("\n✅ Demostrando manejo correcto de Option...");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // CORREGIDO: Verificar antes de usar
    while let Some(value) = numbers.pop() {
        println!("Valor extraído: {}", value);
    }
    
    // CORREGIDO: Verificar índice antes de acceder
    let numbers = vec![1, 2, 3, 4, 5];
    let index = 2;
    
    match numbers.get(index) {
        Some(value) => println!("Valor en índice {}: {}", index, value),
        None => println!("Índice {} fuera de rango", index),
    }
}

/// Función que demuestra propagación correcta de errores
fn demonstrate_error_propagation_correct() -> Result<String, Box<dyn std::error::Error>> {
    println!("\n✅ Demostrando propagación correcta de errores...");
    
    // CORREGIDO: Manejo completo de errores
    let content = read_config_file("config.txt")?;
    
    if content.trim().is_empty() {
        return Ok("Archivo de configuración vacío".to_string());
    }
    
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Ok("No hay líneas en el archivo".to_string());
    }
    
    // CORREGIDO: Validar que la primera línea sea un número
    let first_line = lines[0];
    let port: u16 = first_line.parse()
        .map_err(|e| format!("Error al parsear puerto '{}': {}", first_line, e))?;
    
    Ok(format!("Puerto configurado: {}", port))
}

/// Función que demuestra custom errors correctos
fn demonstrate_custom_error_correct() {
    println!("\n✅ Demostrando custom errors correctos...");
    
    let test_ports = vec!["8080", "99999", "0", "65536", "abc"];
    
    for port_str in test_ports {
        match validate_port_safe(port_str) {
            Ok(port) => println!("Puerto '{}' válido: {}", port_str, port),
            Err(e) => println!("Puerto '{}' inválido: {}", port_str, e),
        }
    }
}

/// Función que valida un puerto de forma segura
fn validate_port_safe(port_str: &str) -> Result<u16, String> {
    // CORREGIDO: Validación completa
    let port: u16 = port_str.parse()
        .map_err(|_| format!("'{}' no es un número válido", port_str))?;
    
    if port == 0 {
        return Err("Puerto no puede ser 0".to_string());
    }
    
    if port > 65535 {
        return Err("Puerto no puede ser mayor a 65535".to_string());
    }
    
    Ok(port)
}

/// Función que demuestra manejo correcto de panic recovery
fn demonstrate_panic_recovery_correct() {
    println!("\n✅ Demostrando manejo correcto de panic recovery...");
    
    // CORREGIDO: Usar catch_unwind apropiadamente
    let result = std::panic::catch_unwind(|| {
        // Código que puede causar panic
        let numbers = vec![1, 2, 3];
        numbers[2] // Acceso seguro
    });
    
    match result {
        Ok(value) => println!("Operación exitosa: {}", value),
        Err(_) => {
            println!("Panic capturado, manejando graciosamente...");
            // CORREGIDO: Recuperación apropiada
            println!("Usando valor por defecto: 0");
        }
    }
}

/// Función que demuestra error handling con tipos personalizados
fn demonstrate_custom_error_types() {
    println!("\n✅ Demostrando tipos de error personalizados...");
    
    #[derive(Debug)]
    enum ConfigError {
        InvalidPort(String),
        InvalidHost(String),
        FileNotFound(String),
        ParseError(String),
    }
    
    impl std::fmt::Display for ConfigError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ConfigError::InvalidPort(port) => write!(f, "Puerto inválido: {}", port),
                ConfigError::InvalidHost(host) => write!(f, "Host inválido: {}", host),
                ConfigError::FileNotFound(file) => write!(f, "Archivo no encontrado: {}", file),
                ConfigError::ParseError(msg) => write!(f, "Error de parseo: {}", msg),
            }
        }
    }
    
    impl std::error::Error for ConfigError {}
    
    // Función que puede fallar con nuestro tipo de error
    fn load_config(port_str: &str, host: &str) -> Result<Config, ConfigError> {
        let port: u16 = port_str.parse()
            .map_err(|_| ConfigError::ParseError(format!("No se pudo parsear puerto: {}", port_str)))?;
        
        if port == 0 {
            return Err(ConfigError::InvalidPort("Puerto no puede ser 0".to_string()));
        }
        
        if host.is_empty() {
            return Err(ConfigError::InvalidHost("Host no puede estar vacío".to_string()));
        }
        
        Ok(Config::new(port, host.to_string(), 30))
    }
    
    // Probar la función
    match load_config("8080", "localhost") {
        Ok(config) => println!("Configuración cargada: {:?}", config),
        Err(e) => println!("Error al cargar configuración: {}", e),
    }
    
    match load_config("0", "localhost") {
        Ok(config) => println!("Configuración cargada: {:?}", config),
        Err(e) => println!("Error al cargar configuración: {}", e),
    }
}

fn main() {
    println!("🦀 Rust Lab - Error Handling SOLUCIÓN CORRECTA");
    println!("{}", "=".repeat(60));
    
    demonstrate_result_correct();
    demonstrate_parsing_correct();
    demonstrate_file_handling_correct();
    demonstrate_option_correct();
    
    // Manejar errores de propagación
    if let Err(e) = demonstrate_error_propagation_correct() {
        println!("Error en propagación: {}", e);
    }
    
    demonstrate_custom_error_correct();
    demonstrate_panic_recovery_correct();
    demonstrate_custom_error_types();
    
    println!("\n✅ Todas las demostraciones completadas sin errores!");
    println!("🎯 Conceptos clave demostrados:");
    println!("   - Result<T, E>: Manejo explícito de errores");
    println!("   - Option<T>: Valores opcionales");
    println!("   - Error propagation: Propagación de errores");
    println!("   - Custom errors: Tipos de error personalizados");
    println!("   - Panic recovery: Recuperación de panics");
}

