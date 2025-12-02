//! 🦀 Memory Management - Bug Spotting Exercise
//! 
//! Este ejercicio demuestra conceptos de gestión de memoria en Rust
//! con bugs intencionales para practicar debugging.

use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;

/// Estructura que representa un nodo en un árbol
#[derive(Debug)]
struct TreeNode {
    value: i32,
    // BUG INTENCIONAL: Vec<TreeNode> en lugar de Vec<Rc<RefCell<TreeNode>>>
    children: Vec<TreeNode>,
    // BUG INTENCIONAL: Referencia directa que puede causar ciclos
    parent: Option<*mut TreeNode>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        Self {
            value,
            children: Vec::new(),
            parent: None,
        }
    }
    
    /// BUG INTENCIONAL: Método que puede causar use-after-free
    fn add_child(&mut self, child: TreeNode) {
        // BUG: No manejar la referencia al parent correctamente
        self.children.push(child);
    }
    
    /// BUG INTENCIONAL: Método que puede causar dangling pointer
    fn get_parent_value(&self) -> Option<i32> {
        // BUG: Dereferenciar puntero sin verificar
        unsafe {
            if let Some(parent) = self.parent {
                Some((*parent).value)
            } else {
                None
            }
        }
    }
}

/// Función que demuestra problemas con Rc y ciclos
fn demonstrate_rc_cycle_bugs() {
    println!("🔍 Demostrando bugs con Rc y ciclos...");
    
    // BUG: Crear ciclo con Rc
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    
    // BUG: Crear referencia circular
    // ESTE CÓDIGO CAUSARÁ MEMORY LEAK:
    // node1.borrow_mut().children.push(node2.clone());
    // node2.borrow_mut().children.push(node1.clone());
    
    println!("Nodos creados: {:?}", node1);
}

/// Función que demuestra problemas con RefCell
fn demonstrate_refcell_bugs() {
    println!("\n🔍 Demostrando bugs con RefCell...");
    
    let data = Rc::new(RefCell::new(42));
    
    // BUG: Múltiples borrows mutables
    // ESTE CÓDIGO CAUSARÁ PANIC:
    // let borrow1 = data.borrow_mut();
    // let borrow2 = data.borrow_mut();
    
    // BUG: Borrow después de move
    // let borrow = data.borrow();
    // let borrow_mut = data.borrow_mut(); // PANIC: ya hay borrow inmutable
    
    println!("Data: {:?}", data);
}

/// Función que demuestra problemas con Arc y threads
fn demonstrate_arc_thread_bugs() {
    println!("\n🔍 Demostrando bugs con Arc y threads...");
    
    use std::thread;
    
    let data = Arc::new(42);
    let mut handles = vec![];
    
    // BUG: Múltiples threads accediendo sin sincronización
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // BUG: Acceso directo sin protección
            println!("Thread {} accede a data: {}", i, data_clone);
        });
        handles.push(handle);
    }
    
    // BUG: No esperar a que terminen los threads
    // for handle in handles {
    //     handle.join().unwrap();
    // }
}

/// Función que demuestra problemas con memory leaks
fn demonstrate_memory_leak_bugs() {
    println!("\n🔍 Demostrando bugs con memory leaks...");
    
    // BUG: Vec que crece indefinidamente
    let mut data = Vec::new();
    for i in 0..1000000 {
        data.push(i);
        // BUG: No liberar memoria periódicamente
    }
    
    println!("Vec creado con {} elementos", data.len());
    
    // BUG: HashMap que no se limpia
    let mut map = HashMap::new();
    for i in 0..100000 {
        map.insert(i, format!("value_{}", i));
        // BUG: No limpiar entradas antiguas
    }
    
    println!("HashMap creado con {} elementos", map.len());
}

/// Función que demuestra problemas con stack overflow
fn demonstrate_stack_overflow_bugs() {
    println!("\n🔍 Demostrando bugs con stack overflow...");
    
    // BUG: Recursión profunda sin optimización
    // ESTE CÓDIGO PUEDE CAUSAR STACK OVERFLOW:
    // let result = deep_recursion(10000);
    
    println!("Recursión profunda puede causar stack overflow");
}

