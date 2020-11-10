MinIMU9-IIO
===========================================

Poll IIO interface for polulu MinIMU9 device


### Configure the driver from userpace

Load kernel modules
```bash
modprobe st_lsm6dsx
modprobe st_lsm6dsx_i2c
modprobe st_gyro
modprobe st_gyro_i2c
modprobe st_magn
modprobe st_magn_i2c
```

Load i2c driver
``` bash
echo lis3mdl 0x1e > /sys/bus/i2c/devices/i2c-1/new_device
echo lsm6ds3 0x6b > /sys/bus/i2c/devices/i2c-1/new_device
```

Configure frequency
```bash
echo 80 > /sys/bus/iio/devices/iio:device0/sampling_frequency
echo 416 > /sys/bus/iio/devices/iio:device1/sampling_frequency
echo 416 > /sys/bus/iio/devices/iio:device2/sampling_frequency
```

### Run

```bash
minimu9-iio
```

### Build

prerequisites

``` bash
install docker
cargo install cross
cargo install --force cargo-make
cargo make --makefile ./Cargo.toml release-darwin
export PATH=$HOME/.cargo/bin:$PATH
```

debug build
```
cargo build
```

build all relases
```
cargo make --makefile ./Cargo.toml release-all
```

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org).
