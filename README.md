# RGX

## Project Description
CLI written in Rust to perform regex searches within files/directories through a TUI file explorer

## Screenshots
![screenshot1](https://github.com/FelipeColona/rgx/blob/main/rgx-screenshot-1.png?raw=true)
![screenshot2](https://github.com/FelipeColona/rgx/blob/main/rgx-screenshot-2.png?raw=true)

## Main Features
- Regex search within files and dirs
- TUI
- File explorer 
- File contents preview
- Matched texts highlight
- Toggle hidden files
- Vim keybindings

## Main Technologies:
- Rust
- TUI
- Regex
- Bash

## How to Set up
- Install Rustup
    - ```curl https://sh.rustup.rs -sSf | sh```
- Install Cargo
    - ```rustup default nightly```
- Clone the repository  
    - ```git clone https://github.com/FelipeColona/rgx```
- Enter the project directory
    - ```cd rgx```
- Run install script
    - ```./install.sh```

## Try It Out!
- Open a terminal
- Expected input
    - ```rgx [regex] [path]```
    - Path is optional, if not provided, the current directory is used
- Example
    - ```rgx '([A-Z])\w+' /home/user```

## Keybindings
- Use <kbd>q</kbd> to **quit rgx**
- Use <kbd>j</kbd> to **go down an entry**
- Use <kbd>k</kbd> to **go up an entry**
- Use <kbd>g</kbd> to **go to the top entry**
- Use <kbd>g</kbd> to **go to the bottom entry**
- Use <kbd>l</kbd> to **enter in a directory**
- Use <kbd>h</kbd> to **return to a previous directory**
- Use <kbd>n</kbd> to **search forwards to the next ocurrence**
- Use <kbd>N</kbd> to **search backwards to the next ocurrence**
- Use <kbd>Ctrl</kbd>+<kbd>h</kbd> to **toggle hidden files visibility**