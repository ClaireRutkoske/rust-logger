// Hide warnings for unused code.
// #![allow(dead_code)]

extern crate time;

use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::convert::AsRef;
use std::string::String;

// Timestamp
fn timestamp() -> i64 {
  let timespec = time::get_time();
    
  let mills: i64 = (timespec.sec as i64 * 1000)+ (timespec.nsec as i64 / 1000 / 1000);
    
  mills
}

fn main() {
    
  // Create a directory, returns `io::Result<()>`
  match fs::create_dir("log") {
    Err(why) => println!("! {:?}", why.kind()),
    Ok(_) => {},
  }
    
  // Create a file, returns `io::Reults<()>`
  match OpenOptions::new().create(true).write(true).open("log/log") {
    Err(why) => println!("! {:?}", why.kind()), 
    Ok(_) => {},
  }
    
  loop {
      
    let mut file = OpenOptions::new().write(true).append(true).open("log/log").unwrap(); 
      
    // The log entry
    println!("Log entry:");
    let mut entry = String::new();
    io::stdin().read_line(&mut entry).expect("Failed to read.");
    let entry = entry.trim();
      
      
    // Tags
    println!("Tag the entry (comma-separated):");
    let mut tags = String::new();
    io::stdin().read_line(&mut tags).expect("Failed to read.");
    let tags = tags.trim();
      
    writeln!(file, "{:?}, {:?}, {:?}", timestamp(), entry, tags).unwrap();
      
    println!("Success!");
      
    // Add another
    println!("Type `1` to add another, or `0` to exit.");
    let mut cont = String::new();
    io::stdin().read_line(&mut cont).expect("Failed to read.");
      
    let cont: u32 = match cont.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
      
    match cont {
      1 => println!("Excellent! Let's continue."),
      0 => break,
      _ => println!("Let's continue."),
    }
    // 
      
  }
    
    
}
