.text
.globl _start
_start:
mov $60, %rax
mov $17, %rdi
syscall