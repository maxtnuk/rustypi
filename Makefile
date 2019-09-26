TARGET = aarch64-unknown-none
LIBNAME = rustypi
X86_64_JSON = test_x86_64.json
TARGET_PATH = target/$(TARGET)/release/$(LIBNAME)

boot_image:
	cargo xbuild --target $(X86_64_JSON)

objdump:
	cargo objdump --target $(TARGET) -- -d -no-show-raw-insn $(TARGET_PATH)

size:
	cargo size --target $(TARGET) $(TARGET_PATH)

nm:
	cargo nm --target $(TARGET) $(TARGET_PATH)

build:
	cargo xrustc --target=$(TARGET) --release
	cargo objcopy -- --strip-all -O binary $(TARGET_PATH) $(LIBNAME).bin

docker_rasp: build
	./docker_run.sh rasp

docker_rm:
	yes | docker-compose rm

gdb_qemu:
	arm-none-eabi-gdb -q $(TARGET_PATH)
