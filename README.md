# File operation commands - Vira

## Usage

`vira <file> <option>`

## Options
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

<br>

### -s or --serch
Usage: `vira <file> -s`
  <br>
Checks if a file contains a specific string.
If so, print the number and line where that character is found.
<br>

### -r or --remove
Usage: `vira <file> -r`
<br>
Removes the specified file.
<br>

### -n or --new
Usage: `vira <file> -n`
<br>
Creates a new file.
<br>
### -sz or --size
Usage: `vira <file> -sz`
<br>
Displays the size of the specified file.

### -b or --backup 
Usage: `vira <file> -b`
Create a backup of your files.

### -cw or --continuew
Usage: `vira <file> -cw`
<br>
Receives characters from standard input and writes the characters successively down the file.

## Applying the vira Command
### For Linux
Only available for Debian-based systems.<br>
Can be used on any OS by installing dpkg<br>
Note that the sudo command is used.<br>
Execute the following command<br>
`sudo dpkg -i vira.deb`
### For Windows

#### For Windows 10
> [!WARNING]
> Please note that Windows version will not be released from v0.1.2


> [!IMPORTANT]
> Unlike the Linux version, the text in the Windows version is not colored.

1. Open Start and click on Settings.
   - The Windows Settings window will appear.
2. Type "environment variables" in the search bar.
   - "Edit the system environment variables" will appear as an option.
3. Click on "Edit the environment variables".
   - The Environment Variables window will appear.
<br>

Under User variables, click on **Path** and then click **Edit**.
The Edit Environment Variable window will appear.<br>
Click on New, enter the path where the vira command is located, and click OK.<br>
Return to the Environment Variables window.<br>
<br>

#### For Windows 11
1. Press Windows key + Pause.
   - Open the System - About screen of the Settings app.
2. Click on the link for System info settings.
   - Open the System Properties window.
3. Click on the Environment Variables button at the bottom right of the Advanced tab.
   - Open the Environment Variables window.

Click New, enter the path where the vira command is located, and click OK.<br>
Return to the Environment Variables window.<br>
