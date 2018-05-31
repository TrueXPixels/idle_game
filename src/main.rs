extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::process::Command;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

// Static Variables
static DATA_PATH: &'static str = "data.json";

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct User {
    name: String,
    balance: i64,
}

fn main() {

    // Create File Path
    let path = Path::new(DATA_PATH);
    let display = path.display();

    // Open File
    let file = File::open(&path);

    // Check if file was found
    if file.is_ok() {
        println!("File Found!");
    } else { // If not, run the following

        // Request User Input - Name
        let name = input("Welcome to Rust Idler! Looks like it's your first time, what should we call you?\n -> ")
            .expect("Something went wrong!");

        // Parse Data
        let data = User { name: name, balance: 100 };

        // Format data into a JSON string
        let res = serde_json::to_string_pretty(&data);

        // Create File
        let mut created_file = match File::create(DATA_PATH) {
            Err(why) => panic!("Can't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Write File - & Send success message
        match created_file.write_all(&res.ok().unwrap().as_bytes()) {
            Err(why) => panic!("Can't write to {}: {}", display, why.description()),
            Ok(_) => println!("\nWell {}... We've added $100 to your account so you can get started, best of luck!", data.name),
        }


    }

}

fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write; // Use Trait
    print!("{}", user_message); // Add message to print buffer
    io::stdout().flush()?; // Flush buffer (output everything in print)

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_right().to_owned())
}
