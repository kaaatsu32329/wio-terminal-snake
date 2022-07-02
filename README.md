# 🐍**Snake**🐍
## Overview
This project implements the generic snake game for the arm based Wio Terminal embedded system. It
was made possible with the help of examples in the 
[atsamd_hal/boards/wio_terminal](https://github.com/atsamd-rs/atsamd/tree/master/boards/wio_terminal)
repository. The code is also based from the same repository. The coordinates for the food cells in the game are 
generated by a lightweight pseudo-random number generation crate. A demonstration of the game can be found on 
[this](https://youtube.com/shorts/gzPQQpc5Q3M?feature=share) link.


## Building and Flashing (Linux)
First step would be to add the proper udev rules for flashing. Refer to udev-rules bit of [this](https://github.com/atsamd-rs/atsamd#getting-code-onto-the-device-with-bootloaders-bossac)
snippet. Then, you can do: 

```bash
$ cargo build --release
```

```bash
$ arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/snake snake.bin
```

```bash
$ bossac -p ttyACM0 -e -w -v --offset=0x4000 snake.bin 
```

You may also follow [this](https://github.com/atsamd-rs/atsamd#getting-code-onto-the-device-with-bootloaders-bossac)
snippet to help you build and flash the rust program.


## License
MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)