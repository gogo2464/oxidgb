Oxidgb
======

Yet another work-in-progress Gameboy emulator for Rust.

Features
--------

- Broken code
- ~~No~~ audio emulation
- Crashes

Running
-------

Windows/Linux/macOS, assuming you have `cargo` on your PATH:

```bash
cargo run --release
```

Running with a debugger
-------

This does not works on Libretro. So do:

```bash
cd glutin_frontend
cargo run --features debugger -- --load '<PATH TO YOUR GAMEBOY ROM>'
```

Then you will be able to use some commands inspirated by gdb such as:
```
'' (just press enter) => to step
'r' or 'run' => to run
'i' or 'regs' => to get registers state
'mem X' => read one opcode not disassembled at address X
'mems X' => read 16 opcodes not disassembled at address X 
'break'
```


Credits
-------

- [BGB](http://bgb.bircd.org/), for comparisons and debugging games
- [The Pandocs](http://bgb.bircd.org/pandocs.htm), for extensive documentation
- [DP's Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf), for getting me started
- [The Cycle Accurate Gameboy Docs](https://github.com/AntonioND/giibiiadvance/blob/master/docs/TCAGBD.pdf)
  for catching a whole load of edge cases

License
-------

Oxidgb is licensed under the MIT license. This can be found [here](LICENSE).
