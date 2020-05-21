FROM andrerichter/raspi3-utils

ENV RUSTYPI /source/bootimage-rustypi.bin
ADD target/kernel/debug/bootimage-rustypi.bin /source

EXPOSE 3333
EXPOSE 5900

WORKDIR /source

CMD ["qemu-system-aarch64 -M raspi3 -gdb tcp::3333 -drive format=raw,file=${RUSTYPI} -vnc :0 -d in_asm -serial null -serial stdio"]