/// Función que demuestra problemas con dangling pointers
fn demonstrate_dangling_pointer_bugs() {
    println!("\n🔍 Demostrando bugs con dangling pointers...");
    
    // BUG: Referencia que no vive lo suficiente
    let reference = {
        let local_data = String::from("Datos locales");
        &local_data // BUG: Referencia a datos que se liberan
    };
    
    // ESTE CÓDIGO CAUSARÍA ERROR DE COMPILACIÓN:
    // println!("Referencia: {}", reference);
    
    println!("Dangling pointers detectados por el compilador");
}

/// Función que demuestra problemas con double free
fn demonstrate_double_free_bugs() {
    println!("\n🔍 Demostrando bugs con double free...");
    
    // BUG: Intentar liberar memoria dos veces
    let data = Box::new(42);
    let data_ptr = Box::into_raw(data);
    
    // BUG: Liberar memoria dos veces
    // unsafe {
    //     Box::from_raw(data_ptr);
    //     Box::from_raw(data_ptr); // BUG: Double free
    // }
    
    println!("Double free detectado por el compilador");
}

/// Función que demuestra problemas con use after free
fn demonstrate_use_after_free_bugs() {
    println!("\n🔍 Demostrando bugs con use after free...");
    
    let data = Box::new(42);
    let data_ptr = Box::into_raw(data);
    
    // BUG: Usar después de liberar
    // unsafe {
    //     Box::from_raw(data_ptr);
    //     println!("Valor: {}", *data_ptr); // BUG: Use after free
    // }
    
    println!("Use after free detectado por el compilador");
}

/// Función que demuestra problemas con buffer overflow
fn demonstrate_buffer_overflow_bugs() {
    println!("\n🔍 Demostrando bugs con buffer overflow...");
    
    let mut buffer = [0; 10];
    
    // BUG: Acceso fuera de bounds
    // ESTE CÓDIGO CAUSARÍA PANIC:
    // buffer[10] = 42; // BUG: Índice fuera de bounds
    
    // BUG: Iteración sin bounds checking
    for i in 0..15 {
        if i < buffer.len() {
            buffer[i] = i as u8;
        }
    }
    
    println!("Buffer: {:?}", buffer);
}

/// Función que demuestra problemas con memory fragmentation
fn demonstrate_memory_fragmentation_bugs() {
    println!("\n🔍 Demostrando bugs con memory fragmentation...");
    
    // BUG: Allocations de diferentes tamaños
    let mut data = Vec::new();
    for i in 0..1000 {
        let size = if i % 2 == 0 { 100 } else { 1000 };
        let vec = vec![0; size];
        data.push(vec);
    }
    
    println!("Fragmentation creada con {} allocations", data.len());
}

/// Función que demuestra problemas con garbage collection
fn demonstrate_gc_bugs() {
    println!("\n🔍 Demostrando bugs con garbage collection...");
    
    // BUG: Rust no tiene GC, pero podemos simular problemas
    let mut data = Vec::new();
    for i in 0..10000 {
        let string = format!("String {}", i);
        data.push(string);
    }
    
    // BUG: No limpiar referencias
    // data.clear(); // Comentado para simular memory leak
    
    println!("Datos creados: {}", data.len());
}

/// Función auxiliar para recursión profunda (comentada para evitar stack overflow)
fn deep_recursion(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n + deep_recursion(n - 1)
    }
}

fn main() {
    println!("🦀 Rust Lab - Memory Management Bug Spotting");
    println!("{}", "=".repeat(60));
    
    // Ejecutar demostraciones
    demonstrate_rc_cycle_bugs();
    demonstrate_refcell_bugs();
    demonstrate_arc_thread_bugs();
    demonstrate_memory_leak_bugs();
    demonstrate_stack_overflow_bugs();
    demonstrate_dangling_pointer_bugs();
    demonstrate_double_free_bugs();
    demonstrate_use_after_free_bugs();
    demonstrate_buffer_overflow_bugs();
    demonstrate_memory_fragmentation_bugs();
    demonstrate_gc_bugs();
    
    println!("\n✅ Ejercicio completado. Revisa los comentarios para entender los bugs.");
    println!("🔧 Rust previene muchos de estos bugs en tiempo de compilación");
}


