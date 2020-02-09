BTKNMLE (BlueTooth Keyboard aNd Mouse Low Energy)
=======

[![build](https://github.com/yskszk63/btknmle/workflows/build/badge.svg)](https://github.com/yskszk63/btknmle/actions)
[![codecov](https://codecov.io/gh/yskszk63/btknmle/branch/master/graph/badge.svg)](https://codecov.io/gh/yskszk63/btknmle)

Expose local keyboard and mouse as Bluetooth HID device.

WIP

Prerequisite
------------

Stop bluez if running.

```
sudo systemctl stop bluez
```

Build & Run
-----------

Archlinux

```
# build
sudo pacman -S libinput
git clone https://github.com/yskszk63/btknmle
cd btknmle
cargo build --release

# run
sudo target/release/btknmle
```

Docker (Arm etc..)

WIP
