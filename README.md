# ptcliputil

Pronounced '_Pee-Tee_ Clip Util', ptcliputil is a CLI tool to copy text in the
terminal, whether it is passed from another command via pipes (`|`) or read
from files with a compatible encoding.

## Usage

```bash
$ ptcliputil
CLI tool to copy text in the terminal, either passed from pipes or read from files

Usage: ptcliputil [OPTIONS] [FILE]

Arguments:
  [FILE]  The file to read (optional if piping data)

Options:
  -v, --verbose          Show the copied text (forced false if multi-line)
  -c, --clear <SECONDS>  Clear the clipboard after N seconds
  -h, --help             Print help
  -V, --version          Print version
```

<!--Fun fact: During the making of this README file many of the terminal
outputs shown were copied using the same `ptcliputil` tool.

E.g: `ptcliputil | ptcliputil` to copy the usage output.-->

### Reading text from files

```bash
$ ptcliputil my-file.txt
✓ Content copied to clipboard (1 lines, 5 characters).
```

### Reading text passed from commands

```bash
$ cat my-file.txt | ptcliputil # Alternative to `ptcliputil <file>`
✓ Content copied to clipboard (1 lines, 5 characters).

$ apt list | ptcliputil # Extremely long text
✓ Content copied to clipboard (80326 lines, 4205087 characters).

$ which waybar | ptcliputil # Will copy the path to waybar
✓ Content copied to clipboard (1 lines, 15 characters).
```

### Using verbose mode

Verbose mode is a new feature (added in v0.2.0) that makes the program show in
the terminal whatever text was copied to the clipboard. To avoid flooding,
though, it only works if the copied text is 1 line long. Using the tool with
very long texts won't print the copied text (even if the --verbose flag was
used) to avoid cluttering the terminal.

```bash
$ ptcliputil my-file.txt --verbose
✓ Copied: "hello"

$ ptcliputil long-text.txt --verbose
✓ Content copied to clipboard (20 lines, 653 characters).

$ which firefox | ptcliputil --verbose
✓ Copied: "/usr/bin/firefox"
```

### Clearing the clipboard

In v0.2.0 I also added a flag to automatically clear the clipboard after a
specified amount of time, measured in seconds, both for privacy and memory
reasons.

```bash
$ cat my_super_secret_password.txt | ptcliputil -c 60
✓ Content copied to clipboard (1 lines, 64 characters).
INFO: The `--clear` flag was detected. Clipboard will be cleared in 60 seconds.
# ...60 seconds later:
Clipboard cleared.
```

_This is a reference to a Python script I made a while ago to generate random
passwords with a few unrelated factors, incluiding the current weather in
Berlin, Germany, the current date and time, the IP address of the user and a
custom passphrase._

### Clipboard dumping

In v0.3.0 I added a functionality to save the current clipboard contents to a
file using the `--save <FILE>` option.

## Building from source code

1. Clone the repo with git (`git clone https://github.com/SaturniNovaDev/ptcliputil`)
2. Run `cargo build --release` to compile to a usable binary
3. **Optional:** Copy the executable to `/usr/local/bin/` to use it system-wide

## Regarding Windows compatibility

I do not currently have plans to port the tool to Windows.

## Known issues

Keep in mind the tool is mainly intended for use with simple programs that
return straightforward strings and ask for no user input, like curl, cat, apt
among others. The tool is compatible with scripts that ask for user input but
the user will not be able to see the prompt in most cases (with Python inputs,
for example).

The `--clear` logic uses `thread::sleep` on the main thread, which means the
process remains alive and persists in the terminal until the timer expires.
