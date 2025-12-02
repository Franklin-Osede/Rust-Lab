//! Tests para los ejercicios de ownership y borrowing

#[cfg(test)]
mod ownership_tests {
    use std::collections::HashMap;
    
    // Importar las estructuras del ejercicio (necesitarías moverlas a un módulo compartido)
    #[derive(Debug, Clone, PartialEq)]
    struct User {
        id: u32,
        name: String,
        email: String,
        posts: Vec<u32>,
    }
    
    impl User {
        fn new(id: u32, name: String, email: String) -> Self {
            Self {
                id,
                name,
                email,
                posts: Vec::new(),
            }
        }
        
        fn add_post(&mut self, post_id: u32) {
            self.posts.push(post_id);
        }
        
        fn get_name(&self) -> &str {
            &self.name
        }
        
        fn get_name_owned(&self) -> String {
            self.name.clone()
        }
    }
    
    #[test]
    fn test_user_creation() {
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
        assert!(user.posts.is_empty());
    }
    
    #[test]
    fn test_add_posts() {
        let mut user = User::new(1, "Bob".to_string(), "bob@example.com".to_string());
        
        user.add_post(101);
        user.add_post(102);
        user.add_post(103);
        
        assert_eq!(user.posts.len(), 3);
        assert_eq!(user.posts, vec![101, 102, 103]);
    }
    
    #[test]
    fn test_get_name_reference() {
        let user = User::new(1, "Charlie".to_string(), "charlie@example.com".to_string());
        let name_ref = user.get_name();
        
        assert_eq!(name_ref, "Charlie");
        // El usuario sigue siendo válido después de obtener la referencia
        assert_eq!(user.id, 1);
    }
    
    #[test]
    fn test_get_name_owned() {
        let user = User::new(1, "David".to_string(), "david@example.com".to_string());
        let name_owned = user.get_name_owned();
        
        assert_eq!(name_owned, "David");
        // El usuario original sigue siendo válido
        assert_eq!(user.get_name(), "David");
    }
    
    #[test]
    fn test_user_cloning() {
        let original = User::new(1, "Eve".to_string(), "eve@example.com".to_string());
        let cloned = original.clone();
        
        assert_eq!(original.id, cloned.id);
        assert_eq!(original.name, cloned.name);
        assert_eq!(original.email, cloned.email);
        assert_eq!(original.posts, cloned.posts);
        
        // Modificar el clon no afecta al original
        let mut cloned_mut = cloned.clone();
        cloned_mut.add_post(999);
        
        assert_eq!(original.posts.len(), 0);
        assert_eq!(cloned_mut.posts.len(), 1);
    }
    
    #[test]
    fn test_hashmap_operations() {
        let mut users = HashMap::new();
        users.insert(1, User::new(1, "Frank".to_string(), "frank@example.com".to_string()));
        users.insert(2, User::new(2, "Grace".to_string(), "grace@example.com".to_string()));
        
        // Test acceso inmutable
        assert!(users.get(&1).is_some());
        assert!(users.get(&3).is_none());
        
        // Test acceso mutable
        if let Some(user) = users.get_mut(&1) {
            user.add_post(201);
            assert_eq!(user.posts.len(), 1);
        }
        
        // Test iteración
        let mut count = 0;
        for (id, user) in &users {
            assert!(id > &0);
            assert!(!user.get_name().is_empty());
            count += 1;
        }
        assert_eq!(count, 2);
    }
    
    #[test]
    fn test_lifetime_safety() {
        let text = String::from("Hello World");
        let first_word = get_first_word_safe(&text);
        
        assert_eq!(first_word, "Hello");
        
        // El texto original sigue siendo válido
        assert_eq!(text, "Hello World");
    }
    
    fn get_first_word_safe(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    #[test]
    fn test_ownership_transfer() {
        let user = User::new(1, "Henry".to_string(), "henry@example.com".to_string());
        let name = user.get_name_owned();
        
        assert_eq!(name, "Henry");
        // El usuario original sigue siendo válido
        assert_eq!(user.get_name(), "Henry");
    }
    
    #[test]
    fn test_multiple_borrows() {
        let mut user = User::new(1, "Iris".to_string(), "iris@example.com".to_string());
        
        // Múltiples referencias inmutables
        let name1 = user.get_name();
        let name2 = user.get_name();
        
        assert_eq!(name1, "Iris");
        assert_eq!(name2, "Iris");
        
        // Referencia mutable después de que las inmutables se liberen
        user.add_post(301);
        assert_eq!(user.posts.len(), 1);
    }
    
    #[test]
    fn test_error_handling_with_option() {
        let mut users = HashMap::new();
        users.insert(1, User::new(1, "Jack".to_string(), "jack@example.com".to_string()));
        
        // Test con Option
        match users.get(&1) {
            Some(user) => {
                assert_eq!(user.get_name(), "Jack");
            }
            None => {
                panic!("Usuario debería existir");
            }
        }
        
        match users.get(&999) {
            Some(_) => {
                panic!("Usuario no debería existir");
            }
            None => {
                // Comportamiento esperado
            }
        }
    }
}

