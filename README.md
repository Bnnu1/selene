# Rust TUI File Manager
I wanted to make a TUI file manager that was fast and could run shell commands...so I did  
To install (You will need Rust installed):  
```
git clone https://github.com/Bnnu1/selene
cd selene
cargo build --release
sudo cp target/release/selene /usr/local/bin/
```  
To run:  
```
selene
```
':' to input commands, it's simply a shell instance so any commands on your machine work
%m to run it on all marked items
%s to run it on the highlighted item
%d in place of the current shown directory
space to mark item
escape to quit
