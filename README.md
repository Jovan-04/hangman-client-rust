## Hangman Client
This is my first project in Rust, the client for a networked implementation of the popular whiteboard game Hangman. I wrote it at the end of my sophomore year of college as an add-on to the final project for my Programming Languages class.  
**Note: This project requires my [hangman-server-rust](https://github.com/Jovan-04/hangman-server-rust) repository to work!** There is more information about setting up the server in that repository.  

### Setup
*tested on linux, but it should mostly work for other OSes too*  
* Before setting up the client, ensure that you have either installed the hangman server or have the IP address of a server to connect to  
1. Open a terminal and navigate to an appropriate directory (such as your Downloads)  
2. Clone this repo with `git clone https://github.com/Jovan-04/hangman-client-rust.git`  
3. Run `cd hangman-server-rust` to navigate to the project's root directory  
4. By default, the client connects to `127.0.0.1:25565`; you can change that by editing the `IP_ADDRESS` string in `src/main.rs:10`
* You can also build this to a binary executable for your OS using `cargo build`. The output file will be `./target/debug/server`.

### Usage
1. Run the project's source code by navigating to its root directory and running `cargo run` in the terminal  
2. If you instead have a binary executable (likely named `client`), you can run it directly with `./client` from the file's directory  
3. Upon running the project, you'll get ASCII art of the game board, which will update as you make guesses  
4. You can submit a guess for a single letter or the entire word; you have up to 6 incorrect guesses before you lose  
5. Have fun!
