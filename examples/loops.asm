; move imm 3 to register b
mov b, 3
; move imm 1 to register a
mov a, 1

; delcare a label
loop:
    ; subtract a from b
    sub b, a
    ; jump to label "loop" until the zero flag is not set
    jnz loop

hlt
