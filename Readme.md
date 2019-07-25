Embedded Rust Sample Project
====

* Languages: C + Rust
* Target Platform: ARM Cortex-M4F
* Build System: CMake + Cargo
* Tools: CLion, STM32CubeMX, GCC, OpenOCD
* Target Board: STM32F3-Discovery

File structure
---
```
+---Core                    // C sources + generated files
|   +---Inc
|   \---Src
+---Drivers                // C libraries
+---rust
|   \---f3blink            // Rust module
|       +---src
\---startup                // MCU ASM startup code
```

