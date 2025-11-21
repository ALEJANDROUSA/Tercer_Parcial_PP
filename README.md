# Parcial Tercer Corte — Paradigmas de Programación  
## Regresión Lineal aplicada a Concurrencia, Programación Orientada a Aspectos (AOP) y Rust

---

# **Objetivos del Parcial**

1. Diseñar una solución concurrente (sin implementación) para el cálculo de regresión lineal usando el paradigma de concurrencia y cálculo de PI como base conceptual.
2. Diseñar una solución usando el paradigma de *Aspect-Oriented Programming* (AOP) que permita realizar regresión lineal sin mezclar la lógica principal con preocupaciones transversales.
3. Implementar la regresión lineal en **Rust**, comparar su desempeño con Python y documentar los resultados.
4. Integrar diagramas UML (componentes y secuencia) para los puntos 1 y 2, explicando detalladamente cada parte.
5. Organizar todo en un README final para subir a GitHub.

---

# Punto 1 — Diseño usando Concurrencia  
*(Diseño, no implementación)*

La idea base se inspira en el cálculo concurrente de PI mediante división del problema en subtareas (workers).  
Aquí aplicamos ese mismo enfoque: **dividir el cálculo de los gradientes de la regresión lineal en múltiples tareas concurrentes**, logrando paralelismo en los cálculos iterativos.

---

## **1.1 Diseño General**

La regresión lineal por gradiente descendente requiere:
- Calcular predicciones `y_pred`
- Calcular errores `error = y_pred - y`
- Calcular gradientes `dw` y `db`
- Actualizar parámetros `w` y `b`

En lugar de hacer esto secuencialmente, se divide el dataset en *N chunks* para que cada worker compute:
- Un gradiente parcial de `dw`
- Un gradiente parcial de `db`

Luego un **coordinador** suma los gradientes parciales y actualiza los parámetros globales.

---

## **1.2 Componentes del Diseño Concurrente**

- **Coordinator**  
  Orquesta el proceso, envía los subtareas a los workers, recolecta resultados y actualiza parámetros.

- **Worker**  
  Calcula gradientes parciales sobre un fragmento del dataset.

- **Data Splitter**  
  Divide `X` y `y` en subconjuntos del mismo tamaño.

- **Gradient Aggregator**  
  Suma los gradientes parciales recibidos de los workers.

- **Shared Model State**  
  Contiene `(w, b)` y es actualizado por el Coordinator.

---

## **1.3 Diagrama 1 — Arquitectura Concurrente (Componentes)**

```
<img width="1028" height="922" alt="DIAGRAMA 1 — Concurrencia (Arquitectura Componentes)" src="https://github.com/user-attachments/assets/2e98a775-aba6-4941-bd4f-08045fa04fde" />

```

---

## **1.4 Explicación del Diagrama**

### **Coordinator**
- Recibe los hiperparámetros (learning rate, epochs).
- Llama al *Data Splitter* para dividir los datos.
- Envía tareas iguales a los workers.
- Recolecta resultados y actualiza el modelo.

### **Workers**
- Cada worker calcula:
  - error parcial  
  - partial_dw  
  - partial_db  
- Trabajan en paralelo y reportan al Coordinator.

### **Gradient Aggregator**
- Recibe todos los gradientes parciales.
- Los suma matemáticamente.
- Devuelve el gradiente real que se usará para la actualización.

### **Shared Model State**
- Los parámetros globales se actualizan solo después de cada iteración (epoch).

---

## **1.5 Diagrama 2 — Concurrencia (Secuencia)**

```
<img width="1400" height="685" alt="DIAGRAMA 2 — Concurrencia (Secuencia)" src="https://github.com/user-attachments/assets/2746b64e-a0ee-477a-a20b-6bf2c362bdb4" />

```

---

## **1.6 Explicación del Diagrama**

1. **Coordinator inicia epoch.**
2. Solicita al Data Splitter dividir los datos.
3. Envía tareas a cada Worker.
4. Los Workers calculan gradientes parciales.
5. Envían resultados al Aggregator.
6. Aggregator combina los gradientes.
7. Coordinator actualiza `(w, b)`.
8. Inicia la siguiente epoch.

---

# Punto 2 — Diseño usando AOP (Aspect-Oriented Programming)  
*(Diseño, no implementación)*

---

## **2.1 Diseño General AOP**

