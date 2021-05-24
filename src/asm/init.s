.section .text.init
.global _start
_start:
    la   sp, __sp
    call _rust_entry
