//! Tests para los ejercicios de concurrency

#[cfg(test)]
mod concurrency_tests {
    use std::sync::{Arc, Mutex, RwLock};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    
    // Importar las estructuras del ejercicio
    #[derive(Debug, Clone, PartialEq)]
    struct Counter {
        value: i32,
    }
    
    impl Counter {
        fn new() -> Self {
            Self { value: 0 }
        }
        
        fn increment(&mut self) {
            self.value += 1;
        }
        
        fn get_value(&self) -> i32 {
            self.value
        }
    }
    
    #[test]
    fn test_counter_creation() {
        let counter = Counter::new();
        assert_eq!(counter.get_value(), 0);
    }
    
    #[test]
    fn test_counter_increment() {
        let mut counter = Counter::new();
        counter.increment();
        assert_eq!(counter.get_value(), 1);
        
        counter.increment();
        counter.increment();
        assert_eq!(counter.get_value(), 3);
    }
    
    #[test]
    fn test_arc_mutex_basic() {
        let counter = Arc::new(Mutex::new(Counter::new()));
        
        // Test acceso inmutable
        {
            let counter_guard = counter.lock().unwrap();
            assert_eq!(counter_guard.get_value(), 0);
        }
        
        // Test acceso mutable
        {
            let mut counter_guard = counter.lock().unwrap();
            counter_guard.increment();
            assert_eq!(counter_guard.get_value(), 1);
        }
        
        // Test acceso después del incremento
        {
            let counter_guard = counter.lock().unwrap();
            assert_eq!(counter_guard.get_value(), 1);
        }
    }
    
