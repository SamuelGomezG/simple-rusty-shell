use std::env;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process;

const HELP_TEXT: &str = "Simple Rusty Shell - v0.1
Available built-in commands:
cd <path>   : Change current directory to <path>
help        : Show this message.
exit | :q   : Close the shell.\n
";

fn main() -> io::Result<()> {
    loop {
        let mut stdout = io::stdout().lock();
        let current_dir = env::current_dir()?;
        stdout.write_all(format!("{}>> ", current_dir.display()).as_bytes())?;
        stdout.flush()?;

        let mut input_buffer = String::new();
        let mut stdin = io::stdin().lock();
        stdin.read_line(&mut input_buffer)?;

        let command_string = input_buffer.trim();
        let mut command_iter = command_string.split_whitespace();
        let command = command_iter.next();
        let mut args = command_iter;

        let mut stderr = io::stderr().lock();
        if let Some(program) = command {
            match program {
                "cd" => {
                    if let Some(input_path) = args.next() {
                        let path = Path::new(input_path);

                        match env::set_current_dir(path) {
                            Ok(_) => {}
                            Err(_) => {
                                stderr.write_all(b"Provided directory does not exist.\n\n")?;
                                stderr.flush()?;
                            }
                        }
                    } else {
                        match env::home_dir() {
                            Some(path) => {
                                env::set_current_dir(path).ok();
                            }
                            None => {
                                stderr.write_all(b"Impossible to get your home directory.\n\n")?;
                                stderr.flush()?;
                            }
                        }
                    }
                }
                "help" => {
                    stdout.write_all(HELP_TEXT.as_bytes())?;
                }
                "exit" | ":q" => {
                    process::exit(0);
                }
                _ => {
                    let command_output = process::Command::new(program).args(args).output();

                    match command_output {
                        Err(e) => {
                            stderr.write_all(format!("Error: {}\n\n", e).as_bytes())?;
                            stderr.flush()?;
                        }
                        Ok(output) => {
                            stdout.write_all(&output.stdout)?;
                            stdout.write_all(b"\n")?;
                            stdout.flush()?;
                        }
                    }
                }
            }
        }
    }
}
