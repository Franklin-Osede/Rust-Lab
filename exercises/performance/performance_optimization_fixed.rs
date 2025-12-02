//! 🦀 Performance Optimization - SOLUCIÓN CORREGIDA
//! 
//! Esta es la versión corregida del ejercicio anterior,
//! mostrando las mejores prácticas de optimización en Rust.

use std::collections::HashMap;
use std::time::Instant;

/// Estructura que representa un usuario con datos optimizada
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    // CORREGIDO: Vec<u32> para IDs de posts (más eficiente)
    posts: Vec<u32>,
    // CORREGIDO: Solo metadata esencial
    last_post_id: Option<u32>,
}

impl User {
    fn new(id: u32, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            posts: Vec::new(),
            last_post_id: None,
        }
    }
    
    /// CORREGIDO: Método eficiente para añadir posts
    fn add_post(&mut self, post_id: u32) {
        self.posts.push(post_id);
        self.last_post_id = Some(post_id);
    }
    
    /// CORREGIDO: Método eficiente para buscar posts
    fn find_post(&self, post_id: u32) -> bool {
        // CORREGIDO: Búsqueda binaria para posts ordenados
        self.posts.binary_search(&post_id).is_ok()
    }
    
    /// CORREGIDO: Método que retorna referencias en lugar de clones
    fn get_posts(&self) -> &[u32] {
        &self.posts
    }
}

/// Función que demuestra optimización de Vec
fn demonstrate_vec_optimization() {
    println!("✅ Demostrando optimización de Vec...");
    
    let start = Instant::now();
    let mut users = Vec::with_capacity(1000); // CORREGIDO: Pre-allocar capacidad
    
    // CORREGIDO: Crear usuarios de forma eficiente
    for i in 0..1000 {
        let user = User::new(
            i,
            format!("User{}", i), // CORREGIDO: Menos allocations
            format!("user{}@example.com", i),
        );
        users.push(user);
    }
    
    let duration = start.elapsed();
    println!("Tiempo para crear 1000 usuarios: {:?}", duration);
    
    // CORREGIDO: Búsqueda eficiente
    let start = Instant::now();
    for user in &users {
        let _ = user.find_post(42); // CORREGIDO: Búsqueda binaria
    }
    let duration = start.elapsed();
    println!("Tiempo para buscar en todos los usuarios: {:?}", duration);
}

/// Función que demuestra optimización de String
fn demonstrate_string_optimization() {
    println!("\n✅ Demostrando optimización de String...");
    
    let start = Instant::now();
    let mut result = String::with_capacity(10000); // CORREGIDO: Pre-allocar capacidad
    
    // CORREGIDO: Usar push_str en lugar de concatenación
    for i in 0..1000 {
        result.push_str(&format!("Item{}, ", i));
    }
    
    let duration = start.elapsed();
    println!("Tiempo para concatenar 1000 strings: {:?}", duration);
    println!("Longitud del resultado: {}", result.len());
}

/// Función que demuestra optimización de HashMap
fn demonstrate_hashmap_optimization() {
    println!("\n✅ Demostrando optimización de HashMap...");
    
    let start = Instant::now();
    let mut map = HashMap::with_capacity(10000); // CORREGIDO: Pre-allocar capacidad
    
    // CORREGIDO: Usar u32 keys en lugar de String
    for i in 0..10000 {
        map.insert(i, i * 2); // CORREGIDO: Keys numéricas
    }
    
    let duration = start.elapsed();
    println!("Tiempo para insertar 10000 elementos: {:?}", duration);
    
    // CORREGIDO: Búsqueda eficiente
    let start = Instant::now();
    for i in 0..1000 {
        let _ = map.get(&i); // CORREGIDO: Búsqueda directa
    }
    let duration = start.elapsed();
    println!("Tiempo para buscar 1000 elementos: {:?}", duration);
}

/// Función que demuestra optimización de clones
fn demonstrate_clone_optimization() {
    println!("\n✅ Demostrando optimización de clones...");
    
    let start = Instant::now();
    let users = create_test_users_optimized(1000);
    
    // CORREGIDO: Usar referencias en lugar de clones
    let mut processed_count = 0;
    for user in &users {
        // CORREGIDO: Usar referencias en lugar de clones
        let _posts = user.get_posts();
        processed_count += 1;
    }
    
    let duration = start.elapsed();
    println!("Tiempo para procesar 1000 usuarios: {:?}", duration);
    println!("Usuarios procesados: {}", processed_count);
}

/// Función que demuestra optimización de iteradores
fn demonstrate_iterator_optimization() {
    println!("\n✅ Demostrando optimización de iteradores...");
    
    let users = create_test_users_optimized(1000);
    
    // CORREGIDO: Una sola pasada sobre los datos
    let start = Instant::now();
    
    let result: Vec<&str> = users.iter()
        .filter(|u| u.id % 2 == 0)
        .map(|u| &u.name[..]) // CORREGIDO: Referencia en lugar de clone
        .collect();
    
    let duration = start.elapsed();
    println!("Tiempo para procesar usuarios (una pasada): {:?}", duration);
    println!("Usuarios procesados: {}", result.len());
}

