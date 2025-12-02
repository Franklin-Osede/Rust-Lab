# 🦀 Rust Lab - Bug Spotting & Testing Mastery

## 🎯 Objetivo
Repositorio para demostrar habilidades avanzadas en Rust a través de ejercicios de bug spotting, testing y debugging. Cada bloque contiene 10 ejercicios progresivos que demuestran diferentes aspectos del lenguaje.

## 📚 Estructura del Proyecto

```
rust_lab/
├── exercises/           # Ejercicios organizados por categorías
│   ├── ownership_borrowing/    # Ownership & Borrowing
│   ├── error_handling/         # Error Handling & Recovery
│   ├── concurrency/           # Concurrency & Async
│   ├── performance/           # Performance & Optimization
│   └── memory_management/     # Memory Management
├── tests/              # Tests de integración
├── docs/               # Documentación y explicaciones
└── scripts/            # Scripts de automatización
```

## 🚀 Cómo usar este repositorio

### **Comandos Básicos:**
```bash
# Ejecutar ejercicios
cargo run --bin ownership_basics
cargo run --bin ownership_basics_fixed
cargo run --bin error_handling_basics

# Ejecutar tests
cargo test

# Ver documentación
cargo doc --open
```

### **Script de Automatización:**
```bash
# Listar ejercicios disponibles
./scripts/run_exercise.sh list

# Ejecutar ejercicio específico
./scripts/run_exercise.sh run ownership_basics

# Ejecutar todos los tests
./scripts/run_exercise.sh test

# Compilar proyecto
./scripts/run_exercise.sh build
```

### **GitHub Actions:**
- ✅ **CI/CD Automático**: Tests en cada push
- ✅ **Multi-versión**: Prueba en stable, beta, nightly
- ✅ **Linting**: Clippy y rustfmt automáticos
- ✅ **Coverage**: Reportes de cobertura de código
- ✅ **Documentación**: Generación automática de docs

## 🎥 Contenido LinkedIn
Cada ejercicio incluye:
- ✅ Código con bugs intencionales
- 🐛 Explicación del bug y su impacto
- 🔧 Solución paso a paso
- 🧪 Tests que demuestran el comportamiento
- 📹 Guión para video explicativo

## 🏆 Habilidades demostradas
- **Memory Safety**: Ownership, borrowing, lifetimes
- **Error Handling**: Result, Option, panic recovery
- **Concurrency**: Threads, async/await, channels
- **Performance**: Zero-cost abstractions, optimization
- **Testing**: Unit tests, integration tests, property-based testing
