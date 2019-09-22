KERNEL_FILE				= target/kernel/debug/bootimage-rustypi.bin

DOCKER_CMD        = docker run -it --rm
DOCKER_ARG_CURDIR = -v $(shell pwd):/work -w /work

DOCKER_EXEC_QEMU     = qemu-system-aarch64 -M raspi3 -drive format=raw,file=$(KERNEL_FILE)
CONTAINER_UTILS   = andrerichter/raspi3-utils

build:
	bootimage build --target kernel.json
qemu: build
		$(DOCKER_CMD) $(DOCKER_ARG_CURDIR) $(CONTAINER_UTILS) \
			$(DOCKER_EXEC_QEMU)
