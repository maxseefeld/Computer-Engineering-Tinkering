[BITS 16]   ; set the assembly code to use 16-bit mode

jmp start   ; jump to the start of the code

; BIOS data area
; TODO: Define the BIOS data area

start:
    mov ax, 0x0000   ; set the segment register
    mov ds, ax       ; to the data segment
    mov es, ax       ; and the extra segment

    mov ah, 0x0e     ; set the function to display a character
    mov al, 'H'      ; set the character to display
    int 0x10         ; call the BIOS interrupt to display the character

    mov ah, 0x0e     ; set the function to display a character
    mov al, 'i'      ; set the character to display
    int 0x10         ; call the BIOS interrupt to display the character

    mov ah, 0x0e     ; set the function to display a character
    mov al, '!'      ; set the character to display
    int 0x10         ; call the BIOS interrupt to display the character

    cli              ; clear the interrupt flag
    hlt              ; halt the CPU

times 510 - ($ - $$) db 0   ; fill the remaining space with zeroes
dw 0xaa55                  ; add the boot signature
