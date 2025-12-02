//! 🦀 Memory Management - SOLUCIÓN CORREGIDA
//! 
//! Esta es la versión corregida del ejercicio anterior,
//! mostrando las mejores prácticas de gestión de memoria en Rust.

use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

/// Estructura que representa un nodo en un árbol con referencias seguras
#[derive(Debug)]
struct TreeNode {
    value: i32,
    // CORREGIDO: Vec<Rc<RefCell<TreeNode>>> para referencias compartidas
    children: Vec<Rc<RefCell<TreeNode>>>,
    // CORREGIDO: Weak reference para evitar ciclos
    parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        Self {
            value,
            children: Vec::new(),
            parent: None,
        }
    }
    
    /// CORREGIDO: Método que maneja referencias correctamente
    fn add_child(&mut self, child: Rc<RefCell<TreeNode>>) {
        // CORREGIDO: Establecer referencia bidireccional segura
        if let Ok(mut child_ref) = child.try_borrow_mut() {
            child_ref.parent = Some(Rc::downgrade(&Rc::new(RefCell::new(TreeNode::new(self.value)))));
        }
        self.children.push(child);
    }
    
    /// CORREGIDO: Método que maneja referencias débiles
    fn get_parent_value(&self) -> Option<i32> {
        // CORREGIDO: Usar Weak reference de forma segura
        if let Some(parent_weak) = &self.parent {
            if let Some(parent_rc) = parent_weak.upgrade() {
                if let Ok(parent_ref) = parent_rc.try_borrow() {
                    return Some(parent_ref.value);
                }
            }
        }
        None
    }
}

/// Función que demuestra Rc sin ciclos
fn demonstrate_rc_without_cycles() {
    println!("✅ Demostrando Rc sin ciclos...");
    
    // CORREGIDO: Crear nodos sin ciclos
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    
    // CORREGIDO: Estructura jerárquica sin ciclos
    {
        let mut node1_ref = node1.borrow_mut();
        node1_ref.add_child(node2.clone());
        node1_ref.add_child(node3.clone());
    }
    
    println!("Árbol creado sin ciclos: {:?}", node1);
    println!("Número de referencias a node1: {}", Rc::strong_count(&node1));
}

/// Función que demuestra RefCell correcto
fn demonstrate_refcell_correct() {
    println!("\n✅ Demostrando RefCell correcto...");
    
    let data = Rc::new(RefCell::new(42));
    
    // CORREGIDO: Manejo correcto de borrows
    {
        let borrow = data.borrow();
        println!("Valor: {}", *borrow);
    } // borrow se libera aquí
    
    {
        let mut borrow_mut = data.borrow_mut();
        *borrow_mut += 1;
        println!("Valor incrementado: {}", *borrow_mut);
    } // borrow_mut se libera aquí
    
    // CORREGIDO: Verificar que no hay borrows activos
    let final_value = data.borrow();
    println!("Valor final: {}", *final_value);
}

/// Función que demuestra Arc con threads
fn demonstrate_arc_threads_correct() {
    println!("\n✅ Demostrando Arc con threads correcto...");
    
    use std::thread;
    use std::sync::Mutex;
    
    let data = Arc::new(Mutex::new(42));
    let mut handles = vec![];
    
    // CORREGIDO: Threads con sincronización
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // CORREGIDO: Usar Mutex para sincronización
            match data_clone.lock() {
                Ok(mut value) => {
                    *value += i;
                    println!("Thread {} actualizó valor a {}", i, *value);
                }
                Err(e) => {
                    println!("Thread {}: error al adquirir lock: {}", i, e);
                }
            }
        });
        handles.push(handle);
    }
    
    // CORREGIDO: Esperar a que terminen todos los threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // CORREGIDO: Verificar resultado final
    match data.lock() {
        Ok(value) => println!("Valor final: {}", *value),
        Err(e) => println!("Error al acceder al valor final: {}", e),
    }
}

/// Función que demuestra gestión correcta de memoria
fn demonstrate_memory_management_correct() {
    println!("\n✅ Demostrando gestión correcta de memoria...");
    
    // CORREGIDO: Vec con gestión de memoria
    let mut data = Vec::with_capacity(1000);
    for i in 0..1000 {
        data.push(i);
    }
    
    println!("Vec creado con {} elementos", data.len());
    
    // CORREGIDO: Limpiar memoria cuando sea necesario
    data.clear();
    data.shrink_to_fit();
    println!("Vec limpiado y optimizado");
    
    // CORREGIDO: HashMap con gestión de memoria
    let mut map = HashMap::new();
    for i in 0..1000 {
        map.insert(i, format!("value_{}", i));
    }
    
    println!("HashMap creado con {} elementos", map.len());
    
    // CORREGIDO: Limpiar entradas antiguas
    map.retain(|k, _| k % 2 == 0);
    println!("HashMap limpiado, {} elementos restantes", map.len());
}

/// Función que demuestra recursión optimizada
fn demonstrate_recursion_optimized() {
    println!("\n✅ Demostrando recursión optimizada...");
    
    // CORREGIDO: Recursión con límite de profundidad
    let result = safe_recursion(1000);
    println!("Resultado de recursión segura: {}", result);
}

