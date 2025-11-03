.data
hello:
  .ascii "Hello, World!\n"

.text
.globl _start

_start:
  mov $1, %rax #puts sys_write number into rax
  mov $1, %rdi #puts stdout descriptor in rdi
  mov $hello, %rsi #puts adress of string in rsi
  mov $14, %rdx #put length of string in rdx
  syscall       #calls kernel to execute syscall (whatever is in rax)

  mov $60, %rax #puts sys_exit number into rax
  xor %rdi, %rdi #set exit status to 0
  syscall       #call kernel to terminate
