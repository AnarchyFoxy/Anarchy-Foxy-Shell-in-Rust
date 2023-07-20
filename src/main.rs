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
