# {if:just}
fw_target := "riscv32imafc-esp-espidf"

# Flash the board and open the serial monitor (probe attached)
[group('firmware')]
flash bin="{name}" profile="debug":
    cd firmware && espflash flash --monitor \
        --bootloader target/{{ fw_target }}/{{ profile }}/build/bootloader.bin \
        target/{{ fw_target }}/{{ profile }}/{{ bin }}

# Build the firmware image
[group('firmware')]
build:
    cd firmware && cargo build --release
# {endif:just}
