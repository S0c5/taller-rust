build-send:
	make build-flipper
	make send-flipper

build-flipper:
	cargo build --release

send-flipper:
	storage send ../target/thumbv7em-none-eabihf/release/hello-world.fap /ext/apps/Examples/hello-world.fap