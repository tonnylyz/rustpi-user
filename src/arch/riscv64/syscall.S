.macro SYSCALL number
.global syscall_\number
syscall_\number:
li a7, \number
ecall #0
ret
.endm

SYSCALL 1
SYSCALL 2
SYSCALL 3
SYSCALL 4
SYSCALL 5
SYSCALL 6
SYSCALL 7
SYSCALL 8
SYSCALL 9
SYSCALL 10
SYSCALL 11
SYSCALL 12
