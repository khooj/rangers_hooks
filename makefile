all: test

test: test.obj
	GoLink.exe /console /entry _start test.obj kernel32.dll

test.obj: test.asm
	nasm -f win32 test.asm -o test.obj

run: test
	./test.exe