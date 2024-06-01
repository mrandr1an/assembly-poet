#!/bin/bash
t=$1
nasm -f elf64 $1.nasm -o $1.o &&
ld helloworld.o -o $1 &&
./$1
