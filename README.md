# Growtopia.exe patcher
This program patches the game executable, replacing the hardcoded hostnames with the one provided by you \
Default values (if you run the program without arguments): \
game path: growtopia.exe \
output file: patched.exe \
replaced ip: 127.0.0.1 \

## Usage
```commandline
PS .\Growtopia_patcher.exe --help
growtopia.exe patcher 1.0
This program replace server string in growtopia.exe with ip address given

USAGE:
    Growtopia_patcher.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>        growtopia.exe file path (default: growtopia.exe)
    -i, --ip <ip>            IP address/hostname to replace with (default: 127.0.0.1)
    -o, --output <output>    Output file path (default: patched.exe)
```