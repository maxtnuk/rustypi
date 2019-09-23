FROM ubuntu:19.10

USER root

WORKDIR	/workspace

RUN apt-get update && \
    apt-get -y install qemu qemu-system && \
    apt-get autoremove -q -y && \
    apt-get clean -q -y
    
