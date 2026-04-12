; ARCH x86_64
; Generated ASM file. Modifications will not be preserved

BITS 64
ORG 0x100000

_start:
    call main
.halt:
    hlt
    jmp .halt

add:
    push rbp
    mov rbp, rsp
    sub rsp, 40
    mov [rbp-8], rcx
    mov [rbp-16], rdx
    mov rax, [rbp-8]
    mov rbx, [rbp-16]
    mov rcx, rax
    add rcx, rbx
    mov rax, rcx
    mov rsp, rbp
    pop rbp
    ret
    mov rsp, rbp
    pop rbp
    ret

main:
    push rbp
    mov rbp, rsp
    sub rsp, 24
    mov rdx, 5
    mov rax, 3
    mov rcx, rdx
    mov rdx, rax
    call add
    mov rbx, rax
    mov [rbp-8], rbx
    mov rcx, [rbp-8]
    mov rax, rcx
    mov rsp, rbp
    pop rbp
    ret
    mov rsp, rbp
    pop rbp
    ret


