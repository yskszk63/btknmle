BTKNMLE (BlueTooth Keyboard aNd Mouse Low Energy)
=================================================

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

Run
---

via Docker

```
docker run \
  --mount type=volume,source=btknmle,target=/var/lib/btknmle \
  --device /dev/input \
  --mount type=bind,source=/run/udev,target=/run/udev,readonly \
  --mount type=bind,source=/sys/class/bluetooth,target=/sys/class/bluetooth,readonly \
  --mount type=bind,source=/sys/kernel/debug,target=/sys/kernel/debug \
  --net host \
  --cap-add net_admin \
  yskszk63/btknmle:latest
```

### tags

- yskszk63/btknmle:latest
- yskszk63/btknmle:amd64
- yskszk63/btknmle:arm32v7
- yskszk63/btknmle:arm32v5

Using
-----

1. Start btknmle [Device]
2. Press any key to start Advertise [Device]
3. Scan bluetooth devices [Host]
4. Connect to btknmle named `btknmle` [Host]
5. Passkey input if required [Device]
6. Connected

Build
-----

Archlinux

```
sudo pacman -S libinput
git clone https://github.com/yskszk63/btknmle
cd btknmle
cargo build --release
```
