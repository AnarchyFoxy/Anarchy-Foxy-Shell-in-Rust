use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use std::path::Path;
use std::error::Error;

// Function to display the ASCII text logo
fn display_logo() {
    println!("    __    _  _    __    ____   ___  _   _  _  _    ____  _____  _  _  _  _ ");
    println!("   /__\\  ( \\( )  /__\\  (  _ \\ / __)( )_( )( \\/ )  ( ___)(  _  )( \\/ )( \\/ )");
    println!("  /(__)\\  )  (  /(__)\\  )   /( (__  ) _ (  \\  /    )__)  )(_)(  )  (  \\  / ");
    println!(" (__)(__)(_)\\_)(__)(__)(_)\\_) \\___)(_) (_) (__)   (__)  (_____)(_/\\_) (__)  ");
    println!("   ___  _   _  ____  __    __   ");
    println!(" / __)( )_( )( ___)(  )  (  )  ");
    println!(" \\__ \\ ) _ (  )__)  )(__  )(__ ");
    println!(" (___/(_) (_)(____)(____)(____)");
}

// Function to parse the input command and arguments
fn parse_command(input: &str) -> (String, Vec<String>) {
    let mut tokens = input.trim().split_whitespace();
    let command = tokens.next().unwrap_or("").to_string();
    let arguments: Vec<String> = tokens.map(|s| s.to_string()).collect();
    (command, arguments)
}

// Function to execute a command with arguments
fn execute_command(command: &str, args: &[String]) -> Result<(), Box<dyn Error>> {
    match command {
        "cd" => {
            if args.is_empty() {
                let home_dir = env::var("HOME")?;
                env::set_current_dir(&home_dir)?;
            } else {
                let new_dir = Path::new(&args[0]);
                env::set_current_dir(new_dir)?;
            }
        }
        _ => {
            Command::new(command)
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?;
        }
    }
    Ok(())
}

fn main() {
    display_logo();

    loop {
        print!("Anarchy Foxy Shell $ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let (command, arguments) = parse_command(&input);

        if command == "exit" {
            println!("Anarchy Foxy Says Goodbye!");
            break;
        }

        match execute_command(&command, &arguments) {
            Ok(_) => continue,
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}