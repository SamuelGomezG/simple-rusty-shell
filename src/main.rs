use std::io::{self, BufRead, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    loop {
        let mut stdout = io::stdout().lock();
        stdout.write_all(b">> ")?;
        stdout.flush()?;

        let mut input_buffer = String::new();
        let mut stdin = io::stdin().lock();
        stdin.read_line(&mut input_buffer)?;

        let command_string = input_buffer.trim();

        if command_string == ":q" || command_string == "exit" {
            break;
        }

        let mut command_iter = command_string.split_whitespace();
        let command = command_iter.next();

        if let Some(program) = command {
            let command_output = Command::new(program).args(command_iter).output()?;
        }
    }

    Ok(())
}
