# 8-Bit CPU Emulator

## Added instructions
1. MOV
2. ADD
3. SUB
4. JMP (Jump)
5. JZ (Jump if zero)
5. JZ (Jump if not zero)
6. HLT (Halt)

# Usage
```bash
cargo run -- --f <filename.asc>
```

## Todo
- [ ] Assembler
    - [x] Lexer/Tokenizer
    - [ ] Add label support (supporting JMP/JZ/JNZ)
- [ ] Error handling
- [ ] Build Debugger
