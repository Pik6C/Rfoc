use colored::*;

pub fn help(){
    println!("{} {}","visual file read system".cyan(),"vira".cyan().bold());
    println!("{}","Usage: vira <file> <option>".green());
    println!("{}","options:".yellow());
    println!("");
    println!("{} or {}","--write".purple(),"-w:".purple());
    println!("       Usage: vira <file> --write <option> <text>");
    println!("       writes the following string to a file");
    println!("       By setting options, you can choose whether to rewrite or continue writing\n");
    println!("       {}","writeoptions:".yellow());
    println!("       {}","-r:".cyan());
    println!("              Rewrites the contents of the file (existing contents well be lost)\n");
    println!("              If the fike does not exist, create a new file");
    println!("      {}","-c".cyan());
    println!("          Continue writing the contents of the file as is.\n");
    println!("      {}","~~~end of Write Oprions~~~".yellow());
    println!("  {} or {}","-s".purple(),"--stdin:".purple());
    println!("      Writes the contents received from standard input into the file one by one. Cannot be deleted.");
    println!("  {} or {}","-r".purple(),"--remove:".purple());
    println!("      remove to this file.\n      Usage: vira <remove file> -r");
    println!("  {} or {}","-n".purple(),"--new:".purple());
    println!("      create new file.\n      Usage: vira <file> -n");
    println!("  {} or {}","-sz".purple(),"--size".purple());
    println!("      Display this file size.     Usage: vira <file> -sz");
}