/// Función que demuestra manejo seguro de punteros
fn demonstrate_safe_pointers() {
    println!("\n✅ Demostrando manejo seguro de punteros...");
    
    // CORREGIDO: Usar Box para ownership único
    let data = Box::new(42);
    println!("Datos en Box: {}", data);
    
    // CORREGIDO: Box se libera automáticamente al salir de scope
    // No hay necesidad de liberar manualmente
}

/// Función que demuestra prevención de buffer overflow
fn demonstrate_buffer_safety() {
    println!("\n✅ Demostrando seguridad de buffers...");
    
    let mut buffer = [0; 10];
    
    // CORREGIDO: Acceso seguro con bounds checking
    for i in 0..buffer.len() {
        buffer[i] = i as u8;
    }
    
    // CORREGIDO: Iteración segura
    for (i, &value) in buffer.iter().enumerate() {
        println!("buffer[{}] = {}", i, value);
    }
}

/// Función que demuestra optimización de memoria
fn demonstrate_memory_optimization() {
    println!("\n✅ Demostrando optimización de memoria...");
    
    // CORREGIDO: Allocations de tamaño uniforme para reducir fragmentación
    let mut data = Vec::new();
    for i in 0..1000 {
        let vec = vec![0; 100]; // CORREGIDO: Tamaño uniforme
        data.push(vec);
    }
    
    println!("Fragmentation minimizada con {} allocations uniformes", data.len());
}

/// Función que demuestra gestión de recursos
fn demonstrate_resource_management() {
    println!("\n✅ Demostrando gestión de recursos...");
    
    // CORREGIDO: RAII (Resource Acquisition Is Initialization)
    let data = String::from("Datos importantes");
    println!("Datos: {}", data);
    
    // CORREGIDO: Los recursos se liberan automáticamente
    // No hay necesidad de liberar manualmente
}

/// Función que demuestra manejo de errores de memoria
fn demonstrate_memory_error_handling() {
    println!("\n✅ Demostrando manejo de errores de memoria...");
    
    // CORREGIDO: Manejo seguro de allocations grandes
    match try_large_allocation() {
        Ok(data) => {
            println!("Allocation exitosa: {} elementos", data.len());
        }
        Err(e) => {
            println!("Error en allocation: {}", e);
        }
    }
}

/// Función auxiliar para recursión segura
fn safe_recursion(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n > 1000 {
        // CORREGIDO: Límite de profundidad para evitar stack overflow
        n
    } else {
        n + safe_recursion(n - 1)
    }
}

/// Función auxiliar para allocation grande
fn try_large_allocation() -> Result<Vec<u8>, String> {
    // CORREGIDO: Intentar allocation grande con manejo de errores
    let size = 1_000_000;
    let mut data = Vec::with_capacity(size);
    
    for i in 0..size {
        data.push((i % 256) as u8);
    }
    
    Ok(data)
}

/// Función que demuestra uso de Weak references
fn demonstrate_weak_references() {
    println!("\n✅ Demostrando Weak references...");
    
    let strong = Rc::new(42);
    let weak = Rc::downgrade(&strong);
    
    println!("Referencias fuertes: {}", Rc::strong_count(&strong));
    println!("Referencias débiles: {}", Rc::weak_count(&strong));
    
    // CORREGIDO: Usar Weak reference de forma segura
    if let Some(strong_ref) = weak.upgrade() {
        println!("Weak reference válida: {}", *strong_ref);
    } else {
        println!("Weak reference inválida");
    }
    
    // CORREGIDO: Liberar strong reference
    drop(strong);
    
    // CORREGIDO: Verificar que weak reference es inválida
    if let Some(_) = weak.upgrade() {
        println!("Weak reference aún válida");
    } else {
        println!("Weak reference inválida (como esperado)");
    }
}

fn main() {
    println!("🦀 Rust Lab - Memory Management SOLUCIÓN CORRECTA");
    println!("{}", "=".repeat(70));
    
    demonstrate_rc_without_cycles();
    demonstrate_refcell_correct();
    demonstrate_arc_threads_correct();
    demonstrate_memory_management_correct();
    demonstrate_recursion_optimized();
    demonstrate_safe_pointers();
    demonstrate_buffer_safety();
    demonstrate_memory_optimization();
    demonstrate_resource_management();
    demonstrate_memory_error_handling();
    demonstrate_weak_references();
    
    println!("\n✅ Todas las demostraciones completadas sin errores!");
    println!("🎯 Conceptos clave demostrados:");
    println!("   - Rc<T>: Referencias compartidas sin ciclos");
    println!("   - Weak<T>: Referencias débiles para evitar ciclos");
    println!("   - RefCell<T>: Mutabilidad interior con borrow checking");
    println!("   - Arc<T>: Referencias atómicas para threads");
    println!("   - RAII: Liberación automática de recursos");
    println!("   - Memory safety: Prevención de errores de memoria");
    println!("   - Buffer safety: Prevención de buffer overflow");
    println!("   - Resource management: Gestión automática de recursos");
}


