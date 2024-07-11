use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Write};
use std::io;

pub fn exists(path:&Path, args:Vec<String>){
    let mut file = File::create(path).expect("Error: failed to create file.");
    let contents = &args[4];
    file.write_all(contents.as_bytes()).expect("Error: failed to write file");
    println!("Successfully wrote to the file.");
}
pub fn filenotfound(path: &Path, args:Vec<String>){
    print!("File not found. Do you want to make a new file?(y or n): ");
    io::stdout().flush().unwrap();
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let trimstr = str.trim();
    if trimstr == "y" || trimstr == "yes"{

        let mut file = File::create(path).expect("Error: failed to create file.");
        let contents = &args[4];
        file.write_all(contents.as_bytes()).expect("Error: failed to write file");
        println!("Successfully wrote to the file.");
    }
}

pub fn wrcontinue(args:Vec<String>){
    let path = &args[1];
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Failed: ");
    let contents = &args[4];
    file.write_all(contents.as_bytes()).expect("Error");
    println!("Successfully appended to the file.");
}