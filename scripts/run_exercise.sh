#!/bin/bash

# 🦀 Rust Lab - Script de Ejecución de Ejercicios
# Este script facilita la ejecución de ejercicios y tests

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para mostrar ayuda
show_help() {
    echo -e "${BLUE}🦀 Rust Lab - Script de Ejecución${NC}"
    echo ""
    echo "Uso: $0 [COMANDO] [OPCIONES]"
    echo ""
    echo "Comandos disponibles:"
    echo "  run <ejercicio>     Ejecutar un ejercicio específico"
    echo "  test               Ejecutar todos los tests"
    echo "  test <categoría>   Ejecutar tests de una categoría específica"
    echo "  build              Compilar todos los ejercicios"
    echo "  clean              Limpiar archivos de compilación"
    echo "  doc                Generar documentación"
    echo "  list               Listar ejercicios disponibles"
    echo "  help               Mostrar esta ayuda"
    echo ""
    echo "Ejemplos:"
    echo "  $0 run ownership_basics"
    echo "  $0 test ownership"
    echo "  $0 build"
    echo "  $0 list"
}

# Función para listar ejercicios disponibles
list_exercises() {
    echo -e "${BLUE}📚 Ejercicios Disponibles:${NC}"
    echo ""
    
    echo -e "${YELLOW}🔐 Ownership & Borrowing:${NC}"
    echo "  - ownership_basics (con bugs intencionales)"
    echo "  - ownership_basics_fixed (solución corregida)"
    echo ""
    
    echo -e "${YELLOW}🚨 Error Handling:${NC}"
    echo "  - error_handling_basics (con bugs intencionales)"
    echo "  - error_handling_basics_fixed (solución corregida)"
    echo ""
    
    echo -e "${YELLOW}⚡ Concurrency:${NC}"
    echo "  - concurrency_basics (con bugs intencionales)"
    echo "  - concurrency_basics_fixed (solución corregida)"
    echo ""
    
    echo -e "${YELLOW}🚀 Performance:${NC}"
    echo "  - performance_optimization (con bugs intencionales)"
    echo "  - performance_optimization_fixed (solución corregida)"
    echo ""
    
    echo -e "${YELLOW}🧠 Memory Management:${NC}"
    echo "  - memory_management (con bugs intencionales)"
    echo "  - memory_management_fixed (solución corregida)"
    echo ""
}

# Función para ejecutar un ejercicio
run_exercise() {
    local exercise=$1
    
    if [ -z "$exercise" ]; then
        echo -e "${RED}❌ Error: Debes especificar el nombre del ejercicio${NC}"
        echo "Usa '$0 list' para ver ejercicios disponibles"
        exit 1
    fi
    
    echo -e "${BLUE}🦀 Ejecutando ejercicio: $exercise${NC}"
    echo ""
    
    if cargo run --bin "$exercise"; then
        echo ""
        echo -e "${GREEN}✅ Ejercicio '$exercise' ejecutado exitosamente${NC}"
    else
        echo ""
        echo -e "${RED}❌ Error al ejecutar el ejercicio '$exercise'${NC}"
        echo "Verifica que el ejercicio existe y está correctamente configurado"
        exit 1
    fi
}

# Función para ejecutar tests
run_tests() {
    local category=$1
    
    echo -e "${BLUE}🧪 Ejecutando tests...${NC}"
    echo ""
    
    if [ -n "$category" ]; then
        echo -e "${YELLOW}Ejecutando tests de la categoría: $category${NC}"
        if cargo test "$category"; then
            echo ""
            echo -e "${GREEN}✅ Tests de '$category' ejecutados exitosamente${NC}"
        else
            echo ""
            echo -e "${RED}❌ Error en tests de '$category'${NC}"
            exit 1
        fi
    else
        if cargo test; then
            echo ""
            echo -e "${GREEN}✅ Todos los tests ejecutados exitosamente${NC}"
        else
            echo ""
            echo -e "${RED}❌ Error en algunos tests${NC}"
            exit 1
        fi
    fi
}

# Función para compilar
build_project() {
    echo -e "${BLUE}🔨 Compilando proyecto...${NC}"
    echo ""
    
    if cargo build; then
        echo ""
        echo -e "${GREEN}✅ Proyecto compilado exitosamente${NC}"
    else
        echo ""
        echo -e "${RED}❌ Error de compilación${NC}"
        exit 1
    fi
}

# Función para limpiar
clean_project() {
    echo -e "${BLUE}🧹 Limpiando archivos de compilación...${NC}"
    echo ""
    
    cargo clean
    echo -e "${GREEN}✅ Limpieza completada${NC}"
}

# Función para generar documentación
generate_docs() {
    echo -e "${BLUE}📚 Generando documentación...${NC}"
    echo ""
    
    if cargo doc --open; then
        echo ""
        echo -e "${GREEN}✅ Documentación generada y abierta en el navegador${NC}"
    else
        echo ""
        echo -e "${RED}❌ Error al generar documentación${NC}"
        exit 1
    fi
}

# Main script logic
case "${1:-help}" in
    "run")
        run_exercise "$2"
        ;;
    "test")
        run_tests "$2"
        ;;
    "build")
        build_project
        ;;
    "clean")
        clean_project
        ;;
    "doc")
        generate_docs
        ;;
    "list")
        list_exercises
        ;;
    "help"|*)
        show_help
        ;;
esac
