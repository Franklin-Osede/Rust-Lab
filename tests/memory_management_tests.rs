//! Tests para los ejercicios de memory management

#[cfg(test)]
mod memory_management_tests {
    use std::rc::{Rc, Weak};
    use std::sync::Arc;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::thread;
    
    // Importar las estructuras del ejercicio
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        children: Vec<Rc<RefCell<TreeNode>>>,
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
        
        fn add_child(&mut self, child: Rc<RefCell<TreeNode>>) {
            if let Ok(mut child_ref) = child.try_borrow_mut() {
                child_ref.parent = Some(Rc::downgrade(&Rc::new(RefCell::new(TreeNode::new(self.value)))));
            }
            self.children.push(child);
        }
        
        fn get_parent_value(&self) -> Option<i32> {
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
    
    #[test]
    fn test_tree_node_creation() {
        let node = TreeNode::new(42);
        assert_eq!(node.value, 42);
        assert!(node.children.is_empty());
        assert_eq!(node.get_parent_value(), None);
    }
    
    #[test]
    fn test_rc_basic_usage() {
        let data = Rc::new(42);
        assert_eq!(*data, 42);
        assert_eq!(Rc::strong_count(&data), 1);
        
        let data_clone = Rc::clone(&data);
        assert_eq!(*data_clone, 42);
        assert_eq!(Rc::strong_count(&data), 2);
        assert_eq!(Rc::strong_count(&data_clone), 2);
    }
    
    #[test]
    fn test_rc_without_cycles() {
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        
        // Establecer jerarquía sin ciclos
        {
            let mut node1_ref = node1.borrow_mut();
            node1_ref.add_child(node2.clone());
            node1_ref.add_child(node3.clone());
        }
        
        assert_eq!(Rc::strong_count(&node1), 1);
        assert_eq!(Rc::strong_count(&node2), 1);
        assert_eq!(Rc::strong_count(&node3), 1);
    }
    
    #[test]
    fn test_refcell_basic_usage() {
        let data = Rc::new(RefCell::new(42));
        
        // Test borrow inmutable
        {
            let borrow = data.borrow();
            assert_eq!(*borrow, 42);
        }
        
        // Test borrow mutable
        {
            let mut borrow_mut = data.borrow_mut();
            *borrow_mut += 1;
            assert_eq!(*borrow_mut, 43);
        }
        
        // Test acceso después de modificación
        {
            let borrow = data.borrow();
            assert_eq!(*borrow, 43);
        }
    }
    
    #[test]
    fn test_refcell_borrow_error() {
        let data = Rc::new(RefCell::new(42));
        
        // Test que no se pueden tener múltiples borrows mutables
        let _borrow1 = data.borrow_mut();
        let borrow2 = data.try_borrow_mut();
        assert!(borrow2.is_err());
    }
    
    #[test]
    fn test_arc_with_threads() {
        let data = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        // Crear múltiples threads que incrementan el contador
        for i in 0..5 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let mut data_guard = data_clone.lock().unwrap();
                *data_guard += i;
            });
            handles.push(handle);
        }
        
        // Esperar a que terminen todos los threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verificar el resultado final
        let data_guard = data.lock().unwrap();
        assert_eq!(*data_guard, 10); // 0+1+2+3+4
    }
    
    #[test]
    fn test_weak_references() {
        let strong = Rc::new(42);
        let weak = Rc::downgrade(&strong);
        
        // Test que weak reference es válida
        assert_eq!(Rc::strong_count(&strong), 1);
        assert_eq!(Rc::weak_count(&strong), 1);
        
        // Test que weak reference funciona
        if let Some(strong_ref) = weak.upgrade() {
            assert_eq!(*strong_ref, 42);
        } else {
            panic!("Weak reference debería ser válida");
        }
        
        // Test que weak reference se invalida cuando strong se libera
        drop(strong);
        
        if let Some(_) = weak.upgrade() {
            panic!("Weak reference debería ser inválida");
        }
    }
    
    #[test]
    fn test_memory_management() {
        let mut data = Vec::with_capacity(1000);
        for i in 0..1000 {
            data.push(i);
        }
        
        assert_eq!(data.len(), 1000);
        assert_eq!(data.capacity(), 1000);
        
        // Test limpieza de memoria
        data.clear();
        data.shrink_to_fit();
        
        assert_eq!(data.len(), 0);
        assert!(data.capacity() < 1000);
    }
    
    #[test]
    fn test_hashmap_memory_management() {
        let mut map = HashMap::new();
        for i in 0..1000 {
            map.insert(i, format!("value_{}", i));
        }
        
        assert_eq!(map.len(), 1000);
        
        // Test limpieza selectiva
        map.retain(|k, _| k % 2 == 0);
        assert_eq!(map.len(), 500);
        
        // Test limpieza completa
        map.clear();
        assert_eq!(map.len(), 0);
    }
    
    #[test]
    fn test_safe_recursion() {
        // Test recursión con límite de profundidad
        let result = safe_recursion(100);
        assert_eq!(result, 5050); // Suma de 0 a 100
    }
    
    #[test]
    fn test_box_ownership() {
        // Test Box para ownership único
        let data = Box::new(42);
        assert_eq!(*data, 42);
        
        // Test que Box se libera automáticamente
        let data_ptr = Box::into_raw(data);
        unsafe {
            let data = Box::from_raw(data_ptr);
            assert_eq!(*data, 42);
        } // Box se libera automáticamente aquí
    }
    
    #[test]
    fn test_buffer_safety() {
        let mut buffer = [0; 10];
        
        // Test acceso seguro con bounds checking
        for i in 0..buffer.len() {
            buffer[i] = i as u8;
        }
        
        // Test iteración segura
        for (i, &value) in buffer.iter().enumerate() {
            assert_eq!(value, i as u8);
        }
    }
    
    #[test]
    fn test_memory_optimization() {
        // Test allocations de tamaño uniforme
        let mut data = Vec::new();
        for i in 0..100 {
            let vec = vec![0; 100]; // Tamaño uniforme
            data.push(vec);
        }
        
        assert_eq!(data.len(), 100);
        assert!(data.iter().all(|v| v.len() == 100));
    }
    
    #[test]
    fn test_resource_management() {
        // Test RAII (Resource Acquisition Is Initialization)
        let data = String::from("Datos importantes");
        assert_eq!(data, "Datos importantes");
        
        // Los recursos se liberan automáticamente al salir de scope
        // No hay necesidad de liberar manualmente
    }
    
    #[test]
    fn test_memory_error_handling() {
        // Test manejo seguro de allocations grandes
        match try_large_allocation() {
            Ok(data) => {
                assert_eq!(data.len(), 1_000_000);
                assert!(data.iter().all(|&x| x < 256));
            }
            Err(e) => {
                panic!("Error inesperado en allocation: {}", e);
            }
        }
    }
    
    #[test]
    fn test_arc_weak_in_threads() {
        let data = Arc::new(42);
        let weak = Arc::downgrade(&data);
        
        let handle = thread::spawn(move || {
            if let Some(strong_ref) = weak.upgrade() {
                *strong_ref
            } else {
                0
            }
        });
        
        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_memory_fragmentation_prevention() {
        // Test allocations de tamaño uniforme para reducir fragmentación
        let mut data = Vec::new();
        for i in 0..100 {
            let vec = vec![i; 100]; // Tamaño uniforme
            data.push(vec);
        }
        
        assert_eq!(data.len(), 100);
        assert!(data.iter().enumerate().all(|(i, v)| v.len() == 100 && v[0] == i));
    }
    
    // Funciones auxiliares
    fn safe_recursion(n: u32) -> u32 {
        if n == 0 {
            0
        } else if n > 1000 {
            // Límite de profundidad para evitar stack overflow
            n
        } else {
            n + safe_recursion(n - 1)
        }
    }
    
    fn try_large_allocation() -> Result<Vec<u8>, String> {
        let size = 1_000_000;
        let mut data = Vec::with_capacity(size);
        
        for i in 0..size {
            data.push((i % 256) as u8);
        }
        
        Ok(data)
    }
}


