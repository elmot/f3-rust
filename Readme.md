Embedded Rust Sample Project
====

* Languages: C + Rust
* Target Platform: ARM Cortex-M4F
* Build System: CMake + Cargo
* Tools: CLion, STM32CubeMX, GCC, OpenOCD
* Target Board: STM32F3-Discovery
* Toolchains: gcc-arm-none-eabi

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

Quick Start
---
One needs to:
 1. Open the project in CLion
 2. Build `SETUP_RUST` config (do not run it) - that will
 3. Run `OCD f3-rust` run configuration
