#![allow(warnings)]
use colored::*;
use std::env;
use std::fs;
use std::fs::{remove_file, File, OpenOptions};
use std::io::{self, stdout, BufReader, Read, Write};
use std::path::Path;
mod about;
mod write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        loop {
            let mut str = String::new();
            io::stdin().read_line(&mut str).unwrap();
            print!("{}", str);
            io::stdout().flush().unwrap();
        }
    }
    // helpオプションです
    if args[1] == "--help" || args[1] == "-h" {
        about::help();
        std::process::exit(0);
    // varsionを表示します
    } else if args[1] == "--version" || args[1] == "-v" {
        println!(
            "{} {}",
            "visual read/write command".cyan(),
            "vira".cyan().bold()
        );
        println!("alpha release v0.1.2");
    }
    // --writeオプションです
    if args.len() > 3 {
        if args.len() > 4 {
            if args[2] == "--write" || args[2] == "-w" {
                // --writeオプションの-rオプションです
                if args[3] == "-r" && args.len() == 5 {
                    let path = Path::new(args[1].as_str());
                    if path.exists() {
                        write::exists(path, args.clone())
                    } else {
                        write::filenotfound(path, args.clone());
                    }
                } else if args[3] == "-c" && args.len() == 5 {
                    write::wrcontinue(args);
                }
            } else {
                let path = &args[1];
                let mut file = File::create(path).expect("Error: failed to create file.");
                let contents = &args[3];
                file.write_all(contents.as_bytes())
                    .expect("Error: failed to write file");
                println!("Successfully wrote to the file.");
            }
        } else {
            eprintln!("Usage: vira <filename> --write <option> <text>");
            std::process::exit(1);
        }
    } else if args.len() == 3 && args[2] == "-s" || args.len() == 3 && args[2] == "--stdin" {
        let path = &args[1];
        match OpenOptions::new().read(true).append(true).open(path) {
            Ok(mut file) => {
                let mut str = String::new();
                // ファイルの中身をstrに入れる
                file.read_to_string(&mut str).unwrap();
                println!("~~file~~\n{}\n~~EOF~~", str); //ファイルの中身を出力
                                                        // ループ処理
                loop {
                    // 標準入力のためのString
                    str = String::new();
                    // strに入力内容記入
                    io::stdin().read_line(&mut str).unwrap();
                    // \nをなくす
                    str = str.trim().to_string();
                    // match処置でエラー処理
                    match file.write_all(str.as_bytes()) {
                        Ok(_) => {
                            // 成功すればかけた旨のメッセージを出力する
                            println!("Successfully wrote to the file, content={}", str);
                            // 何このflush
                            stdout().flush().unwrap();
                        }
                        Err(e) => {
                            // エラー内容出力。出たらissueで報告してね
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }
            // そもそも開けられなかったら出る
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    } else if args.len() == 3 && args[2] == "-r" || args.len() == 3 && args[2] == "--remove" {
        // ファイル消す
        let _ = match remove_file(args[1].to_string()) {
            Ok(_) => {
                //消せたら出力する
                println!("Successflly to remove file. file:{}", args[1]);
            }
            Err(e) => {
                // できなかったら出る。もし出たら報告して
                eprintln!("failed to remove file: {}", e);
            }
        };
    } else if args.len() == 3 && args[2] == "-n" || args.len() == 3 && args[2] == "--new" {
        // 新しくファイルを作る
        // touchでいいじゃんって言わないで
        match File::create(args[1].as_str()) {
            Ok(_) => {
                // 成功
                println!("Successflly to create file. file:{}", args[1])
            }
            Err(e) => {
                //失敗
                eprintln!("failed to remove file: {}", e);
                std::process::exit(1);
            }
        }
    } else if args.len() == 3 && args[2] == "-sz" || args.len() == 3 && args[2] == "--size" {
        // ファイルのサイズを出力します
        // metadataで大きさ取得
        match fs::metadata(args[1].to_string()) {
            Ok(metadeta) => {
                // file_sizeに大きさを保存。.len()がファイルの大きさ
                let file_size = metadeta.len();
                // 出力。もしMBとかそういうのにしたかったら後々計算を追加する
                println!("File size: {} bytes", file_size);
            }
            Err(e) => {
                // error出力。出たら教えて
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    // backupを作ります
    } else if args.len() == 3 && args[2] == "-b" || args.len() == 3 && args[2] == "--backup" {
        match File::open(args[1].clone()) {
            Ok(mut f) => {
                let mut content = String::new();
                match f.read_to_string(&mut content) {
                    Ok(_) => {
                        let npath = format!("{}.backup", args[1].clone());
                        match File::create(npath) {
                            Ok(mut fi) => {
                                let _ = fi.write_all(content.as_bytes());
                                println!("Backup created successfully");
                            }
                            Err(rt) => eprintln!("Error:{}", rt),
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
    //普通のviraの処理。ほぼcatと同じ動きする。違いといえば最後に改行するぐらい
    else {
        let path = &args[1];
        if !Path::new(path).exists() {
            eprintln!("File not found.");
            std::process::exit(1);
        }
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {}: {}", path, e);
                std::process::exit(1);
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            eprintln!("Error reading file {}: {}", path, e);
            std::process::exit(1);
        }
        println!("{}", contents);
    }
}
