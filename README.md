# 8-Bit CPU Emulator

## CPU Architecture
- Word Size
    - **Data Width:** 8 bits
    - **Address width:** 16 bits
    - **Address space:** 64 KB (0x0000- 0xFFFF)

## Supported Instructions

| Instruction | Syntax       |
| ----------- | ------------ |
| MOV         | mov reg, imm |
| ADD         | add r1, r2   |
| SUB         | sub r1, r2   |
| JMP         | jmp addr     |
| JZ          | jz addr      |
| JNZ         | jnz addr     |
| HLT (Halt)  | hlt          |


## Registers
| Register | Size   | Description                    |
| -------- | ------ | ------------------------------ |
| A        | 8-bit  | General                        |
| B        | 8-bit  | General                        |
| C        | 8-bit  | General                        |
| D        | 8-bit  | General                        |
| PC       | 16-bit | Program Counter                |
| SP       | 16-bit | Stack pointer (unused for now) |

## Flags
| Flag  | Description  |
| ----- | ------------ |
| Z     | Zero Flag    |
| C     | Carry/Borrow |


# Usage
```bash
cargo run -- --f <examples/filename.asc>
```

## Todo
- [ ] Assembler
    - [x] Lexer/Tokenizer
    - [ ] Add label support (supporting JMP/JZ/JNZ)
- [ ] Error handling
- [ ] Build Debugger
