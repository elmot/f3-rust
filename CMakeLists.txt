#THIS FILE IS AUTO GENERATED FROM THE TEMPLATE! DO NOT CHANGE!
SET(CMAKE_SYSTEM_NAME Generic)
SET(CMAKE_SYSTEM_VERSION 1)
cmake_minimum_required(VERSION 3.7)

# specify cross compilers and tools
SET(CMAKE_C_COMPILER_WORKS 1)
SET(CMAKE_C_COMPILER arm-none-eabi-gcc)
SET(CMAKE_CXX_COMPILER_WORKS 1)
SET(CMAKE_CXX_COMPILER arm-none-eabi-g++)
set(CMAKE_ASM_COMPILER  arm-none-eabi-gcc)
set(CMAKE_AR arm-none-eabi-ar)
set(CMAKE_OBJCOPY arm-none-eabi-objcopy)
set(CMAKE_OBJDUMP arm-none-eabi-objdump)
set(SIZE arm-none-eabi-size)

SET(LINKER_SCRIPT ${CMAKE_SOURCE_DIR}/STM32F303VCTx_FLASH.ld)

#Cortex-M4 without FPU
#SET(RUST_TARGET_ARCH thumbv7em-none-eabi)
SET(RUST_TARGET_ARCH thumbv7em-none-eabihf)

#Enable FPU
SET(FPU_FLAGS "-mfloat-abi=hard -mfpu=fpv4-sp-d16")

SET(COMMON_FLAGS
    "-mcpu=cortex-m4 ${FPU_FLAGS} -mthumb -mthumb-interwork -ffunction-sections -fdata-sections \
    -g -fno-common -fmessage-length=0 -specs=nosys.specs -specs=nano.specs")

SET(CMAKE_CXX_FLAGS_INIT "${COMMON_FLAGS} -std=c++11")
SET(CMAKE_C_FLAGS_INIT "${COMMON_FLAGS} -std=gnu99")
SET(CMAKE_EXE_LINKER_FLAGS_INIT "-Wl,-gc-sections,--print-memory-usage -T ${LINKER_SCRIPT}")

PROJECT(f3-rust C CXX ASM)
set(CMAKE_CXX_STANDARD 11)

add_definitions(-D__weak=__attribute__\(\(weak\)\) -D__packed=__attribute__\(\(__packed__\)\) -DUSE_HAL_DRIVER -DSTM32F303xC)

file(GLOB_RECURSE SOURCES "startup/*.*" "Drivers/*.*" "Core/*.*")

include_directories(Core/Inc Drivers/STM32F3xx_HAL_Driver/Inc Drivers/STM32F3xx_HAL_Driver/Inc/Legacy Drivers/CMSIS/Device/ST/STM32F3xx/Include Drivers/CMSIS/Include)

#Rust part start
set(RUST_MODULE_NAME f3blink)
set(RUST_MODULE_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/rust/${RUST_MODULE_NAME})
set(RUST_LIB ${RUST_MODULE_ROOT}/target/${RUST_TARGET_ARCH}/debug/lib${RUST_MODULE_NAME}${CMAKE_STATIC_LIBRARY_SUFFIX})
set_source_files_properties(
        ${RUST_LIB}
        PROPERTIES
        EXTERNAL_OBJECT true
        GENERATED true
)

set_property(DIRECTORY APPEND
        PROPERTY ADDITIONAL_MAKE_CLEAN_FILES ${RUST_MODULE_ROOT}/target
)

add_custom_command(OUTPUT ${RUST_LIB}
        COMMAND cargo build --target=${RUST_TARGET_ARCH}
        DEPENDS ${RUST_MODULE_ROOT}/src/*.*
        COMMENT "Compiling rust"
        WORKING_DIRECTORY ${RUST_MODULE_ROOT}
        USES_TERMINAL
        )
#Rust part end

add_executable(${PROJECT_NAME}.elf ${SOURCES} ${RUST_LIB} ${LINKER_SCRIPT})

set(CMAKE_EXE_LINKER_FLAGS
    "${CMAKE_EXE_LINKER_FLAGS} -Wl,-Map=${PROJECT_BINARY_DIR}/${PROJECT_NAME}.map")

set(HEX_FILE ${PROJECT_BINARY_DIR}/${PROJECT_NAME}.hex)
set(BIN_FILE ${PROJECT_BINARY_DIR}/${PROJECT_NAME}.bin)

add_custom_command(TARGET ${PROJECT_NAME}.elf POST_BUILD
        COMMAND ${CMAKE_OBJCOPY} -Oihex $<TARGET_FILE:${PROJECT_NAME}.elf> ${HEX_FILE}
        COMMAND ${CMAKE_OBJCOPY} -Obinary $<TARGET_FILE:${PROJECT_NAME}.elf> ${BIN_FILE}
        COMMENT "Building ${HEX_FILE}
Building ${BIN_FILE}")