El objetivo es **separar la lógica principal** (entrenar la regresión lineal) de las *preocupaciones transversales*, como:

- Logging de cada epoch  
- Monitoreo del costo  
- Medición de tiempo  
- Validación de datos  
- Control de hiperparámetros  

En AOP, estas funcionalidades se implementan mediante *Aspectos* que se “inyectan” sin modificar el código original.

---

## **2.2 Componentes del Diseño AOP**

### **Core Model**
- Contiene solo la lógica matemática de regresión lineal.
- No tiene prints, logs, validaciones, nada externo.

### **Logging Aspect**
- Registra cada epoch, gradientes, MSE.

### **Monitoring Aspect**
- Mide el tiempo de ejecución de cada ciclo.

### **Validation Aspect**
- Verifica que los datos no estén vacíos o corruptos.

### **Hyperparameter Aspect**
- Puede modificar dinámicamente valores como learning rate (annealing).

---

## **2.3 Diagrama 3 — AOP (Componentes)**

```
<img width="881" height="581" alt="DIAGRAMA 3 — AOP (Componentes)" src="https://github.com/user-attachments/assets/fc3f4157-5f4f-460a-adc1-79c5068b0994" />

```

---

## **2.4 Explicación del Diagrama**

### **Core Model**
- Mantiene únicamente:
  - cálculo del error,
  - cálculo de gradientes,
  - actualización de parámetros.

### **Aspect Weaving Engine**
- Inyector que combina el Core con los Aspectos en tiempo de compilación o ejecución.

### **Aspectos**
- Cada aspecto intercepta un punto de ejecución (“join point”), por ejemplo:
  - inicio de epoch,
  - actualización de parámetros,
  - cálculo de costo.

---

## **2.5 Diagrama 4 — AOP (Secuencia)**

```
<img width="1047" height="553" alt="DIAGRAMA 4 — AOP (Secuencia)" src="https://github.com/user-attachments/assets/2366f21e-d9c5-4295-a0bd-ba83f503758c" />

```

---

## **2.6 Explicación del Diagrama**

1. El usuario inicia el entrenamiento.  
2. El *Aspect Weaving Engine* intercepta la llamada y activa:
   - Validation Aspect
   - Logging Aspect
   - Monitoring Aspect  
3. Se ejecuta el Core Model para un epoch.  
4. Al finalizar, los aspectos vuelven a ejecutarse:
   - logs,
   - métricas,
   - monitoreo.  
5. Se repite hasta completar las epochs.

---

# Punto 3 — Implementación en Rust + Comparación con Python

---

## **3.1 Implementación en Rust**

El archivo final del código se llama:

**`regresion_lineal.rs`**

---

## **3.2 Comparación de Desempeño entre Python y Rust**

### Tiempo de ejecución estimado (1000 epochs, dataset simple)

| Lenguaje | Tiempo estimado | Motivo |
|---------|-----------------|--------|
| **Python (NumPy)** | 45–70 ms | NumPy es rápido pero Python tiene overhead interpretado |
| **Rust** | 3–7 ms | Rust compila a binario nativo, sin overhead |

---

### Uso de CPU

| Lenguaje | CPU | Descripción |
|---------|-----|-------------|
| Python | Medio | NumPy usa optimizaciones pero el intérprete añade peso |
| Rust | Bajo–Medio | Código nativo optimizado sin VM |

---

### Uso de memoria

| Lenguaje | Memoria | Razón |
|---------|---------|-------|
| Python | Alta | Carga del intérprete + NumPy |
| Rust | Muy baja | No usa VM ni garbage collector |

---

## Ventajas comparativas

| Criterio | Python | Rust |
|---------|--------|------|
| Facilidad de desarrollo | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| Velocidad | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Control de memoria | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| Seguridad | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| Ecosistema científico | ⭐⭐⭐⭐⭐ | ⭐⭐ |

---

# Conclusiones Finales

1. El paradigma de concurrencia permite acelerar el proceso mediante paralelización del cálculo de gradientes.  
2. El paradigma AOP separa preocupaciones, permitiendo un código más limpio y modular.  
3. Rust supera ampliamente a Python en rendimiento para regresión lineal por gradiente descendente gracias a su compilación a binario nativo.  
4. Los diagramas UML complementan la documentación y explican de forma clara la arquitectura y la secuencia de eventos.

---

