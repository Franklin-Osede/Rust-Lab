//! 🦀 Performance Optimization - Bug Spotting Exercise
//! 
//! Este ejercicio demuestra conceptos de optimización de rendimiento en Rust
//! con bugs intencionales para practicar debugging.

use std::collections::HashMap;
use std::time::Instant;

/// Estructura que representa un usuario con datos
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    // BUG INTENCIONAL: Vec<String> en lugar de Vec<u32> para posts
    posts: Vec<String>,
    // BUG INTENCIONAL: HashMap innecesario para datos simples
    metadata: HashMap<String, String>,
}

impl User {
    fn new(id: u32, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            posts: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// BUG INTENCIONAL: Método ineficiente para añadir posts
    fn add_post(&mut self, post: String) {
        // BUG: Clonar String innecesariamente
        self.posts.push(post.clone());
        
        // BUG: Añadir metadata innecesario
        self.metadata.insert("last_post".to_string(), post);
    }
    
    /// BUG INTENCIONAL: Método ineficiente para buscar posts
    fn find_post(&self, query: &str) -> Option<&String> {
        // BUG: Búsqueda lineal ineficiente
        for post in &self.posts {
            if post.contains(query) {
                return Some(post);
            }
        }
        None
    }
    
    /// BUG INTENCIONAL: Método que causa allocations innecesarias
    fn get_all_posts(&self) -> Vec<String> {
        // BUG: Clonar todos los posts innecesariamente
        self.posts.clone()
    }
}

/// Función que demuestra problemas de performance con Vec
fn demonstrate_vec_performance_bugs() {
    println!("🔍 Demostrando bugs de performance con Vec...");
    
    let start = Instant::now();
    let mut users = Vec::new();
    
    // BUG: Crear usuarios de forma ineficiente
    for i in 0..1000 {
        let user = User::new(
            i,
            format!("User {}", i), // BUG: String allocation en cada iteración
            format!("user{}@example.com", i), // BUG: String allocation en cada iteración
        );
        users.push(user);
    }
    
    let duration = start.elapsed();
    println!("Tiempo para crear 1000 usuarios: {:?}", duration);
    
    // BUG: Búsqueda ineficiente
    let start = Instant::now();
    for user in &users {
        let _ = user.find_post("test"); // BUG: Búsqueda lineal
    }
    let duration = start.elapsed();
    println!("Tiempo para buscar en todos los usuarios: {:?}", duration);
}

/// Función que demuestra problemas con String allocations
fn demonstrate_string_allocation_bugs() {
    println!("\n🔍 Demostrando bugs de String allocations...");
    
    let start = Instant::now();
    let mut result = String::new();
    
    // BUG: Concatenación ineficiente
    for i in 0..1000 {
        result = result + &format!("Item {}, ", i); // BUG: Nueva String en cada iteración
    }
    
    let duration = start.elapsed();
    println!("Tiempo para concatenar 1000 strings: {:?}", duration);
    println!("Longitud del resultado: {}", result.len());
}

/// Función que demuestra problemas con HashMap
fn demonstrate_hashmap_performance_bugs() {
    println!("\n🔍 Demostrando bugs de performance con HashMap...");
    
    let start = Instant::now();
    let mut map = HashMap::new();
    
    // BUG: Insertar con String keys innecesarias
    for i in 0..10000 {
        let key = format!("key_{}", i); // BUG: String allocation
        let value = format!("value_{}", i); // BUG: String allocation
        map.insert(key, value);
    }
    
    let duration = start.elapsed();
    println!("Tiempo para insertar 10000 elementos: {:?}", duration);
    
    // BUG: Búsqueda ineficiente
    let start = Instant::now();
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let _ = map.get(&key); // BUG: String allocation para búsqueda
    }
    let duration = start.elapsed();
    println!("Tiempo para buscar 1000 elementos: {:?}", duration);
}

/// Función que demuestra problemas con clones innecesarios
fn demonstrate_clone_bugs() {
    println!("\n🔍 Demostrando bugs con clones innecesarios...");
    
    let start = Instant::now();
    let users = create_test_users(1000);
    
    // BUG: Clonar usuarios innecesariamente
    let mut processed_users = Vec::new();
    for user in &users {
        processed_users.push(user.clone()); // BUG: Clone innecesario
    }
    
    let duration = start.elapsed();
    println!("Tiempo para clonar 1000 usuarios: {:?}", duration);
    
    // BUG: Clonar datos innecesariamente
    let start = Instant::now();
    for user in &users {
        let _ = user.get_all_posts(); // BUG: Clone de todos los posts
    }
    let duration = start.elapsed();
    println!("Tiempo para obtener todos los posts: {:?}", duration);
}