    #[test]
    fn test_arc_mutex_multiple_threads() {
        let counter = Arc::new(Mutex::new(Counter::new()));
        let mut handles = vec![];
        
        // Crear múltiples threads que incrementan el contador
        for _ in 0..5 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut counter_guard = counter_clone.lock().unwrap();
                counter_guard.increment();
            });
            handles.push(handle);
        }
        
        // Esperar a que terminen todos los threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verificar el valor final
        let counter_guard = counter.lock().unwrap();
        assert_eq!(counter_guard.get_value(), 5);
    }
    
    #[test]
    fn test_rwlock_basic() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));
        
        // Test lectura múltiple
        {
            let reader1 = data.read().unwrap();
            let reader2 = data.read().unwrap();
            assert_eq!(*reader1, vec![1, 2, 3]);
            assert_eq!(*reader2, vec![1, 2, 3]);
        }
        
        // Test escritura
        {
            let mut writer = data.write().unwrap();
            writer.push(4);
            assert_eq!(*writer, vec![1, 2, 3, 4]);
        }
        
        // Test lectura después de escritura
        {
            let reader = data.read().unwrap();
            assert_eq!(*reader, vec![1, 2, 3, 4]);
        }
    }
    
    #[test]
    fn test_rwlock_multiple_threads() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));
        let mut handles = vec![];
        
        // Crear threads que leen
        for i in 0..3 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let reader = data_clone.read().unwrap();
                println!("Reader {}: {:?}", i, *reader);
            });
            handles.push(handle);
        }
        
        // Crear thread que escribe
        let data_clone = Arc::clone(&data);
        let writer_handle = thread::spawn(move || {
            let mut writer = data_clone.write().unwrap();
            writer.push(4);
            println!("Writer: {:?}", *writer);
        });
        handles.push(writer_handle);
        
        // Esperar a que terminen todos los threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verificar el resultado final
        let reader = data.read().unwrap();
        assert_eq!(*reader, vec![1, 2, 3, 4]);
    }
    
    #[test]
    fn test_channels_basic() {
        let (tx, rx) = mpsc::channel();
        
        // Enviar mensaje
        tx.send("Hello").unwrap();
        
        // Recibir mensaje
        let received = rx.recv().unwrap();
        assert_eq!(received, "Hello");
    }
    
    #[test]
    fn test_channels_multiple_senders() {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];
        
        // Crear múltiples senders
        for i in 0..3 {
            let tx_clone = tx.clone();
            let handle = thread::spawn(move || {
                tx_clone.send(format!("Message from thread {}", i)).unwrap();
            });
            handles.push(handle);
        }
        
        // Cerrar el sender original
        drop(tx);
        
        // Recibir todos los mensajes
        let mut messages = Vec::new();
        while let Ok(msg) = rx.recv() {
            messages.push(msg);
        }
        
        // Esperar a que terminen todos los threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verificar que se recibieron todos los mensajes
        assert_eq!(messages.len(), 3);
        assert!(messages.iter().any(|m| m.contains("thread 0")));
        assert!(messages.iter().any(|m| m.contains("thread 1")));
        assert!(messages.iter().any(|m| m.contains("thread 2")));
    }
    
    #[test]
    fn test_channels_timeout() {
        let (tx, rx) = mpsc::channel();
        
        // Enviar mensaje
        tx.send("Hello").unwrap();
        
        // Recibir con timeout
        let received = rx.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(received, "Hello");
        
        // Test timeout cuando no hay mensajes
        let result = rx.recv_timeout(Duration::from_millis(10));
        assert!(result.is_err());
    }
    
    #[test]
    fn test_deadlock_prevention() {
        let resource1 = Arc::new(Mutex::new(0));
        let resource2 = Arc::new(Mutex::new(0));
        
        // Test mismo orden de locks para evitar deadlock
        let res1_clone = Arc::clone(&resource1);
        let res2_clone = Arc::clone(&resource2);
        
        let handle1 = thread::spawn(move || {
            let _lock1 = res1_clone.lock().unwrap();
            thread::sleep(Duration::from_millis(10));
            let _lock2 = res2_clone.lock().unwrap();
        });
        
        let res1_clone2 = Arc::clone(&resource1);
        let res2_clone2 = Arc::clone(&resource2);
        
        let handle2 = thread::spawn(move || {
            let _lock1 = res1_clone2.lock().unwrap();
            thread::sleep(Duration::from_millis(10));
            let _lock2 = res2_clone2.lock().unwrap();
        });
        
        // Esperar a que terminen ambos threads
        handle1.join().unwrap();
        handle2.join().unwrap();
        
        // Si llegamos aquí, no hubo deadlock
        assert!(true);
    }
    
    #[test]
    fn test_error_handling_in_threads() {
        let data = Arc::new(Mutex::new(vec![1, 2, 3]));
        let mut handles = vec![];
        
        // Crear threads que pueden fallar
        for i in 0..5 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                match data_clone.lock() {
                    Ok(mut data_guard) => {
                        data_guard.push(i);
                        println!("Thread {} añadió elemento", i);
                    }
                    Err(e) => {
                        println!("Thread {}: error al adquirir lock: {}", i, e);
                    }
                }
            });
            handles.push(handle);
        }
        
        // Esperar a que terminen todos los threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verificar el resultado final
        let data_guard = data.lock().unwrap();
        assert_eq!(data_guard.len(), 8); // 3 originales + 5 nuevos
    }
    
    #[test]
    fn test_thread_local_storage() {
        use std::cell::RefCell;
        
        thread_local! {
            static COUNTER: RefCell<i32> = RefCell::new(0);
        }
        
        let mut handles = vec![];
        
        // Crear threads que usan thread local storage
        for i in 0..3 {
            let handle = thread::spawn(move || {
                COUNTER.with(|counter| {
                    *counter.borrow_mut() += i;
                });
                
                let value = COUNTER.with(|counter| *counter.borrow());
                println!("Thread {}: counter = {}", i, value);
            });
            handles.push(handle);
        }
        
        // Esperar a que terminen todos los threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verificar que cada thread tiene su propio counter
        COUNTER.with(|counter| {
            assert_eq!(*counter.borrow(), 0); // El thread principal no fue modificado
        });
    }
    
    #[test]
    fn test_arc_weak_references() {
        use std::rc::{Rc, Weak};
        
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
}


