//! Tests para los ejercicios de error handling

#[cfg(test)]
mod error_handling_tests {
    use std::fs::File;
    use std::io::{self, Write};
    use std::num::ParseIntError;
    
    // Importar las estructuras del ejercicio
    #[derive(Debug, Clone, PartialEq)]
    struct Config {
        port: u16,
        host: String,
        timeout: u64,
        debug_level: String,
    }
    
    impl Config {
        fn new(port: u16, host: String, timeout: u64) -> Self {
            Self {
                port,
                host,
                timeout,
                debug_level: "info".to_string(),
            }
        }
        
        fn set_debug_level(&mut self, level: &str) -> Result<(), String> {
            let valid_levels = ["trace", "debug", "info", "warn", "error"];
            
            if valid_levels.contains(&level) {
                self.debug_level = level.to_string();
                Ok(())
            } else {
                Err(format!("Nivel de debug inválido: {}. Niveles válidos: {:?}", level, valid_levels))
            }
        }
        
        fn get_debug_level(&self) -> &str {
            &self.debug_level
        }
    }
    
    #[test]
    fn test_config_creation() {
        let config = Config::new(8080, "localhost".to_string(), 30);
        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "localhost");
        assert_eq!(config.timeout, 30);
        assert_eq!(config.debug_level, "info");
    }
    
    #[test]
    fn test_debug_level_validation() {
        let mut config = Config::new(8080, "localhost".to_string(), 30);
        
        // Test nivel válido
        assert!(config.set_debug_level("debug").is_ok());
        assert_eq!(config.get_debug_level(), "debug");
        
        // Test nivel inválido
        assert!(config.set_debug_level("invalid").is_err());
        assert_eq!(config.get_debug_level(), "debug"); // No cambió
        
        // Test todos los niveles válidos
        for level in ["trace", "debug", "info", "warn", "error"] {
            assert!(config.set_debug_level(level).is_ok());
            assert_eq!(config.get_debug_level(), level);
        }
    }
    
    #[test]
    fn test_parsing_errors() {
        // Test parseo exitoso
        assert_eq!("123".parse::<i32>().unwrap(), 123);
        
        // Test parseo con error
        assert!("not_a_number".parse::<i32>().is_err());
        assert!("".parse::<i32>().is_err());
        assert!("12.34".parse::<i32>().is_err());
    }
    
    #[test]
    fn test_file_handling() {
        // Test archivo que no existe
        let result = File::open("archivo_inexistente.txt");
        assert!(result.is_err());
        
        // Test archivo que existe
        let temp_file = "test_config.txt";
        let mut file = File::create(temp_file).unwrap();
        file.write_all(b"port=8080\nhost=localhost\ntimeout=30").unwrap();
        drop(file);
        
        let result = File::open(temp_file);
        assert!(result.is_ok());
        
        // Limpiar archivo temporal
        std::fs::remove_file(temp_file).unwrap();
    }
    
    #[test]
    fn test_option_handling() {
        let mut numbers = vec![1, 2, 3, 4, 5];
        
        // Test pop exitoso
        assert_eq!(numbers.pop(), Some(5));
        assert_eq!(numbers.pop(), Some(4));
        
        // Test pop en vector vacío
        numbers.clear();
        assert_eq!(numbers.pop(), None);
        
        // Test get con índice válido
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(numbers.get(0), Some(&1));
        assert_eq!(numbers.get(4), Some(&5));
        
        // Test get con índice inválido
        assert_eq!(numbers.get(5), None);
        assert_eq!(numbers.get(10), None);
    }
    
    #[test]
    fn test_error_propagation() {
        // Test función que puede fallar
        fn parse_port(port_str: &str) -> Result<u16, ParseIntError> {
            port_str.parse()
        }
        
        // Test parseo exitoso
        assert_eq!(parse_port("8080"), Ok(8080));
        
        // Test parseo con error
        assert!(parse_port("not_a_number").is_err());
        assert!(parse_port("").is_err());
    }
    
    #[test]
    fn test_custom_error_types() {
        #[derive(Debug, PartialEq)]
        enum ConfigError {
            InvalidPort(String),
            InvalidHost(String),
            ParseError(String),
        }
        
        impl std::fmt::Display for ConfigError {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    ConfigError::InvalidPort(port) => write!(f, "Puerto inválido: {}", port),
                    ConfigError::InvalidHost(host) => write!(f, "Host inválido: {}", host),
                    ConfigError::ParseError(msg) => write!(f, "Error de parseo: {}", msg),
                }
            }
        }
        
        impl std::error::Error for ConfigError {}
        
        fn validate_config(port_str: &str, host: &str) -> Result<Config, ConfigError> {
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
        
        // Test configuración válida
        let result = validate_config("8080", "localhost");
        assert!(result.is_ok());
        
        // Test puerto inválido
        let result = validate_config("0", "localhost");
        assert!(result.is_err());
        if let Err(ConfigError::InvalidPort(_)) = result {
            // Error esperado
        } else {
            panic!("Error inesperado");
        }
        
        // Test host vacío
        let result = validate_config("8080", "");
        assert!(result.is_err());
        if let Err(ConfigError::InvalidHost(_)) = result {
            // Error esperado
        } else {
            panic!("Error inesperado");
        }
        
        // Test parseo de puerto
        let result = validate_config("not_a_number", "localhost");
        assert!(result.is_err());
        if let Err(ConfigError::ParseError(_)) = result {
            // Error esperado
        } else {
            panic!("Error inesperado");
        }
    }
    
    #[test]
    fn test_panic_recovery() {
        // Test panic recovery
        let result = std::panic::catch_unwind(|| {
            // Código que no causa panic
            42
        });
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        
        // Test panic recovery con panic
        let result = std::panic::catch_unwind(|| {
            panic!("Panic intencional");
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_result_combinators() {
        // Test map
        let result: Result<i32, &str> = Ok(42);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped, Ok(84));
        
        // Test map_err
        let result: Result<i32, &str> = Err("error");
        let mapped = result.map_err(|e| format!("Error: {}", e));
        assert_eq!(mapped, Err("Error: error".to_string()));
        
        // Test and_then
        let result: Result<i32, &str> = Ok(42);
        let chained = result.and_then(|x| if x > 0 { Ok(x * 2) } else { Err("negative") });
        assert_eq!(chained, Ok(84));
        
        // Test or_else
        let result: Result<i32, &str> = Err("error");
        let recovered = result.or_else(|_| Ok(0));
        assert_eq!(recovered, Ok(0));
    }
    
    #[test]
    fn test_option_combinators() {
        // Test map
        let option: Option<i32> = Some(42);
        let mapped = option.map(|x| x * 2);
        assert_eq!(mapped, Some(84));
        
        // Test and_then
        let option: Option<i32> = Some(42);
        let chained = option.and_then(|x| if x > 0 { Some(x * 2) } else { None });
        assert_eq!(chained, Some(84));
        
        // Test or_else
        let option: Option<i32> = None;
        let recovered = option.or_else(|| Some(0));
        assert_eq!(recovered, Some(0));
        
        // Test unwrap_or
        let option: Option<i32> = None;
        let value = option.unwrap_or(42);
        assert_eq!(value, 42);
        
        // Test unwrap_or_else
        let option: Option<i32> = None;
        let value = option.unwrap_or_else(|| 42);
        assert_eq!(value, 42);
    }
}


