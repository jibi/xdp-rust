xdp-rust
========

POC showing how to run Rust as an XDP (only SKB_MODE for now) program.

## How to run it

Download and patch Linux 5.2.4:
```sh
$ wget https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.2.4.tar.xz
$ tar xvf linux-5.2.4.tar.xz
$ cd linux-5.2.4
$ patch -p1 < ../xdp-rust/0001-XDP-Rust.patch
```

Compile and install it:
```sh
$ make
$ sudo make modules_install
$ sudo make headers_install INSTALL_HDR_PATH=/usr
$ sudo make install
```

Then compile and load the module (just run `make` in the `xdp-rust/mod`
directory and maybe change the dest IP/port of the program before).
