#shell
cargo xbuild --target kernel.json
bootimage build --target kernel.json
docker-compose run qemu
