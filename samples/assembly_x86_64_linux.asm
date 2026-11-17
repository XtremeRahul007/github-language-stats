; nasm -f elf64 assembly_x86_64_linux.asm && ld -o hello assembly_x86_64_linux.o
section .data
msg db "Hello, World!", 10
len equ $ - msg
section .text
global _start
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, len
    syscall
    mov rax, 60
    xor rdi, rdi
    syscall
