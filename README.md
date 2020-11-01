# Pulsar SSD1306: Block Height OLED Display for Cosmos Validators

This little package displays the current block height for Cosmos chains on SSD1306 OLED displays.

SSD1306 displays are commonly available OLED displays for Raspberry Pi and other IoT devices. Check out [Adafruit OLED bonnet](https://www.adafruit.com/product/3531), and [Adafruit PiOLDED](https://www.adafruit.com/product/3527).

## OS Configuration

### Raspberry Pi OS

Use [raspi-config](https://www.raspberrypi.org/documentation/configuration/raspi-config.md) to enable I2C by default.

### Ubuntu

Install Raspi Config:

1. Go to [raspberrypi.org](https://archive.raspberrypi.org/debian/pool/main/r/raspi-config/) and pick the latest version.

2. Download and install:

```sh
# install dependencies
$ sudo apt-get install lua5.1

# get raspi-config
$ wget https://archive.raspberrypi.org/debian/pool/main/r/raspi-config/raspi-config_[DATE]_all.deb

# install
$ sudo dpkg -i raspi-config_[DATE]_all.deb
```

Run `raspi-config` and enable I2C.

### Test device

Before running this sample, you can check that your OLED display is detected by installing `i2c-tools`, then running `i2cdetect -y 1`. The first connected SSD1306 will usually be at address `0x3c`. This package assumes `0x3c`. If your device is at a different address, then you may need to modify the code for it to work.

```sh
$ sudo apt-get install i2c-tools

$ sudo i2cdetect -y 1
     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
00:          -- -- -- -- -- -- -- -- -- -- -- -- --
10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
30: -- -- -- -- -- -- -- -- -- -- -- -- 3c -- -- --
40: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
70: -- -- -- -- -- -- -- --
```

## Building the code

The easiest way to build is with [Cross](https://github.com/rust-embedded/cross). 

Cross-compile a release for your platform. For Ubuntu 64-bit:

```sh
cross build --release --target aarch64-unknown-linux-gnu
```

After the build finishes, copy it to your Raspberry Pi

```sh
scp target/aarch64-unknown-linux-gnu/release/pulsar-ssd1306 user@ip:/home/user
```

## Running on validator

Pulsar SSD1306 currently works by reading the block height from a Tendermint node running on localhost.

SSH to your Pi and run it with:

```sh
sudo ./pulsar-ssd1306
```

If all goes well, you should see your block height on the display! Obviously, the chain should already be running.

## Note

Querying the block height every second from Tendermint is not very efficient. A better approach is to monitor `systemd`, and update the display whenever there's a new log entry. I'll leave that as an exercise for an ambitious developer. PRs welcome!
