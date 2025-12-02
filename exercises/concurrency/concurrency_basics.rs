//! 🦀 Concurrency Basics - Bug Spotting Exercise
//! 
//! Este ejercicio demuestra conceptos de concurrencia en Rust
//! con bugs intencionales para practicar debugging.

use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc;
use std::time::Duration;

/// Estructura que representa un contador compartido
#[derive(Debug)]
struct Counter {
    value: i32,
    // BUG INTENCIONAL: Mutex<i32> en lugar de Arc<Mutex<i32>>
    // Esto causará problemas de ownership
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }
    
    /// BUG INTENCIONAL: Método que no maneja el Mutex correctamente
    fn increment(&mut self) {
        self.value += 1;
    }
    
    /// BUG INTENCIONAL: Método que no maneja el Mutex correctamente
    fn get_value(&self) -> i32 {
        self.value
    }
}

/// Función que demuestra problemas con threads
fn demonstrate_thread_bugs() {
    println!("🔍 Demostrando bugs con threads...");
    
    let mut counter = Counter::new();
    
    // BUG: Intentar compartir counter entre threads
    // ESTE CÓDIGO CAUSARÁ ERROR DE COMPILACIÓN:
    // let handle1 = thread::spawn(move || {
    //     counter.increment();
    // });
    // let handle2 = thread::spawn(move || {
    //     counter.increment();
    // });
    
    println!("Counter inicial: {:?}", counter);
}

/// Función que demuestra problemas con Arc y Mutex
fn demonstrate_arc_mutex_bugs() {
    println!("\n🔍 Demostrando bugs con Arc y Mutex...");
    
    let counter = Arc::new(Mutex::new(Counter::new()));
    let mut handles = vec![];
    
    // BUG: No manejar el Result del lock
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // BUG: Unwrap sin verificar
            let mut counter_guard = counter_clone.lock().unwrap();
            counter_guard.increment();
            println!("Thread {} incrementó el contador", i);
        });
        handles.push(handle);
    }
    
    // BUG: No esperar a que terminen los threads
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    
    // BUG: Intentar acceder al contador sin lock
    // println!("Valor final: {}", counter.lock().unwrap().get_value());
}

/// Función que demuestra problemas con RwLock
fn demonstrate_rwlock_bugs() {
    println!("\n🔍 Demostrando bugs con RwLock...");
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // BUG: Múltiples writers simultáneos
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // BUG: No manejar el Result del write lock
            let mut writer = data_clone.write().unwrap();
            writer.push(i);
            println!("Writer {} añadió elemento", i);
        });
        handles.push(handle);
    }
    
    // BUG: Reader mientras hay writers
    let data_clone = Arc::clone(&data);
    let reader_handle = thread::spawn(move || {
        // BUG: No manejar el Result del read lock
        let reader = data_clone.read().unwrap();
        println!("Reader lee: {:?}", *reader);
    });
    handles.push(reader_handle);
    
    // BUG: No esperar a que terminen
    // for handle in handles {
    //     handle.join().unwrap();
    // }
}

/// Función que demuestra problemas con channels
fn demonstrate_channel_bugs() {
    println!("\n🔍 Demostrando bugs con channels...");
    
    // BUG: Usar channel síncrono cuando se necesita asíncrono
    let (tx, rx) = mpsc::channel();
    
    // BUG: Múltiples senders sin clonar
    let tx1 = tx.clone();
    let tx2 = tx; // BUG: tx se mueve aquí
    
    // ESTE CÓDIGO CAUSARÁ ERROR DE COMPILACIÓN:
    // let handle1 = thread::spawn(move || {
    //     tx1.send("Mensaje 1").unwrap();
    // });
    // let handle2 = thread::spawn(move || {
    //     tx2.send("Mensaje 2").unwrap();
    // });
    
    // BUG: No manejar el Result del send
    // tx.send("Mensaje").unwrap();
    
    // BUG: No recibir mensajes
    // while let Ok(msg) = rx.recv() {
    //     println!("Recibido: {}", msg);
    // }
}

/// Función que demuestra problemas con data races
fn demonstrate_data_race_bugs() {
    println!("\n🔍 Demostrando bugs con data races...");
    
    let shared_data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // BUG: Múltiples threads accediendo sin sincronización adecuada
    for i in 0..10 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            // BUG: No usar el lock correctamente
            let mut data = data_clone.lock().unwrap();
            *data += i;
            // BUG: No liberar el lock explícitamente
            drop(data);
        });
        handles.push(handle);
    }
    
    // BUG: No esperar a que terminen
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    
    // BUG: Acceder sin lock
    // println!("Valor final: {}", *shared_data.lock().unwrap());
}

/// Función que demuestra problemas con deadlocks
fn demonstrate_deadlock_bugs() {
    println!("\n🔍 Demostrando bugs con deadlocks...");
    
    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));
    
    // BUG: Orden de locks que puede causar deadlock
    let res1_clone = Arc::clone(&resource1);
    let res2_clone = Arc::clone(&resource2);
    
    let handle1 = thread::spawn(move || {
        // BUG: Lock en orden 1, 2
        let _lock1 = res1_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let _lock2 = res2_clone.lock().unwrap();
        println!("Thread 1 adquirió ambos locks");
    });
    
    let res1_clone2 = Arc::clone(&resource1);
    let res2_clone2 = Arc::clone(&resource2);
    
    let handle2 = thread::spawn(move || {
        // BUG: Lock en orden 2, 1 (orden inverso)
        let _lock2 = res2_clone2.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let _lock1 = res1_clone2.lock().unwrap();
        println!("Thread 2 adquirió ambos locks");
    });
    
    // BUG: No manejar el join
    // handle1.join().unwrap();
    // handle2.join().unwrap();
}

/// Función que demuestra problemas con async/await
fn demonstrate_async_bugs() {
    println!("\n🔍 Demostrando bugs con async/await...");
    
    // BUG: Usar async sin runtime
    // ESTE CÓDIGO NO COMPILARÁ SIN DEPENDENCIAS ADICIONALES:
    // async fn async_function() -> i32 {
    //     tokio::time::sleep(Duration::from_millis(100)).await;
    //     42
    // }
    
    // BUG: No manejar el Future
    // let future = async_function();
    // let result = future.await;
    
    println!("Async/await requiere dependencias adicionales como tokio");
}

/// Función que demuestra problemas con lifetimes en threads
fn demonstrate_lifetime_bugs() {
    println!("\n🔍 Demostrando bugs con lifetimes en threads...");
    
    let data = String::from("Datos temporales");
    
    // BUG: Referencia que no vive lo suficiente
    // ESTE CÓDIGO CAUSARÁ ERROR DE COMPILACIÓN:
    // let handle = thread::spawn(move || {
    //     println!("Datos: {}", data);
    // });
    
    // BUG: Usar referencia después de move
    // println!("Datos originales: {}", data);
    
    println!("Lifetimes en threads requieren cuidado especial");
}

fn main() {
    println!("🦀 Rust Lab - Concurrency Bug Spotting");
    println!("{}", "=".repeat(50));
    
    // Ejecutar demostraciones (algunas compilarán, otras no)
    demonstrate_thread_bugs();
    demonstrate_arc_mutex_bugs();
    demonstrate_rwlock_bugs();
    demonstrate_channel_bugs();
    demonstrate_data_race_bugs();
    demonstrate_deadlock_bugs();
    demonstrate_async_bugs();
    demonstrate_lifetime_bugs();
    
    println!("\n✅ Ejercicio completado. Revisa los comentarios para entender los bugs.");
    println!("🔧 Algunos bugs requieren dependencias adicionales como tokio para async/await");
}