/// Función que demuestra problemas con iteradores ineficientes
fn demonstrate_iterator_bugs() {
    println!("\n🔍 Demostrando bugs con iteradores ineficientes...");
    
    let users = create_test_users(1000);
    
    // BUG: Múltiples pasadas sobre los datos
    let start = Instant::now();
    
    // BUG: Primera pasada para contar
    let count = users.iter().count();
    
    // BUG: Segunda pasada para filtrar
    let filtered: Vec<_> = users.iter()
        .filter(|u| u.id % 2 == 0)
        .collect();
    
    // BUG: Tercera pasada para mapear
    let mapped: Vec<_> = filtered.iter()
        .map(|u| u.name.clone()) // BUG: Clone innecesario
        .collect();
    
    let duration = start.elapsed();
    println!("Tiempo para procesar usuarios (múltiples pasadas): {:?}", duration);
    println!("Usuarios procesados: {}", mapped.len());
}

/// Función que demuestra problemas con Box y heap allocations
fn demonstrate_heap_allocation_bugs() {
    println!("\n🔍 Demostrando bugs con heap allocations...");
    
    let start = Instant::now();
    
    // BUG: Box innecesario para datos pequeños
    let mut data = Vec::new();
    for i in 0..10000 {
        let boxed_int = Box::new(i); // BUG: Box innecesario
        data.push(boxed_int);
    }
    
    let duration = start.elapsed();
    println!("Tiempo para crear 10000 Box<i32>: {:?}", duration);
    
    // BUG: Vec<Box<T>> innecesario
    let start = Instant::now();
    let mut boxed_vecs = Vec::new();
    for i in 0..100 {
        let vec = Box::new(vec![i; 100]); // BUG: Box innecesario
        boxed_vecs.push(vec);
    }
    
    let duration = start.elapsed();
    println!("Tiempo para crear 100 Box<Vec<i32>>: {:?}", duration);
}

/// Función que demuestra problemas con recursión ineficiente
fn demonstrate_recursion_bugs() {
    println!("\n🔍 Demostrando bugs con recursión ineficiente...");
    
    let start = Instant::now();
    
    // BUG: Recursión ineficiente (sin memoización)
    let result = fibonacci_inefficient(35);
    
    let duration = start.elapsed();
    println!("Tiempo para fibonacci(35) ineficiente: {:?}", duration);
    println!("Resultado: {}", result);
}

/// Función que demuestra problemas con locks innecesarios
fn demonstrate_lock_bugs() {
    println!("\n🔍 Demostrando bugs con locks innecesarios...");
    
    use std::sync::{Arc, Mutex};
    
    let start = Instant::now();
    let data = Arc::new(Mutex::new(0));
    
    // BUG: Lock en cada iteración
    for i in 0..10000 {
        let data_clone = Arc::clone(&data);
        // BUG: Lock innecesario para operación simple
        if let Ok(mut value) = data_clone.lock() {
            *value += i;
        }
    }
    
    let duration = start.elapsed();
    println!("Tiempo para 10000 locks: {:?}", duration);
}

/// Función auxiliar para crear usuarios de prueba
fn create_test_users(count: usize) -> Vec<User> {
    let mut users = Vec::new();
    for i in 0..count {
        let user = User::new(
            i as u32,
            format!("User {}", i),
            format!("user{}@example.com", i),
        );
        users.push(user);
    }
    users
}

/// Función de Fibonacci ineficiente (sin memoización)
fn fibonacci_inefficient(n: u32) -> u64 {
    if n <= 1 {
        n as u64
    } else {
        fibonacci_inefficient(n - 1) + fibonacci_inefficient(n - 2)
    }
}

fn main() {
    println!("🦀 Rust Lab - Performance Optimization Bug Spotting");
    println!("{}", "=".repeat(60));
    
    // Ejecutar demostraciones
    demonstrate_vec_performance_bugs();
    demonstrate_string_allocation_bugs();
    demonstrate_hashmap_performance_bugs();
    demonstrate_clone_bugs();
    demonstrate_iterator_bugs();
    demonstrate_heap_allocation_bugs();
    demonstrate_recursion_bugs();
    demonstrate_lock_bugs();
    
    println!("\n✅ Ejercicio completado. Revisa los comentarios para entender los bugs.");
    println!("🔧 Usa 'cargo run --release' para ver diferencias de performance más claras");
}


