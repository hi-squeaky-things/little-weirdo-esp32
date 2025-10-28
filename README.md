```
 ⡇ ⡇⢹⠁⢹⠁⡇ ⣏⡉ ⡇⢸⣏⡉⡇⣏⡱⡏⢱⡎⢱
 ⠧⠤⠇⠸ ⠸ ⠧⠤⠧⠤ ⠟⠻⠧⠤⠇⠇⠱⠧⠜⠣⠜
on the ESP32
```
![GitHub License](https://img.shields.io/github/license/hi-squeaky-things/little-weirdo-esp32)

A Rust #no-std optimized wave table synthesizer for embedded devices, the esp32 in this case.

> [!CAUTION]
> This project is actively being developed with frequent breaking changes. APIs may shift, features are incomplete, and stability is not guaranteed. Use at your own risk and expect regular updates that might require code adjustments. Have fun!

> [!IMPORTANT]
> **Hi Squeaky Things** can happen at any time. _Little Weirdo_ is ready to squeak, squuuueak, squeeeeeaak, squeaaaaaaaaak!


## How to use it

```
$ cargo build --release
$ espflash flash --monitor target/xtensa-esp32s3-none-elf/release/little_weirdo_esp32
```

## Performance

```
...
-:= > Set CPU Speed to _240MHz
> performance run start

HEAP INFO
Size: 64000
Current usage: 12140
Memory Layout: 
Internal | ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ | Used: 18% (Used 12140 of 64000, free: 51860)

...
> patch change in 7 µs (max 22µs)

...
----- 1 sec cycle @ 44.1KHz (max 22µs ) -----
> average - total compute time spend  528831
> clock :: average 11µs
> deadline passed 0x,
> highest process time 0µs (@0)
> lowest  process time 7µs
> headroom = 47 %
```

> [!NOTE]
> On average Little Weirdo spend 11µs for each sample to be calculated on a ESP32S3. This leaves us with a headroom of 47%. Also switching a patch can happen in one cycle (average 7 µs)