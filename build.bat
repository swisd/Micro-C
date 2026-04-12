@echo off
set /p name=".asm file name (no extension): "
%cd%/LocalNASM/nasm -f win64 %name%.asm -o %name%.o
rem C:\msys64\mingw64\bin\ld.exe %name%.o -o %name%.exe
C:\msys64\ucrt64\bin\gcc.exe %name%.o -o %name%.exe