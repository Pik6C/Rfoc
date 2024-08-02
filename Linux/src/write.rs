use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;

pub fn exists(path: &Path, args: Vec<String>) {
    match File::create(path) {
        Ok(mut file) => {
            let contents = &args[4];
            match file.write_all(contents.as_bytes()) {
                Ok(_) => {
                    println!("Successfully wrote to the file.");
                    return;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        // File::createのErr処理
        Err(er) => {
            eprintln!("Error: {}", er);
            std::process::exit(1);
        }
    }
}
pub fn filenotfound(path: &Path, args: Vec<String>) {
    print!("File not found. Do you want to make a new file?(y or n): ");
    io::stdout().flush().unwrap(); // 流石にmatchいらない
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap(); // ここは流石にmatchいらないと思う
    let trimstr = str.trim();
    if trimstr == "y" || trimstr == "yes" || trimstr == "" {
        match File::create(path) {
            Ok(mut file) => {
                let contents = &args[4];
                match file.write_all(contents.as_bytes()) {
                    Ok(_) => {
                        println!("Successfully wrote to the file.");
                        return;
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            // File::createのErr処理
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

pub fn wrcontinue(args: Vec<String>) {
    let path = &args[1];
    match OpenOptions::new().append(true).open(path) {
        Ok(mut file) => {
            let contents = &args[4];
            match file.write_all(contents.as_bytes()) {
                Ok(_) => {
                    println!("Successfully append to the file.");
                    return;
                }
                // file.write_allのErrです
                Err(e) => {
                    eprint!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        // OpenOptions::new()のErrです
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
