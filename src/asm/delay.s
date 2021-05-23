# Delay for a certain number of cycles.
.global _ASM_DELAY_CYCLES
_ASM_DELAY_CYCLES:
    srli a0, a0, 1 # Divide by two.
    1:
    addi a0, a0, -1
    bnez a0, 1b
