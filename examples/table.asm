; Table of 3
mov b, 3
mov c, 10 ; Counter

loop:
    add a, b
    sub c, 1
    jnz loop
hlt
