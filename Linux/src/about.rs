use colored::*;

pub fn help() {
    println!(
        "{} {}",
        "visual file read system".cyan(),
        "vira".cyan().bold()
    );
    println!("{}", "Usage: vira <file> <option>".green());
    println!("{}", "options:".yellow());
    println!("");
    println!("{} or {}", "-v".purple(), "--version".purple());
    println!("  Display the vira version");
    println!("{} or {}", "--write".purple(), "-w:".purple());
    println!("       Usage: vira <file> --write <option> <text>");
    println!("       writes the following string to a file");
    println!("       By setting options, you can choose whether to rewrite or continue writing\n");
    println!("       {}", "writeoptions:".yellow());
    println!("       {}", "-r:".cyan());
    println!("              Rewrites the contents of the file (existing contents well be lost)\n");
    println!("              If the fike does not exist, create a new file");
    println!("      {}", "-c".cyan());
    println!("          Continue writing the contents of the file as is.\n");
    println!("      {}", "~~~end of Write Oprions~~~".yellow());
    println!("  {} or {}", "-s".purple(), "--serch:".purple());
    println!("      Checks if a file contains a specific string.");
    println!("      If so, print the number and line where that character is found.");
    println!("          Usage: vira <file> -s <serch string>");
    println!("  {} or {}", "-r".purple(), "--remove:".purple());
    println!("      remove to this file.\n      Usage: vira <remove file> -r");
    println!("  {} or {}", "-n".purple(), "--new:".purple());
    println!("      create new file.\n      Usage: vira <file> -n");
    println!("  {} or {}", "-sz".purple(), "--size".purple());
    println!("      Display this file size.     Usage: vira <file> -sz");
    println!("  {} or {}", "-b".purple(), "--backup".purple());
    println!("      Create a backup of that file.   Usage: vira <file> -b");
    println!("  {} or {}", "-cw".purple(), "--continuew".purple());
    println!("      Receives characters from standard input and writes the characters successively down the file.");
    println!("      Usage: vira <file> -cw");
}
