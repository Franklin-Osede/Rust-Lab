//! 🦀 Ownership Basics - Bug Spotting Exercise
//! 
//! Este ejercicio demuestra conceptos fundamentales de ownership en Rust
//! con bugs intencionales para practicar debugging.

use std::collections::HashMap;

/// Estructura que representa un usuario con datos sensibles
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    // BUG INTENCIONAL: Vec<i32> en lugar de Vec<u32> para IDs
    posts: Vec<i32>,
}

impl User {
    /// Crea un nuevo usuario
    fn new(id: u32, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            posts: Vec::new(),
        }
    }
    
    /// Añade un post al usuario
    fn add_post(&mut self, post_id: u32) {
        // BUG INTENCIONAL: Conversión incorrecta de u32 a i32
        self.posts.push(post_id as i32);
    }
    
    /// Obtiene el nombre del usuario
    fn get_name(&self) -> &str {
        &self.name
    }
    
    /// BUG INTENCIONAL: Método que intenta mover el String
    fn take_name(self) -> String {
        // Este método consume el struct completo
        self.name
    }
}

/// Función que demuestra problemas de ownership
fn demonstrate_ownership_bugs() {
    println!("🔍 Demostrando bugs de ownership...");
    
    // Crear un usuario
    let mut user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    
    // Añadir algunos posts
    user.add_post(101);
    user.add_post(102);
    
    println!("Usuario creado: {:?}", user);
    
    // BUG: Intentar usar el usuario después de moverlo
    let name = user.take_name();
    println!("Nombre extraído: {}", name);
    
    // ESTE CÓDIGO CAUSARÁ ERROR DE COMPILACIÓN:
    // println!("Usuario después de take_name: {:?}", user);
    // println!("Email del usuario: {}", user.email);
    
    // BUG: Intentar modificar después de move
    // user.add_post(103); // ERROR: use after move
}

/// Función que demuestra problemas con referencias
fn demonstrate_borrowing_bugs() {
    println!("\n🔍 Demostrando bugs de borrowing...");
    
    let mut users = HashMap::new();
    users.insert(1, User::new(1, "Bob".to_string(), "bob@example.com".to_string()));
    users.insert(2, User::new(2, "Charlie".to_string(), "charlie@example.com".to_string()));
    
    // BUG: Múltiples referencias mutables
    let user1 = users.get_mut(&1).unwrap();
    let user2 = users.get_mut(&2).unwrap();
    
    // ESTE CÓDIGO CAUSARÁ ERROR DE COMPILACIÓN:
    // user1.add_post(201);
    // user2.add_post(202);
    
    // BUG: Referencia inmutable y mutable al mismo tiempo
    let user_ref = users.get(&1).unwrap();
    let user_mut = users.get_mut(&1).unwrap();
    
    // ESTE CÓDIGO CAUSARÁ ERROR DE COMPILACIÓN:
    // println!("Usuario: {:?}", user_ref);
    // user_mut.add_post(203);
}

/// Función que demuestra lifetime issues
fn demonstrate_lifetime_bugs() {
    println!("\n🔍 Demostrando bugs de lifetime...");
    
    let text = String::from("Hello, World!");
    let result = get_first_word(&text);
    
    println!("Primera palabra: {}", result);
    
    // BUG: Referencia que vive más que el valor
    let bad_reference = {
        let temp_string = String::from("Temporary");
        get_first_word(&temp_string)
        // temp_string se destruye aquí, pero bad_reference intenta usarla
    };
    
    // ESTE CÓDIGO CAUSARÁ ERROR DE COMPILACIÓN:
    // println!("Referencia inválida: {}", bad_reference);
}

/// Función que retorna una referencia con lifetime problemático
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main() {
    println!("🦀 Rust Lab - Ownership & Borrowing Bug Spotting");
    println!("{}", "=".repeat(50));
    
    // Ejecutar demostraciones (algunas compilarán, otras no)
    demonstrate_ownership_bugs();
    demonstrate_borrowing_bugs();
    demonstrate_lifetime_bugs();
    
    println!("\n✅ Ejercicio completado. Revisa los comentarios para entender los bugs.");
}
