# ptcliputil

Pronounced '_Pee-Tee_ Clip Util', ptcliputil is a CLI tool to copy text in the terminal, wether it is passed from another command via pipes (`|`) or read from files with a compatible encoding.

## Usage

```bash
$ ptcliputil
Usage:
  list | ptcliputil
  ptcliputil <file>
```

### Reading text from files

```bash
$ ptcliputil my-file.txt
✓ Content copied to clipboard.
```

### Reading text passed from commands

```bash
$ cat my-file.txt | ptcliputil # Alternative to `ptcliputil <file>`
✓ Content copied to clipboard.

$ apt list | ptcliputil # Extremely long text
✓ Content copied to clipboard.

$ which waybar | ptcliputil # Will copy the path to waybar
✓ Content copied to clipboard.
```

I am working on an update to make the program more verbose when it comes to the output because, as you can see, the output is currently the same for everything, and that can be improved.
