mod game_state;
use game_state::GameState;

use std::io::prelude::*;
use std::net::TcpStream;

const IP_ADDRESS: &str = "127.0.0.1";

fn main() {
    // let mut stream = TcpStream::connect("127.0.0.1:25565")?;

    let packet = request_game_update();
    println!("{:?}", &packet);
    process_packet(packet);

    // let char_guess = b'a';
    // guess_letter(&mut stream, char_guess);

    // guess_word(&mut stream, "asdf".to_string());

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

// TODO: refactor to remove stream inputs
//                                  u8 because we only support ASCII chars
fn guess_letter(stream: &mut TcpStream, guess: u8) {
    let mut out = [0; 16];
    out[1] = guess;
    let _ = stream.write(&out);
}

fn guess_word(stream: &mut TcpStream, guess: String) {
    let guess_bytes = &guess.as_bytes();
    let mut out = [0; 16];
    // iterate over all bytes in string, replacing bytes 2..n of output packet
    for (i, b) in guess_bytes.iter().take(15).enumerate() {
        out[i+1] = *b;
    }
    let _ = stream.write(&out);
}

fn clear_output() { // this only clears the text currently on screen, not any history 
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

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
    rows.push(String::from("> "));

    for row in rows {
        println!("{}", row);
    }
    
}

fn stringify_game_state_chars(word_progress: &Vec<char>) -> String {
    let mut string = String::new();

    for char in word_progress {
        string.push(*char);
        string.push(' ');
    }

    string
}

fn process_packet(packet: [u8; 40]) {
    // new GameState object from our packet
    let gs = GameState::deserialize(packet);

    // clear terminal
    clear_output();
    
    // render new output
    render_game_state(gs);

}