```
 ⡇ ⡇⢹⠁⢹⠁⡇ ⣏⡉ ⡇⢸⣏⡉⡇⣏⡱⡏⢱⡎⢱
 ⠧⠤⠇⠸ ⠸ ⠧⠤⠧⠤ ⠟⠻⠧⠤⠇⠇⠱⠧⠜⠣⠜
on the ESP32
```
![GitHub License](https://img.shields.io/github/license/hi-squeaky-things/little-weirdo-esp32)

#no-std optimized additive/subtractive/granular/sample based synthesizer and sequencer.

The Little Weirdo synthesizer library is a high-level Rust library for creating digital audio synthesizers. It provides a flexible and modular architecture for building synthesizers, allowing (squeaky sound) developers/designers to create a wide range of sounds and effects.

**[Hi Squeaky Things](https://www.hi-squeaky-things.nl)** can happen at any time™. _Little Weirdo_ is ready to squeak, squuuueak, squeeeeeaak, squeaaaaaaaaak!

See the Little Weirdo in embedded context action, checkout **[Little Squeaky Machine Hardware!](https://github.com/hi-squeaky-things/little-squeaky-machine-hardware)** or
buy the embedded reference hardware @ **[Hi Squeaky Things](https://www.hi-squeaky-things.nl)** to support the development of this library.


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
Current usage: 32464
Memory Layout: 
Internal | █████████████████░░░░░░░░░░░░░░░░░░ | Used: 50% (Used 32464 of 64000, free: 31536)

...
> patch change in 176 µs (max 22µs)

...
----- 1 sec cycle @ 44.1KHz (max 22µs ) -----
> average - total compute time spend  714895
> clock :: average 16µs
> deadline passed 8691x,
> highest process time 37µs (@469)
> lowest  process time 14µs
> headroom = 28 %
```

> [!NOTE]
> On average Little Weirdo spend 16µs for each sample to be calculated on a ESP32S3. This leaves us with a headroom of 28%. Switching a patch can NOT happen in one cycle (average 125 µs), need attention