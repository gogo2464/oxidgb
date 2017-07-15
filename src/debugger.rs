/**
 * debugger.rs
 *
 * The debugger provides a command line interface for debugging Oxidgb and Gameboy
 *  games.
**/

use rustyline::error::ReadlineError;
use rustyline::Editor;

use core::cpu::CPU;
use core::cpu::GameboyDebugger;

pub struct CommandLineDebugger {
    pub enabled : bool,
    pub shutdown : bool,
    pub breakpoints : Vec<u16>,
    editor : Editor<()>
}

impl CommandLineDebugger {
    pub fn build() -> CommandLineDebugger {
        CommandLineDebugger {
            enabled : true,
            shutdown : false,
            breakpoints : Vec::new(),
            editor : Editor::<()>::new()
        }
    }
}

impl GameboyDebugger for CommandLineDebugger {
    fn debug(&mut self, cpu : &mut CPU) {
        // Check to see if we have any breakpoints
        if self.breakpoints.contains(&cpu.regs.pc) {
            println!("Hit breakpoint: {:04X}", cpu.regs.pc);
            self.enabled = true;
        }

        if !self.enabled {
            return
        }

        let mut instruction = cpu.mem.read(cpu.regs.pc) as u16;

        if instruction == 0xCB {
            instruction = (instruction << 8) | cpu.mem.read(cpu.regs.pc + 1) as u16;
        }

        println!("{:04X} @ {:04X}", instruction, cpu.regs.pc);

        loop {
            println!("Debugger:");
            match self.editor.readline("") {
                Ok(line) => {
                    let mut args = line.trim().split(" ");

                    match args.nth(0) {
                        Some("") => {
                            println!("Stepping...");
                            break;
                        }
                        Some("r") |
                        Some("run") => {
                            println!("Running...");
                            self.enabled = false;
                            break;
                        },
                        Some("i") |
                        Some("regs") => {
                            println!("af = {:04X}", cpu.regs.get_af());
                            println!("bc = {:04X}", cpu.regs.get_bc());
                            println!("de = {:04X}", cpu.regs.get_de());
                            println!("hl = {:04X}", cpu.regs.get_hl());
                            println!("sp = {:04X}", cpu.regs.sp);
                            println!("pc = {:04X}", cpu.regs.pc);
                        },
                        Some("mem") => {
                            match args.nth(0) {
                                Some(value) => match u16::from_str_radix(&value, 16) {
                                    Ok(number) => {
                                        println!("{:04X} = {:02X}", number, cpu.mem.read(number));
                                    },
                                    _ => {
                                        println!("Invalid number.");
                                    }
                                },
                                _ => {
                                    println!("Requires an argument.");
                                }
                            }
                        },
                        Some("mems") => {
                            match args.nth(0) {
                                Some(value) => match u16::from_str_radix(&value, 16) {
                                    Ok(number) => {
                                        for i in 0 .. 0xF {
                                            print!("{:04X} = {:02X} ", number + i, cpu.mem.read(number + i));
                                        }
                                        println!();
                                    },
                                    _ => {
                                        println!("Invalid number.");
                                    }
                                },
                                _ => {
                                    println!("Requires an argument.");
                                }
                            }
                        },
                        Some("break") => {
                            match args.nth(0) {
                                Some(value) => match u16::from_str_radix(&value, 16) {
                                    Ok(number) => {
                                        match self.breakpoints.iter().position(|&r| r == number) {
                                            Some(position) => {
                                                println!("Removing breakpoint: {:04X}", number);
                                                self.breakpoints.remove(position);
                                            },
                                            _ => {
                                                println!("Adding breakpoint: {:04X}", number);
                                                self.breakpoints.push(number);
                                            }
                                        }
                                    },
                                    _ => {
                                        println!("Invalid number.");
                                    }
                                },
                                _ => {
                                    println!("Requires an argument.");
                                }
                            }
                        },
                        None => {}
                        _ => {
                            println!("Unknown command.");
                        }
                    }
                },
                Err(ReadlineError::Interrupted) |
                Err(ReadlineError::Eof) => {
                    println!("Closing...");
                    self.shutdown = true;
                    self.enabled = false;
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    self.shutdown = true;
                    self.enabled = false;
                    break
                }
            }
        }
    }
}
