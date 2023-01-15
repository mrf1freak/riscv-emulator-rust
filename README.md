# RISCV Emulator in Rust

A small RISC-V emulator written in rust

### Memory Size (Required)

Specify the size of simulated memory with `-m <size>` or `--memory <size>`

```
./emulator -m 4096 instructions.bin
00000000    lui   x4,0xabcde
00000004    auipc x4,0xabcde
00000008    jal   x1,0x8
00000010    jalr  x4,16(x1)
0000001c    bne   x11,x11,0xfffffff8
00000020    blt   x0,x0,0xfffffff4
00000024    bge   x10,x0,0xfffffff0
00000028    bltu  x0,x0,0xffffffec
0000002c    bgeu  x0,x10,0xffffffe8
00000030    beq   x0,x0,0x8
00000038    bne   x0,x11,0x8
00000040    blt   x10,x0,0x8
```

---

### Interactive Mode

Can be used by passing the "interactive" flag (-i or --interactive) in options

```
./emulator -i -m 4096 instructions.bin
---INTERACTIVE MODE---
<space> - run next command
m - dump memory
r - dump registers
q - quite

00000000    lui   x4,0xabcde
00000004    auipc x4,0xabcde
00000008    jal   x1,0x8
00000010    jalr  x4,16(x1)
```

