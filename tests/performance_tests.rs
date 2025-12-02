//! Tests para los ejercicios de performance

#[cfg(test)]
mod performance_tests {
    use std::collections::HashMap;
    use std::time::Instant;
    
    // Importar las estructuras del ejercicio
    #[derive(Debug, Clone, PartialEq)]
    struct User {
        id: u32,
        name: String,
        email: String,
        posts: Vec<u32>,
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
        
        fn add_post(&mut self, post_id: u32) {
            self.posts.push(post_id);
            self.last_post_id = Some(post_id);
        }
        
        fn find_post(&self, post_id: u32) -> bool {
            self.posts.binary_search(&post_id).is_ok()
        }
        
        fn get_posts(&self) -> &[u32] {
            &self.posts
        }
    }
    
    #[test]
    fn test_user_creation() {
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
        assert!(user.posts.is_empty());
        assert_eq!(user.last_post_id, None);
    }
    
    #[test]
    fn test_user_posts() {
        let mut user = User::new(1, "Bob".to_string(), "bob@example.com".to_string());
        
        user.add_post(101);
        user.add_post(102);
        user.add_post(103);
        
        assert_eq!(user.posts.len(), 3);
        assert_eq!(user.posts, vec![101, 102, 103]);
        assert_eq!(user.last_post_id, Some(103));
    }
    
    #[test]
    fn test_user_post_search() {
        let mut user = User::new(1, "Charlie".to_string(), "charlie@example.com".to_string());
        
        user.add_post(101);
        user.add_post(102);
        user.add_post(103);
        
        // Test búsqueda exitosa
        assert!(user.find_post(101));
        assert!(user.find_post(102));
        assert!(user.find_post(103));
        
        // Test búsqueda fallida
        assert!(!user.find_post(104));
        assert!(!user.find_post(100));
    }
    
    #[test]
    fn test_vec_pre_allocation() {
        let start = Instant::now();
        
        // Test con pre-allocación
        let mut vec1 = Vec::with_capacity(1000);
        for i in 0..1000 {
            vec1.push(i);
        }
        
        let duration1 = start.elapsed();
        
        let start = Instant::now();
        
        // Test sin pre-allocación
        let mut vec2 = Vec::new();
        for i in 0..1000 {
            vec2.push(i);
        }
        
        let duration2 = start.elapsed();
        
        // Verificar que ambos vectores son iguales
        assert_eq!(vec1, vec2);
        
        // Pre-allocación debería ser más rápida (aunque puede variar)
        println!("Con pre-allocación: {:?}", duration1);
        println!("Sin pre-allocación: {:?}", duration2);
    }
    
    #[test]
    fn test_string_optimization() {
        let start = Instant::now();
        
        // Test concatenación eficiente
        let mut result = String::with_capacity(10000);
        for i in 0..1000 {
            result.push_str(&format!("Item{}, ", i));
        }
        
        let duration = start.elapsed();
        println!("Tiempo de concatenación: {:?}", duration);
        
        assert!(result.len() > 0);
        assert!(result.contains("Item0"));
        assert!(result.contains("Item999"));
    }
    
    #[test]
    fn test_hashmap_optimization() {
        let start = Instant::now();
        
        // Test HashMap con pre-allocación
        let mut map = HashMap::with_capacity(10000);
        for i in 0..10000 {
            map.insert(i, i * 2);
        }
        
        let duration = start.elapsed();
        println!("Tiempo de inserción: {:?}", duration);
        
        // Test búsqueda
        let start = Instant::now();
        for i in 0..1000 {
            assert_eq!(map.get(&i), Some(&(i * 2)));
        }
        let duration = start.elapsed();
        println!("Tiempo de búsqueda: {:?}", duration);
    }
    
    #[test]
    fn test_iterator_optimization() {
        let users = create_test_users(1000);
        
        let start = Instant::now();
        
        // Test iteración optimizada (una sola pasada)
        let result: Vec<&str> = users.iter()
            .filter(|u| u.id % 2 == 0)
            .map(|u| &u.name[..])
            .collect();
        
        let duration = start.elapsed();
        println!("Tiempo de iteración optimizada: {:?}", duration);
        
        assert_eq!(result.len(), 500); // 1000 / 2
        assert!(result.iter().all(|name| name.starts_with("User")));
    }
    
    #[test]
    fn test_clone_avoidance() {
        let users = create_test_users(100);
        
        let start = Instant::now();
        
        // Test sin clones innecesarios
        let mut total_posts = 0;
        for user in &users {
            let posts = user.get_posts();
            total_posts += posts.len();
        }
        
        let duration = start.elapsed();
        println!("Tiempo sin clones: {:?}", duration);
        
        assert_eq!(total_posts, 0); // Los usuarios no tienen posts
    }
    
    #[test]
    fn test_fibonacci_optimization() {
        let start = Instant::now();
        
        // Test Fibonacci optimizado
        let result = fibonacci_optimized(35);
        
        let duration = start.elapsed();
        println!("Tiempo de Fibonacci optimizado: {:?}", duration);
        
        assert_eq!(result, 9227465);
    }
    
    #[test]
    fn test_memory_layout_optimization() {
        // Test estructura optimizada
        let start = Instant::now();
        
        let mut users = Vec::with_capacity(1000);
        for i in 0..1000 {
            let user = User::new(
                i,
                format!("User{}", i),
                format!("user{}@example.com", i),
            );
            users.push(user);
        }
        
        let duration = start.elapsed();
        println!("Tiempo de creación optimizada: {:?}", duration);
        
        assert_eq!(users.len(), 1000);
    }
    
    #[test]
    fn test_slice_patterns() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        let start = Instant::now();
        
        let mut sum = 0;
        for chunk in data.chunks(2) {
            match chunk {
                [a, b] => sum += a + b,
                [a] => sum += a,
                _ => {}
            }
        }
        
        let duration = start.elapsed();
        println!("Tiempo de slice patterns: {:?}", duration);
        
        assert_eq!(sum, 55); // 1+2+3+4+5+6+7+8+9+10
    }
    
    #[test]
    fn test_cow_optimization() {
        use std::borrow::Cow;
        
        let start = Instant::now();
        
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
        println!("Tiempo de Cow: {:?}", duration);
        
        assert_eq!(results.len(), 1000);
        assert!(results.iter().any(|v| v == "even"));
        assert!(results.iter().any(|v| v.contains("odd_")));
    }
    
    #[test]
    fn test_benchmark_comparison() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        // Test método ineficiente (múltiples pasadas)
        let start = Instant::now();
        let count = data.iter().count();
        let filtered: Vec<_> = data.iter().filter(|&&x| x % 2 == 0).collect();
        let mapped: Vec<_> = filtered.iter().map(|&&x| x * 2).collect();
        let duration1 = start.elapsed();
        
        // Test método eficiente (una sola pasada)
        let start = Instant::now();
        let result: Vec<_> = data.iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 2)
            .collect();
        let duration2 = start.elapsed();
        
        println!("Múltiples pasadas: {:?}", duration1);
        println!("Una sola pasada: {:?}", duration2);
        
        assert_eq!(mapped, result);
    }
    
    // Funciones auxiliares
    fn create_test_users(count: usize) -> Vec<User> {
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
}