/// Función que demuestra optimización de heap allocations
fn demonstrate_heap_optimization() {
    println!("\n✅ Demostrando optimización de heap allocations...");
    
    let start = Instant::now();
    
    // CORREGIDO: Usar Vec directamente en lugar de Box
    let mut data = Vec::with_capacity(10000);
    for i in 0..10000 {
        data.push(i); // CORREGIDO: Sin Box innecesario
    }
    
    let duration = start.elapsed();
    println!("Tiempo para crear 10000 i32: {:?}", duration);
    
    // CORREGIDO: Vec<Vec<T>> en lugar de Vec<Box<Vec<T>>>
    let start = Instant::now();
    let mut vecs = Vec::with_capacity(100);
    for i in 0..100 {
        let vec = vec![i; 100]; // CORREGIDO: Sin Box innecesario
        vecs.push(vec);
    }
    
    let duration = start.elapsed();
    println!("Tiempo para crear 100 Vec<i32>: {:?}", duration);
}

/// Función que demuestra optimización de recursión
fn demonstrate_recursion_optimization() {
    println!("\n✅ Demostrando optimización de recursión...");
    
    let start = Instant::now();
    
    // CORREGIDO: Fibonacci con memoización
    let result = fibonacci_optimized(35);
    
    let duration = start.elapsed();
    println!("Tiempo para fibonacci(35) optimizado: {:?}", duration);
    println!("Resultado: {}", result);
}

/// Función que demuestra optimización de locks
fn demonstrate_lock_optimization() {
    println!("\n✅ Demostrando optimización de locks...");
    
    use std::sync::{Arc, Mutex};
    
    let start = Instant::now();
    let data = Arc::new(Mutex::new(0));
    
    // CORREGIDO: Lock una sola vez
    if let Ok(mut value) = data.lock() {
        for i in 0..10000 {
            *value += i;
        }
    }
    
    let duration = start.elapsed();
    println!("Tiempo para 10000 operaciones con un lock: {:?}", duration);
}

/// Función que demuestra optimización con Cow
fn demonstrate_cow_optimization() {
    println!("\n✅ Demostrando optimización con Cow...");
    
    use std::borrow::Cow;
    
    let start = Instant::now();
    
    // CORREGIDO: Usar Cow para evitar clones innecesarios
    let mut results = Vec::new();
    for i in 0..1000 {
        let value = if i % 2 == 0 {
            Cow::Borrowed("even")
        } else {
            Cow::Owned(format!("odd_{}", i))
        };
        results.push(value);
    }
    
    let duration = start.elapsed();
    println!("Tiempo para crear 1000 Cow: {:?}", duration);
    println!("Resultados: {}", results.len());
}

/// Función que demuestra optimización con slice patterns
fn demonstrate_slice_optimization() {
    println!("\n✅ Demostrando optimización con slice patterns...");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let start = Instant::now();
    
    // CORREGIDO: Usar slice patterns para procesamiento eficiente
    let mut sum = 0;
    for chunk in data.chunks(2) {
        match chunk {
            [a, b] => sum += a + b,
            [a] => sum += a,
            _ => {}
        }
    }
    
    let duration = start.elapsed();
    println!("Tiempo para procesar chunks: {:?}", duration);
    println!("Suma: {}", sum);
}

/// Función auxiliar para crear usuarios de prueba optimizada
fn create_test_users_optimized(count: usize) -> Vec<User> {
    let mut users = Vec::with_capacity(count);
    for i in 0..count {
        let user = User::new(
            i as u32,
            format!("User{}", i),
            format!("user{}@example.com", i),
        );
        users.push(user);
    }
    users
}

/// Función de Fibonacci optimizada con memoización
fn fibonacci_optimized(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    
    let mut memo = vec![0; (n + 1) as usize];
    memo[0] = 0;
    memo[1] = 1;
    
    for i in 2..=n as usize {
        memo[i] = memo[i - 1] + memo[i - 2];
    }
    
    memo[n as usize]
}

fn main() {
    println!("🦀 Rust Lab - Performance Optimization SOLUCIÓN CORRECTA");
    println!("{}", "=".repeat(70));
    
    demonstrate_vec_optimization();
    demonstrate_string_optimization();
    demonstrate_hashmap_optimization();
    demonstrate_clone_optimization();
    demonstrate_iterator_optimization();
    demonstrate_heap_optimization();
    demonstrate_recursion_optimization();
    demonstrate_lock_optimization();
    demonstrate_cow_optimization();
    demonstrate_slice_optimization();
    
    println!("\n✅ Todas las optimizaciones completadas!");
    println!("🎯 Conceptos clave demostrados:");
    println!("   - Pre-allocation: Reservar capacidad anticipadamente");
    println!("   - String optimization: Usar push_str en lugar de concatenación");
    println!("   - Iterator optimization: Una sola pasada sobre los datos");
    println!("   - Clone avoidance: Usar referencias cuando sea posible");
    println!("   - Memory layout: Estructuras de datos eficientes");
    println!("   - Algorithm optimization: Búsqueda binaria, memoización");
    println!("   - Lock optimization: Minimizar tiempo de lock");
    println!("   - Cow optimization: Copy-on-write para flexibilidad");
}


