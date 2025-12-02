//! 🦀 Ownership Basics - SOLUCIÓN CORREGIDA
//! 
//! Esta es la versión corregida del ejercicio anterior,
//! mostrando las mejores prácticas de ownership en Rust.

use std::collections::HashMap;

/// Estructura que representa un usuario con datos sensibles
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    // CORREGIDO: Vec<u32> para IDs de posts
    posts: Vec<u32>,
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
        // CORREGIDO: No hay conversión innecesaria
        self.posts.push(post_id);
    }
    
    /// Obtiene el nombre del usuario (referencia inmutable)
    fn get_name(&self) -> &str {
        &self.name
    }
    
    /// Obtiene el nombre como String (clona el valor)
    fn get_name_owned(&self) -> String {
        self.name.clone()
    }
    
    /// Obtiene el nombre moviendo el struct (consume el struct)
    fn take_name(mut self) -> String {
        // Mueve el String fuera del struct
        std::mem::take(&mut self.name)
    }
}

/// Función que demuestra ownership correcto
fn demonstrate_ownership_correct() {
    println!("✅ Demostrando ownership correcto...");
    
    // Crear un usuario
    let mut user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    
    // Añadir algunos posts
    user.add_post(101);
    user.add_post(102);
    
    println!("Usuario creado: {:?}", user);
    
    // Usar referencia inmutable para leer
    println!("Nombre del usuario: {}", user.get_name());
    println!("Email del usuario: {}", user.email);
    
    // Añadir más posts usando referencia mutable
    user.add_post(103);
    println!("Usuario después de añadir post: {:?}", user);
    
    // Si necesitamos mover el nombre, podemos clonarlo
    let name_clone = user.get_name_owned();
    println!("Nombre clonado: {}", name_clone);
    
    // El usuario original sigue siendo válido
    println!("Usuario original: {:?}", user);
}

/// Función que demuestra borrowing correcto
fn demonstrate_borrowing_correct() {
    println!("\n✅ Demostrando borrowing correcto...");
    
    let mut users = HashMap::new();
    users.insert(1, User::new(1, "Bob".to_string(), "bob@example.com".to_string()));
    users.insert(2, User::new(2, "Charlie".to_string(), "charlie@example.com".to_string()));
    
    // Usar referencias mutables de forma secuencial
    if let Some(user1) = users.get_mut(&1) {
        user1.add_post(201);
        println!("Usuario 1 actualizado: {:?}", user1);
    }
    
    if let Some(user2) = users.get_mut(&2) {
        user2.add_post(202);
        println!("Usuario 2 actualizado: {:?}", user2);
    }
    
    // Usar referencias inmutables para lectura
    for (id, user) in &users {
        println!("Usuario {}: {} ({})", id, user.get_name(), user.email);
    }
}

/// Función que demuestra lifetimes correctos
fn demonstrate_lifetime_correct() {
    println!("\n✅ Demostrando lifetimes correctos...");
    
    let text = String::from("Hello, World!");
    let result = get_first_word_safe(&text);
    
    println!("Primera palabra: {}", result);
    
    // Usar la referencia mientras el valor original existe
    println!("Texto completo: {}", text);
    println!("Primera palabra otra vez: {}", result);
}

/// Función que retorna una referencia con lifetime explícito
fn get_first_word_safe<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

/// Función que demuestra manejo correcto de ownership en funciones
fn process_users_correctly() {
    println!("\n✅ Procesando usuarios correctamente...");
    
    let mut users = vec![
        User::new(1, "Alice".to_string(), "alice@example.com".to_string()),
        User::new(2, "Bob".to_string(), "bob@example.com".to_string()),
    ];
    
    // Procesar usuarios con referencias
    for user in &mut users {
        user.add_post(100 + user.id);
        println!("Procesado: {} con {} posts", user.get_name(), user.posts.len());
    }
    
    // Usar usuarios después del procesamiento
    println!("Total de usuarios: {}", users.len());
    for user in &users {
        println!("- {}: {} posts", user.get_name(), user.posts.len());
    }
}

fn main() {
    println!("🦀 Rust Lab - Ownership & Borrowing SOLUCIÓN CORRECTA");
    println!("{}", "=".repeat(60));
    
    demonstrate_ownership_correct();
    demonstrate_borrowing_correct();
    demonstrate_lifetime_correct();
    process_users_correctly();
    
    println!("\n✅ Todas las demostraciones completadas sin errores de compilación!");
    println!("🎯 Conceptos clave demostrados:");
    println!("   - Ownership: quién posee los datos");
    println!("   - Borrowing: referencias inmutables y mutables");
    println!("   - Lifetimes: duración de las referencias");
    println!("   - Move semantics: transferencia de ownership");
}
