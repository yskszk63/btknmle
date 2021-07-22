BTKNMLE (BlueTooth Keyboard aNd Mouse Low Energy)
=================================================

[![build](https://github.com/yskszk63/btknmle/workflows/build/badge.svg)](https://github.com/yskszk63/btknmle/actions)
[![codecov](https://codecov.io/gh/yskszk63/btknmle/branch/master/graph/badge.svg)](https://codecov.io/gh/yskszk63/btknmle)
[![dependency status](https://deps.rs/repo/github/yskszk63/btknmle/status.svg)](https://deps.rs/repo/github/yskszk63/btknmle)

Turn your keyboard and mouse connected to your computer into a bluetooth hid device.

WIP

Runtime Requirements
--------------------

- Linux 5.8+
- libinput

(Sorry. Only linux is supported.)

Build Requirements
------------------

Rust 1.51+

Prerequisite
------------

Stop bluez if running.

```
sudo systemctl stop bluetooth.service
```

Run
---

via [Docker](https://github.com/yskszk63/btknmle/pkgs/container/btknmle)

```bash
docker run --rm \
  --mount type=volume,source=btknmle,target=/var/lib/btknmle \
  --device /dev/input --mount type=bind,source=/run/udev,target=/run/udev,readonly \
  --mount type=bind,source=/sys/class/bluetooth,target=/sys/class/bluetooth,readonly \
  --net host --cap-add net_admin \
  ghcr.io/yskszk63/btknmle:latest
```

or

[Download form Nightly Release Page](https://github.com/yskszk63/btknmle/releases/tag/nightly)

- pre build binary
- deb package

arch: amd64 / arm / armv7 / arm64

usage

```
btknmle

USAGE:
    btknmle [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -v, --verbosity
    -V, --version      Prints version information

OPTIONS:
    -d, --device-id <device-id>    [env: BTKNMLE_DEVID=] [default: 0]
        --grab <grab>              [env: BTKNMLE_GRAB=]
    -f, --var-file <var-file>      [env: BTKNMLE_VAR_FILE=] [default: /var/lib/btknmle/db.toml]
    ```

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

```bash
sudo pacman -S libinput
git clone https://github.com/yskszk63/btknmle
cd btknmle
cargo build --release
```

Tested device
-------------

Currently ThinkPad A285 (ArchLinux) as device and Pixel3 as host only.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
