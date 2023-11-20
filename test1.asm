
STD_OUTPUT_HANDLE equ -11
NULL equ 0

global _start
extern ExitProcess, GetStdHandle, WriteConsoleA

section	.data
val1 dd 1
val2 dd 2
val3 dd 3

section .bss
dummy resd 1
	
section .text
_start:	                ;tell linker entry point

    mov eax, 2
    mov edi, val1
    mov ecx, 3
    test ecx, ecx
    repne scasd

_end:
   push NULL
   call ExitProcess
	