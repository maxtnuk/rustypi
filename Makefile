TARGET = aarch64-unknown-none
LIBNAME = rustypi
X86_64_JSON = test_x86_64.json

boot_image:
	cargo xbuild --target $(X86_64_JSON)

objdump:
	cargo objdump --target $(TARGET) -- -disassemble -print-imm-hex target/$(TARGET)/release/$(LIBNAME)

build:
	cargo xrustc --target=$(TARGET) --release
	cargo objcopy -- --strip-all -O binary target/$(TARGET)/release/$(LIBNAME) $(LIBNAME).bin

docker_rasp:
	./docker_run.sh rasp

docker_rm:
	yes | docker-compose rm
