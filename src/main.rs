mod game_state;
use game_state::GameState;

use std::io::prelude::*;
use std::io;
use std::net::TcpStream;

const IP_ADDRESS: &str = "127.0.0.1:25565";

fn main() {
    process_packet(request_game_update());
    // main game loop
    loop {
        let mut guess = String::new();

        // prompt user for input
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        println!("{}", guess);

        // trim whitespace and convert to uppercase
        guess = guess.trim().to_string().to_ascii_uppercase();

        // ensure guesses are ASCII
        if !guess.chars().all(|c| c.is_ascii()) {
            println!("Guess must contain only ASCII characters");
            continue; // restart loop to prompt for input again
        }

        // single character, submit a letter guess
        if guess.len() == 1 {
            guess_letter(guess.as_bytes()[0]);
        } else { // otherwise, guess the whole word
            guess_word(guess);
        }
        // request a game update after the guess is submitted
        process_packet(request_game_update());
    }
}


// send a request_game_update packet and return the byte-array response
fn request_game_update() -> [u8; 40] {
    let stream = TcpStream::connect(&IP_ADDRESS);
    let mut stream = match stream {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    // per protocol-doc, a request_game_update packet has first byte as 0
    let _ = stream.write(&[0; 16]);

    let mut buffer = [0; 40];
    let _ = stream.read(&mut buffer);

    buffer
}

//              u8 because we only support ASCII chars
fn guess_letter(guess: u8) {
    let stream = TcpStream::connect(&IP_ADDRESS);
    let mut stream = match stream {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let mut out = [0; 16];
    out[0] = 1; // byte 0: 1 (protocol-doc)
    out[2] = guess; // byte 2: ASCII letter guess
    let _ = stream.write(&out);
}

fn guess_word(guess: String) {
    let stream = TcpStream::connect(&IP_ADDRESS);
    let mut stream = match stream {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let guess_bytes = &guess.as_bytes();
    let mut out = [0; 16];
    out[0] = 1;
    out[1] = 1;
    // iterate over all bytes in string, replacing bytes 3..n of output packet
    for (i, b) in guess_bytes.iter().take(15).enumerate() {
        out[i+2] = *b;
    }
    let _ = stream.write(&out);
}

fn clear_output() { // this only clears the text currently on screen, not any history 
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// prints the game state out to the terminal
fn render_game_state(gs: GameState) {
    let mut rows: Vec<String> = Vec::new();

    let inc = gs.incorrect_guesses;
    rows.push(String::from("HANGMAN by Evan Scherrer & Blythe Kelly"));
    rows.push(format!     ("Game Result: {:?} ", gs.game_result));
    rows.push(format!     ("Incorrect Guesses: {} / 6", gs.incorrect_guesses));
    rows.push(String::from(" "));
    rows.push(String::from(" |—————|"));
    rows.push(String::from(" |     |"));
    rows.push(format!     (" |     {}", if inc >= 1 {"_"} else {" "}));
    rows.push(format!     (" |    {}", if inc >= 1 {"(_)"} else {"   "})); // this is so damn ugly but I don't have any other ideas
    rows.push(format!     (" |   {}", if [2, 3, 4].contains(&inc) {"  |  "} else if inc == 5 {"——|  "} else if inc == 6 {"——|——"} else {"     "}));
    rows.push(format!     (" |     {}", if inc >= 2 {"|"} else {" "} ));
    rows.push(format!     (" |    {}", if inc == 3 {"/  "} else if inc >= 4 {"/ \\"} else {"   "}));
    rows.push(format!     (" |   {}", if inc == 3 {"/    "} else if inc >= 4 {"/   \\"} else {"     "}));
    rows.push(String::from(" |"));
    rows.push(format!     ("_|_____________"));
    rows.push(String::from(" "));
    rows.push(format!     ("Word: {}", stringify_game_state_chars(&gs.word_progress)));
    rows.push(String::from(" "));
    rows.push(format!     ("Letters Guessed: {}", stringify_game_state_chars(&gs.letters_guessed)));
    rows.push(format!     ("______________________________"));
    rows.push(String::from(" "));

    for row in rows {
        println!("{}", row);
    }
    
}

// helper function for render_game_state()
fn stringify_game_state_chars(word_progress: &Vec<char>) -> String {
    let mut string = String::new();

    for char in word_progress {
        string.push(*char);
        string.push(' ');
    }

    string
}

// this doesn't return a GameState object...should we make it that way?
fn process_packet(packet: [u8; 40]) {
    // new GameState object from our packet
    let gs = GameState::deserialize(packet);

    // clear terminal
    clear_output();
    
    // render new output
    render_game_state(gs);

}