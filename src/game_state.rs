#[derive(Debug)]
pub enum GameResult {
    Running,
    Won,
    Lost,
}

// represents the state of a hangman game - rendering is done client-side
pub struct GameState {
    pub word_progress: Vec<char>,
    pub game_result: GameResult,
    pub incorrect_guesses: u8,
    pub letters_guessed: Vec<char>,
}

impl GameState {
    pub fn deserialize(packet: [u8; 40]) -> Self {
        // word progress is the first 14 bytes of the packet
        let word_progress: Vec<char> = packet[..14]
            .iter()
            // The closure |&byte| byte as char takes each reference to a byte (&byte) from the iterator and converts it into a char by using the as keyword (ChatGPT)
            .map(|&byte| byte as char)
            .collect();

        // match bytes 14 & 15 against protocol-doc
        let game_result = match packet[14..16] {
            [0, 0] => GameResult::Lost,
            [0, 1] => GameResult::Won,
            [1, _] => GameResult::Running,
            _      => GameResult::Running, // not ideal but this should never happen... right?
        };

        // byte 16 of gamestate packet is the number of incorrect guesses
        let incorrect_guesses = packet[16];

        let letters_guessed: Vec<char> = packet[17..]
        .iter()
        // The closure |&byte| byte as char takes each reference to a byte (&byte) from the iterator and converts it into a char by using the as keyword (ChatGPT)
        .map(|&byte| byte as char)
        .collect();

        // create a new GameState instance
        GameState {
            word_progress,
            game_result,
            incorrect_guesses,
            letters_guessed,
        }
    }
}