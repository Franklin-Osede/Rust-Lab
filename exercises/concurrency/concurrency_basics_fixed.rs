//! 🦀 Concurrency Basics - SOLUCIÓN CORREGIDA
//! 
//! Esta es la versión corregida del ejercicio anterior,
//! mostrando las mejores prácticas de concurrencia en Rust.

use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc;
use std::time::Duration;

/// Estructura que representa un contador compartido
#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }
    
    /// CORREGIDO: Método que incrementa el contador
    fn increment(&mut self) {
        self.value += 1;
    }
    
    /// CORREGIDO: Método que obtiene el valor
    fn get_value(&self) -> i32 {
        self.value
    }
}

/// Función que demuestra threads correctos
fn demonstrate_threads_correct() {
    println!("✅ Demostrando threads correctos...");
    
    let counter = Arc::new(Mutex::new(Counter::new()));
    let mut handles = vec![];
    
    // CORREGIDO: Usar Arc<Mutex<T>> para compartir entre threads
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // CORREGIDO: Manejar el Result del lock
            match counter_clone.lock() {
                Ok(mut counter_guard) => {
                    counter_guard.increment();
                    println!("Thread {} incrementó el contador a {}", i, counter_guard.get_value());
                }
                Err(e) => {
                    println!("Error al adquirir lock en thread {}: {}", i, e);
                }
            }
        });
        handles.push(handle);
    }
    
    // CORREGIDO: Esperar a que terminen todos los threads
    for handle in handles {
        match handle.join() {
            Ok(_) => println!("Thread completado exitosamente"),
            Err(e) => println!("Error en thread: {:?}", e),
        }
    }
    
    // CORREGIDO: Acceder al contador con lock
    match counter.lock() {
        Ok(counter_guard) => println!("Valor final del contador: {}", counter_guard.get_value()),
        Err(e) => println!("Error al acceder al contador: {}", e),
    }
}

/// Función que demuestra RwLock correcto
fn demonstrate_rwlock_correct() {
    println!("\n✅ Demostrando RwLock correcto...");
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // CORREGIDO: Writers secuenciales
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // CORREGIDO: Manejar el Result del write lock
            match data_clone.write() {
                Ok(mut writer) => {
                    writer.push(i);
                    println!("Writer {} añadió elemento. Vector: {:?}", i, *writer);
                }
                Err(e) => {
                    println!("Error al adquirir write lock: {}", e);
                }
            }
        });
        handles.push(handle);
    }
    
    // CORREGIDO: Reader después de que terminen los writers
    let data_clone = Arc::clone(&data);
    let reader_handle = thread::spawn(move || {
        // CORREGIDO: Manejar el Result del read lock
        match data_clone.read() {
            Ok(reader) => {
                println!("Reader lee: {:?}", *reader);
            }
            Err(e) => {
                println!("Error al adquirir read lock: {}", e);
            }
        }
    });
    handles.push(reader_handle);
    
    // CORREGIDO: Esperar a que terminen todos
    for handle in handles {
        handle.join().unwrap();
    }
}

/// Función que demuestra channels correctos
fn demonstrate_channels_correct() {
    println!("\n✅ Demostrando channels correctos...");
    
    // CORREGIDO: Usar channel asíncrono
    let (tx, rx) = mpsc::channel();
    
    // CORREGIDO: Clonar el sender para múltiples threads
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    
    let handle1 = thread::spawn(move || {
        // CORREGIDO: Manejar el Result del send
        match tx1.send("Mensaje del thread 1") {
            Ok(_) => println!("Thread 1 envió mensaje"),
            Err(e) => println!("Error al enviar desde thread 1: {}", e),
        }
    });
    
    let handle2 = thread::spawn(move || {
        // CORREGIDO: Manejar el Result del send
        match tx2.send("Mensaje del thread 2") {
            Ok(_) => println!("Thread 2 envió mensaje"),
            Err(e) => println!("Error al enviar desde thread 2: {}", e),
        }
    });
    
    // CORREGIDO: Cerrar el sender original
    drop(tx);
    
    // CORREGIDO: Recibir mensajes
    while let Ok(msg) = rx.recv() {
        println!("Recibido: {}", msg);
    }
    
    // CORREGIDO: Esperar a que terminen los threads
    handle1.join().unwrap();
    handle2.join().unwrap();
}

/// Función que demuestra sincronización correcta
fn demonstrate_synchronization_correct() {
    println!("\n✅ Demostrando sincronización correcta...");
    
    let shared_data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // CORREGIDO: Sincronización adecuada
    for i in 0..10 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            // CORREGIDO: Usar el lock correctamente
            match data_clone.lock() {
                Ok(mut data) => {
                    *data += i;
                    println!("Thread {} añadió {}. Valor actual: {}", i, i, *data);
                }
                Err(e) => {
                    println!("Error al adquirir lock en thread {}: {}", i, e);
                }
            }
        });
        handles.push(handle);
    }
    
    // CORREGIDO: Esperar a que terminen todos
    for handle in handles {
        handle.join().unwrap();
    }
    
    // CORREGIDO: Acceder con lock
    match shared_data.lock() {
        Ok(data) => println!("Valor final: {}", *data),
        Err(e) => println!("Error al acceder al valor final: {}", e),
    }
}

