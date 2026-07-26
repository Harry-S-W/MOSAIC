/*
This file is part of MOSAIC.

MOSAIC is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

MOSAIC is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
MOSAIC. If not, see <https://www.gnu.org/licenses/>.
*/

/*
core shell file which parses commands and allocates them to seperate files which exeute the command

shell.rs sort of acts like a receptionist
*/

/*

NEW SHELL

- For easier development in the future, the shell has been redesigned where it is a series of match startments. There are defined root commands
and inside the root commands there is a match statement for an argument (i.e. project + open). This way we can edit individual commands and they sort of act
as their own family. 
*/

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::shell::projectManager::session::{SessionData, SystemVerifier};
use crate::analysis::{run};

pub fn shell_initiation(_session: &mut SessionData) -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    println!("MOSAIC -- v{}-alpha pre-release (GLPv3)\n", version); // opening message
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("MOSAIC >> ");
        println!(""); 
        match readline {
            Ok(line) => {
                let args = match shell_words::split(&line) {
                    Ok(args) => args,
                    Err(_) => {
                        eprintln!("[MOSAIC ERROR] Mismatched quotes in command.");
                        continue;
                    }
                };
                if args.is_empty() {
                    continue;
                }

                // We match on a slice of references: &[&str]
                let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

                match arg_refs.as_slice() {
                    [] => continue,
                    ["quit"] | ["exit"] => break,
                    
                    ["session"] => {
                        let data = SessionData::read_session_data();
                        println!("Session Data:\n{:#?}", data);
                    }

                    // project commands
                    ["project", secondary_command, ..] => {
                        match *secondary_command {
                            "open" => {
                                // path checker
                                if let Some(path) = arg_refs.get(2) {
                                    println!("Opening project at directory: {}", path);
                                    // SystemVerifier::project_open(path);
                                } else {
                                    eprintln!("[MOSAIC ERROR] 'project open' requires a file directory path.");
                                }
                            }
                            "delete" => { // more here to keep the idea - NOT FUNCTIONAL
                                if let Some(id) = arg_refs.get(2) {
                                    println!("Deleting project: {}", id);
                                } else {
                                    eprintln!("[MOSAIC ERROR] 'project delete' requires a target ID or path.");
                                }
                            }
                            _ => println!("Unknown project command: '{}'", secondary_command),
                        }
                    }

                    // Participant commands
                    ["participant", "verify"] => {
                        match SystemVerifier::participant() {
                            Ok(path) => println!("Participant Path: {:?}", path),
                            Err(err) => eprintln!("[MOSAIC ERROR] {}", err),
                        }
                    }

                    // UMD
                    ["umd", secondary_command, ..] => {
                        match *secondary_command {
                            "run" => {
                                // runs the UMD based on driver
                                // CALLS MOSAIC-UMD for UMD conversion and driver call
                            }
                            _ => println!("Unknown UMD command: '{}'", secondary_command),
                        }
                        //let input_path = "/Users/harrywoodhouse/MOSAIC/MOSAIC/test_data/v15044gf0000d1dlc67og65r2deqmhd0.csv"; // will be 
                        //let output_path = "/Users/harrywoodhouse/MOSAIC/MOSAIC/MOSAIC-Engine/data/";
                        //run::init(input_path, output_path);
                        
                    }

                    // Unknown
                    [unknown, ..] => {
                        println!("Unknown command root: '{}'", unknown);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => { // Ctrl-C
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => { // Ctrl-D
                println!("CTRL-D");
                break
            },
            Err(err) => {
                eprintln!("Something went wrong: {:?}", err); 
            }
        }
        println!(""); 

        
    }
    Ok(())
}

