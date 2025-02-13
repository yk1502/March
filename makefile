FLAGS = -C target-cpu=native



all:
	RUSTFLAGS="$(FLAGS)" cargo run --release


clean:
	cargo clean