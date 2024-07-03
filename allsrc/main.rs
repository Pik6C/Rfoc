use std::fs::{remove_file, File, OpenOptions};
use std::io::{self, stdout, Read, Write};
use std::env;
use std::path::Path;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        loop{
            let mut str = String::new();
            io::stdin().read_line(&mut str).unwrap();
            print!("{}",str);
            io::stdout().flush().unwrap();
        }
    }
    if args[1] == "--help" || args[1] == "-h"{
        println!("visual file read system vira");
        println!("Usage: vira <option> <file>\n");
        println!("options:");
        println!("");
        println!("  -w or --write:");
        println!("       Usage: vira <file> --write <option> <text>");
        println!("       writes the following string to a file");
        println!("       By setting options, you can choose whether to rewrite or continue writing\n(default is to continue writing)\n");
        println!("       writeoptions:");
        println!("       -r:");
        println!("              Rewrites the contents of the file (existing contents well be lost)\n");
        println!("              If the fike does not exist, create a new file");
        println!("      -c:");
        println!("          Continue writing the contents of the file as is.\n");
        println!("      ~~~end of write options~~~\n")
        println!("  -s or --stdin:");
        println!("          Writes the contents received from standard input into the file one by one. Cannot be deleted.");
        println!("  -r or --remove:");
        println!("      remove to this file.\n      Usage: vira <remove file> -r");
        println!("  -n or --new:");
        println!("      create new file.\n      Usage: vira <file> -n");
        std::process::exit(0);
    }
    if args.len() > 3{
        if args.len() > 4{
        if args[2] == "--write" || args[2] == "-w"{
            if args[3] == "-r" && args.len() == 5{
                let path = Path::new(args[1].as_str());
		if path.exists() {
            let mut file = File::create(path).expect("Error: failed to create file.");
            let contents = &args[4];
            file.write_all(contents.as_bytes()).expect("Error: failed to write file");
            println!("Successfully wrote to the file.");
		
		} else {

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
            }else if args[3] == "-c" && args.len() == 5{
                let path = &args[1];
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(path)
                    .expect("msg");
                let contents = &args[4];
                file.write_all(contents.as_bytes()).expect("");
                println!("Successfully appended to the file.");
            }
            }else{
                let path = &args[1];
                let mut file = File::create(path).expect("Error: failed to create file.");
                let contents = &args[3];
                file.write_all(contents.as_bytes()).expect("Error: failed to write file");
                println!("Successfully wrote to the file.");
            }
        
        }else{
            eprintln!("Usage: vira <filename> --write <option> <text>");
            std::process::exit(1);

        }
    }else if args.len() == 3 && args[2] == "-s" || args.len() == 3 && args[2] == "--stdin"{
        let path = &args[1];
            match OpenOptions::new()
                .append(true)
                .open(path){
                    Ok(mut file) =>{
                        let mut str=String::new();
                        file.read_to_string(&mut str).expect("Error");                       
                        println!("~file~\n{}\n~~EOF~~",str);
                        loop{
                        str = String::new();
                        io::stdin().read_line(&mut str).unwrap();
                        match file.write_all(str.as_bytes()){
                                Ok(_) => {
                                    print!("Successflly wrote to the file, content={}",str);
                                    stdout().flush().unwrap();
                                },
                            Err(e) =>{
                                eprintln!("Error: {}",e)
                            }
                        }
                    }
                    },
                    Err(e) => {
                        eprintln!("Error: {}",e);
                    }
                }
            
    }else if args.len() == 3 && args[2] == "-r" || args.len() == 3 && args[2] == "--remove"{
        let _ = match remove_file(args[1].to_string()){
            Ok(_) =>{
                println!("Successflly to remove file. file:{}",args[1]);
            },
            Err(e) => {
                eprintln!("failed to remove file: {}",e);
            }
        };
    }else if args.len() == 3 && args[2] == "-n" || args.len() == 3 && args[2] == "--new"{
        match File::create(args[1].as_str()){
            Ok(_) => {
                println!("Successflly to create file. file:{}",args[1])
            },
            Err(e) =>{
                eprintln!("failed to remove file: {}",e);
                std::process::exit(1);
            }
        }
    }
    else{
        let path = &args[1];
        if !Path::new(path).exists(){
            eprintln!("File not found.");
            std::process::exit(1);
        }
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {}: {}",path,e);
                std::process::exit(1);
            }
        
        };
        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents){
            eprintln!("Error reading file {}: {}",path,e);
            std::process::exit(1);
        }
        println!("{}",contents);
}
}
