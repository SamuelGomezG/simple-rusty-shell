use std::env;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Stdio};

const HELP_TEXT: &str = "Simple Rusty Shell - v0.1
Available built-in commands:
cd <path>   : Change current directory to <path>
help        : Show this message.
exit | :q   : Close the shell.\n
";

const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

fn print_prompt(stdout: &mut impl Write, current_dir: &Path) -> io::Result<()> {
    write!(stdout, "{}{} {}>> ", BLUE, current_dir.display(), RESET)?;
    stdout.flush()
}

fn print_error(stderr: &mut impl Write, msg: &str) -> io::Result<()> {
    write!(stderr, "{}Error: {}{}\n\n", RED, msg, RESET)?;
    stderr.flush()
}

fn run_cd(stderr: &mut impl Write, arg: Option<&str>, current_dir: &mut PathBuf) -> io::Result<()> {
    let path = match arg {
        Some(input_path) => PathBuf::from(input_path),
        None => match env::home_dir() {
            Some(home) => home,
            None => return print_error(stderr, "Impossible to get your home directory."),
        },
    };

    if let Err(e) = env::set_current_dir(&path) {
        print_error(stderr, &e.to_string())?;
    } else {
        *current_dir = env::current_dir()?;
    }
    Ok(())
}

fn run_external(
    stderr: &mut impl Write,
    program: &str,
    args: std::str::SplitWhitespace<'_>,
) -> io::Result<()> {
    match process::Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Err(e) => print_error(stderr, &e.to_string()),
        Ok(mut child) => {
            child.wait()?;
            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    let mut current_dir = env::current_dir()?;
    let mut input_buffer = String::new();

    loop {
        print_prompt(&mut stdout, &current_dir)?;

        input_buffer.clear();
        stdin.read_line(&mut input_buffer)?;

        let command_string = input_buffer.trim();
        let mut command_iter = command_string.split_whitespace();

        if let Some(program) = command_iter.next() {
            match program {
                "cd" => run_cd(&mut stderr, command_iter.next(), &mut current_dir)?,
                "help" => write!(stdout, "{}", HELP_TEXT)?,
                "exit" | ":q" => process::exit(0),
                _ => run_external(&mut stderr, program, command_iter)?,
            }
        }
    }
}
