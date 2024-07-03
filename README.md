# Visual File Read System - Vira

## Usage

`vira <file> <option>`

## Options
<br>

### -w or --write
Usage: `vira <file> --write <option> <text>`
<br>
Writes the specified string to a file. By setting options, you can choose whether to overwrite or append to the file.
<br>

#### Write Options
- **-r:**  
  Overwrites the contents of the file (existing contents will be lost). If the file does not exist, a new file will be created.
- **-c:**  
  Appends the contents to the file as is.
<br>

**End of write options.**
<br>

### -s or --stdin
  Usage: `vira <file> -s`
Writes the contents received from standard input into the file sequentially. The content cannot be deleted.
<br>

### -r or --remove
Usage: `vira <file> -r`
Removes the specified file.
<br>

### -n or --new
Usage: `vira <file> -n`
Creates a new file.
<br>
### -sz or --size
Usage: vira <file> -sz
Displays the size of the specified file.