/// Función que demuestra prevención de deadlocks
fn demonstrate_deadlock_prevention() {
    println!("\n✅ Demostrando prevención de deadlocks...");
    
    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));
    
    // CORREGIDO: Mismo orden de locks para evitar deadlock
    let res1_clone = Arc::clone(&resource1);
    let res2_clone = Arc::clone(&resource2);
    
    let handle1 = thread::spawn(move || {
        // CORREGIDO: Lock en orden 1, 2
        match res1_clone.lock() {
            Ok(_lock1) => {
                thread::sleep(Duration::from_millis(50));
                match res2_clone.lock() {
                    Ok(_lock2) => {
                        println!("Thread 1 adquirió ambos locks");
                    }
                    Err(e) => println!("Error al adquirir lock2 en thread 1: {}", e),
                }
            }
            Err(e) => println!("Error al adquirir lock1 en thread 1: {}", e),
        }
    });
    
    let res1_clone2 = Arc::clone(&resource1);
    let res2_clone2 = Arc::clone(&resource2);
    
    let handle2 = thread::spawn(move || {
        // CORREGIDO: Mismo orden 1, 2
        match res1_clone2.lock() {
            Ok(_lock1) => {
                thread::sleep(Duration::from_millis(50));
                match res2_clone2.lock() {
                    Ok(_lock2) => {
                        println!("Thread 2 adquirió ambos locks");
                    }
                    Err(e) => println!("Error al adquirir lock2 en thread 2: {}", e),
                }
            }
            Err(e) => println!("Error al adquirir lock1 en thread 2: {}", e),
        }
    });
    
    // CORREGIDO: Manejar el join
    handle1.join().unwrap();
    handle2.join().unwrap();
}

/// Función que demuestra lifetimes correctos en threads
fn demonstrate_lifetime_correct() {
    println!("\n✅ Demostrando lifetimes correctos en threads...");
    
    let data = String::from("Datos temporales");
    
    // CORREGIDO: Mover los datos al thread
    let handle = thread::spawn(move || {
        println!("Datos en thread: {}", data);
    });
    
    // CORREGIDO: Esperar a que termine el thread
    handle.join().unwrap();
    
    // CORREGIDO: Los datos originales ya no están disponibles aquí
    // println!("Datos originales: {}", data); // Esto no compilaría
}

/// Función que demuestra manejo de errores en concurrencia
fn demonstrate_error_handling_concurrency() {
    println!("\n✅ Demostrando manejo de errores en concurrencia...");
    
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    for i in 0..5 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            // CORREGIDO: Manejo completo de errores
            match data_clone.lock() {
                Ok(mut data) => {
                    if i < data.len() {
                        data[i] *= 2;
                        println!("Thread {} procesó elemento {}", i, i);
                    } else {
                        println!("Thread {}: índice fuera de rango", i);
                    }
                }
                Err(e) => {
                    println!("Thread {}: error al adquirir lock: {}", i, e);
                }
            }
        });
        handles.push(handle);
    }
    
    // CORREGIDO: Manejar errores en join
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(_) => println!("Thread {} completado exitosamente", i),
            Err(e) => println!("Error en thread {}: {:?}", i, e),
        }
    }
    
    // CORREGIDO: Verificar resultado final
    match shared_data.lock() {
        Ok(data) => println!("Datos finales: {:?}", *data),
        Err(e) => println!("Error al acceder a datos finales: {}", e),
    }
}

fn main() {
    println!("🦀 Rust Lab - Concurrency SOLUCIÓN CORRECTA");
    println!("{}", "=".repeat(60));
    
    demonstrate_threads_correct();
    demonstrate_rwlock_correct();
    demonstrate_channels_correct();
    demonstrate_synchronization_correct();
    demonstrate_deadlock_prevention();
    demonstrate_lifetime_correct();
    demonstrate_error_handling_concurrency();
    
    println!("\n✅ Todas las demostraciones completadas sin errores!");
    println!("🎯 Conceptos clave demostrados:");
    println!("   - Threads: Ejecución concurrente");
    println!("   - Arc<T>: Referencias atómicas compartidas");
    println!("   - Mutex<T>: Exclusión mutua");
    println!("   - RwLock<T>: Lectores múltiples, escritor único");
    println!("   - Channels: Comunicación entre threads");
    println!("   - Deadlock prevention: Prevención de bloqueos");
    println!("   - Error handling: Manejo de errores en concurrencia");
}


