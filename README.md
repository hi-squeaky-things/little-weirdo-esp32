```
 ⡇ ⡇⢹⠁⢹⠁⡇ ⣏⡉ ⡇⢸⣏⡉⡇⣏⡱⡏⢱⡎⢱
 ⠧⠤⠇⠸ ⠸ ⠧⠤⠧⠤ ⠟⠻⠧⠤⠇⠇⠱⠧⠜⠣⠜
on the ESP32
```

A Rust #no-std optimized wave table synthesizer for embedded devices, the esp32 in this case.

> [!CAUTION]
> This project is actively being developed with frequent breaking changes. APIs may shift, features are incomplete, and stability is not guaranteed. Use at your own risk and expect regular updates that might require code adjustments. Have fun!

> [!IMPORTANT]
> **Hi Squeaky Things** can happen at any time. _Little Weirdo_ is ready to squeak, squuuueak, squeeeeeaak, squeaaaaaaaaak!


## How to use it

```
$ cargo build --release
$ espflash flash --monitor target/xtensa-esp32-none-elf/release/little_weirdo_esp32
```
