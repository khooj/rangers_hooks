STD_OUTPUT_HANDLE equ -11
NULL equ 0

global _start
extern ExitProcess, GetStdHandle, WriteConsoleA

section	.data
s1 db 'HELLO, WORLDd', 0 ;source
len equ $-s1
; addr db '%d', 0
; addr_len equ $-addr

section .bss
dummy resd 1
s2 resb 20              ;destination
	
section .text
_start:	                ;tell linker entry point
   mov dl, 0 
   test dl, dl
   jl _end

   mov eax, len
   mov edx, 0
   mov ebx, 4
   div ebx

   mov esi, s1
   sub esp, len
   sub esp, edx
   mov edi, esp
   mov edx, esp
   mov ecx, len
   repne movsb
	
; loop_here:
;    lodsb
   ;or      al, 20h
;    stosb
;    loop    loop_here	
   ; cld
   ; rep	movsb


	
   push STD_OUTPUT_HANDLE
   call GetStdHandle
   ; mov ecx, eax

   push NULL
   push dummy
   push len ;message length
   push edx ;msg to write
   push eax ;handle
   call WriteConsoleA

   ; push NULL
   ; push dummy
   ; push addr_len ;message length
   ; push addr ;msg to write
   ; push ecx ;handle
   ; call WriteConsoleA

_end:
   push NULL
   call ExitProcess
	