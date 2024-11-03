#!/bin/sh

target=$1

work_dir="./target/thumbv8m.main-none-eabihf/debug"

arm-none-eabi-objcopy -O binary $target $target.bin

dfu-util -a 0 -s 0x08000000 -D $target.bin
