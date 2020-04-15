.global asm_page_fault_handler
asm_page_fault_handler:
    jal page_fault_handler_stub
1:  nop
    j   1b // TODO: restore context