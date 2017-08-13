Oxidgb
======

Yet another work-in-progress Gameboy emulator for Rust.

Features
--------

- Broken code
- No audio emulation
- Crashes

Running
-------

Windows/Linux/macOS, assuming you have `cargo` on your PATH:

```bash
cargo run --manifest-path sdl_frontend/Cargo.toml
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