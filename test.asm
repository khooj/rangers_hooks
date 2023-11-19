STD_OUTPUT_HANDLE equ -11
NULL equ 0

global _start
extern ExitProcess, GetStdHandle, WriteConsoleA

section	.data
s1 db 'HELLO, WORLDd', 0 ;source
len equ $-s1

section .bss
dummy resd 1
s2 resb 20              ;destination
	
section .text
_start:	                ;tell linker entry point
   mov dl, 0 
   test dl, dl
   jl _end
	
; loop_here:
;    lodsb
   ;or      al, 20h
;    stosb
;    loop    loop_here	
   ; cld
   ; rep	movsb
	
   push STD_OUTPUT_HANDLE
   call GetStdHandle

   push NULL
   push dummy
   push len ;message length
   push s1 ;msg to write
   push eax ;handle
   call WriteConsoleA

_end:
   push NULL
   call ExitProcess
	