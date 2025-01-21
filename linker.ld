ENTRY(_start)

SECTIONS {
    . = 0x100000;
    _payload_start = .;

    .text ALIGN(4K) : {
        *(.text._start)
        *(.text*)
    }

    .rodata ALIGN(4K) : {
        *(.rodata*)
    }

    .data ALIGN(4K) : {
        *(.data*)
    }

    .bss ALIGN(4K) : {
        *(.bss*)
        *(COMMON)
        . = ALIGN(8);
    }

    _payload_end = .;

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment)
    }
}