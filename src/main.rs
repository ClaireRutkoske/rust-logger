// Hide warnings for unused code.
#![allow(dead_code)]

extern crate time;

use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::string::String;


// Log structure
struct Single {
  timestamp: i64,
  entry: String,
  tags: String,
}

// Timestamp
fn timestamp() -> i64 {
  let timespec = time::get_time();
  let mills: i64 = (timespec.sec as i64 * 1000)+ (timespec.nsec as i64 / 1000 / 1000);    
  mills
}

// Read line and store
fn reader() -> String {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("Failed to read.");
  s
}


fn main() {
    
  // Create a directory, returns `io::Result<()>`
  match fs::create_dir("log") {
    Err(why) => println!("! {:?}", why.kind()),
    Ok(_) => println!("Log dir/ created."),
  }
    
  // Create a file, returns `io::Reults<()>`
  match OpenOptions::new().create(true).write(true).open("log/log") {
    Err(why) => println!("! {:?}", why.kind()), 
    Ok(_) => println!("Log is open."),
  }
  
  
    
  loop {
      
    let mut file = OpenOptions::new().write(true).append(true).open("log/log").unwrap(); 
    
    // The log entry
    println!("Log entry:");
    let entry = reader();
      
    // Tags
    println!("Tag the entry (comma-separated):");
    let tags = reader();
    
    // Create Entry struct
    let single = Single  {
      timestamp: timestamp(),
      entry: entry.trim().to_string(),
      tags: tags.trim().to_string(),
    };
      
    // Write to the log file  
    writeln!(file, "{:?}, {:?}, {:?}", single.timestamp, single.entry, single.tags).unwrap();
      
    println!("Success!");
      
    // Add another
    println!("Type `1` to add another, or `0` to exit.");
    let cont = reader();
      
    let cont: u32 = match cont.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
      
    match cont {
      1 => println!("Excellent! Let's continue."),
      0 => break,
      _ => println!("Let's continue."),
    }
      
  }
    
}
