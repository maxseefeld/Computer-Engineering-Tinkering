BITS 16     ; set the assembly code to use 16-bit mode

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

    ; Initialize the NVIDIA GPU
    mov ax, 0x4f00       ; set the function to get the VBE controller information
    mov di, 0x00         ; set the destination address for the controller information
    mov bx, 0x03          ; set the VBE mode to 1024x768x24bpp
    int 0x10             ; call the BIOS interrupt to get the controller information

    mov ax, 0x4f02       ; set the function to set the VBE mode
    mov bx, 0x03          ; set the VBE mode to 1024x768x24bpp
    int 0x10             ; call the BIOS interrupt to set the VBE mode

    ; Initialize the USB driver
    mov ax, 0x0300     ; BIOS function to check USB support
    int 0x15           ; call the BIOS interrupt to check USB support

    jc no_usb_support  ; jump to no_usb_support if USB is not supported

    mov ax, 0x1d00     ; BIOS function to check number of USB controllers
    int 0x15           ; call the BIOS interrupt to check number of USB controllers
    mov bx, ax         ; save number of controllers in BX

    ; loop through all USB controllers
    xor cx, cx         ; set the controller index to 0
    mov si, usb_dev_tbl; set the pointer to the device table
    usb_loop:
      mov ax, 0x1d01     ; BIOS function to check USB controller info
      int 0x15           ; call the BIOS interrupt to get controller info
      jc no_usb_ctrl     ; jump if no controller is found
      mov byte [si], dl  ; save controller type
      mov byte [si+1], dh; save controller interface
      mov byte [si+2], ch; save number of ports
      mov byte [si+3], cl; save protocol
      add si, 4          ; increment the device table pointer
      inc cx             ; increment the controller index
      cmp cx, bx         ; compare the index with the number of controllers
      jb
