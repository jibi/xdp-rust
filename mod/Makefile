obj-m         += xdp_rust.o
xdp_rust-objs := xdp_rust_mod.o xdp_rust_prog.o

all:
	cd ./xdp_rust_prog && cargo clean && RUSTFLAGS="--emit obj -C opt-level=z" cargo build --release
	cp xdp_rust_prog/target/release/deps/*.o xdp_rust_prog.o
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
