# Vira
A simple file read/write command
<br>
Basic usage:
`vira <file> <option>`
If you run vira without specifying anything, it will take input from standard input, just like cat.
<br>

Option types:
`-s`:
Takes text from standard input and writes it to a file.
`-r`:
Deletes the file.

<br>
`--write` or `-w`:
Writes the following text to the file. There are several options for this:
<br>
==write options==
`-r`:Rewrites the contents of the file (existing contents will be lost). If the file does not exist, it will create a new file.
