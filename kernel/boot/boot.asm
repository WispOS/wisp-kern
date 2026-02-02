[BITS 16]
[ORG 0x7C00]

_start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti
    
    mov [boot_drive], dl
    
    mov ah, 0x0E
    mov al, 'S'
    int 0x10
    
    xor ax, ax
    mov es, ax
    mov bx, 0x0500
    
    mov cx, 32
    
.read_loop:
    push cx
    
    mov ah, 0x02
    mov al, 1
    mov ch, 0
    mov cl, [sector]
    mov dh, 0
    mov dl, [boot_drive]
    int 0x13
    jc disk_error
    
    inc byte [sector]
    add bx, 512
    
    pop cx
    loop .read_loop
    
    mov ah, 0x0E
    mov al, 'D'
    int 0x10
    
    cli
    lgdt [gdt_descriptor]
    
    mov eax, cr0
    or al, 1
    mov cr0, eax
    
    jmp 0x08:protected_mode

disk_error:
    mov ah, 0x0E
    mov al, 'E'
    int 0x10
    cli
    hlt

[BITS 32]
protected_mode:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000
    
    mov byte [0xB8000], 'P'
    mov byte [0xB8001], 0x0F
    
    jmp 0x500

align 4
gdt_start:
    dd 0, 0
gdt_code:
    dw 0xFFFF
    dw 0
    db 0
    db 0x9A
    db 0xCF
    db 0
gdt_data:
    dw 0xFFFF
    dw 0
    db 0
    db 0x92
    db 0xCF
    db 0
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

boot_drive: db 0
sector: db 2

times 510-($-$$) db 0
dw 0xAA55