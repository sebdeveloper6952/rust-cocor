
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
pub struct Token {
    pub name: String,
    pub lexeme: String,
}

impl Token {
    pub fn new(name: String, lexeme: String) -> Token {
        Token { name, lexeme }
    }

    pub fn empty() -> Token {
        Token {name:String::new(), lexeme:String::new()}
    }
}

pub struct Scanner {
    pub tokens: Vec<Token>,
    index: u32,
}

impl Scanner {
    pub fn new(path: &str) -> Scanner {
        let mut scanner = Scanner {
            tokens: Vec::new(),
            index: 0,
        };
        scanner.init(path);

        scanner
    }
    pub fn next_token(&mut self) -> Option<Token> {
        match self.tokens.get(self.index as usize) {
            Some(token) => {
                self.index += 1;
                Some(token.clone())
            }
            None => None,
        }
    }

    fn init(&mut self, path: &str) {
        let file = fs::read_to_string(path).unwrap();
        let mut dfa: HashMap<u32, HashMap<u8, u32>> = HashMap::new();
        let mut accepting_states: HashMap<u32, String> = HashMap::new();
    dfa.insert(0, HashMap::new());dfa.get_mut(&0).unwrap().insert(107, 1);dfa.get_mut(&0).unwrap().insert(103, 1);dfa.get_mut(&0).unwrap().insert(114, 1);dfa.get_mut(&0).unwrap().insert(97, 1);dfa.get_mut(&0).unwrap().insert(71, 1);dfa.get_mut(&0).unwrap().insert(87, 1);dfa.get_mut(&0).unwrap().insert(69, 1);dfa.get_mut(&0).unwrap().insert(90, 1);dfa.get_mut(&0).unwrap().insert(119, 1);dfa.get_mut(&0).unwrap().insert(79, 1);dfa.get_mut(&0).unwrap().insert(99, 1);dfa.get_mut(&0).unwrap().insert(78, 1);dfa.get_mut(&0).unwrap().insert(109, 1);dfa.get_mut(&0).unwrap().insert(72, 1);dfa.get_mut(&0).unwrap().insert(89, 1);dfa.get_mut(&0).unwrap().insert(86, 1);dfa.get_mut(&0).unwrap().insert(68, 1);dfa.get_mut(&0).unwrap().insert(66, 1);dfa.get_mut(&0).unwrap().insert(100, 1);dfa.get_mut(&0).unwrap().insert(113, 1);dfa.get_mut(&0).unwrap().insert(105, 1);dfa.get_mut(&0).unwrap().insert(111, 1);dfa.get_mut(&0).unwrap().insert(112, 1);dfa.get_mut(&0).unwrap().insert(75, 1);dfa.get_mut(&0).unwrap().insert(118, 1);dfa.get_mut(&0).unwrap().insert(82, 1);dfa.get_mut(&0).unwrap().insert(77, 1);dfa.get_mut(&0).unwrap().insert(120, 1);dfa.get_mut(&0).unwrap().insert(83, 1);dfa.get_mut(&0).unwrap().insert(73, 1);dfa.get_mut(&0).unwrap().insert(102, 1);dfa.get_mut(&0).unwrap().insert(44, 2);dfa.get_mut(&0).unwrap().insert(98, 1);dfa.get_mut(&0).unwrap().insert(88, 1);dfa.get_mut(&0).unwrap().insert(106, 1);dfa.get_mut(&0).unwrap().insert(84, 1);dfa.get_mut(&0).unwrap().insert(110, 1);dfa.get_mut(&0).unwrap().insert(116, 1);dfa.get_mut(&0).unwrap().insert(104, 1);dfa.get_mut(&0).unwrap().insert(70, 1);dfa.get_mut(&0).unwrap().insert(101, 1);dfa.get_mut(&0).unwrap().insert(108, 1);dfa.get_mut(&0).unwrap().insert(115, 1);dfa.get_mut(&0).unwrap().insert(67, 1);dfa.get_mut(&0).unwrap().insert(76, 1);dfa.get_mut(&0).unwrap().insert(74, 1);dfa.get_mut(&0).unwrap().insert(85, 1);dfa.get_mut(&0).unwrap().insert(122, 1);dfa.get_mut(&0).unwrap().insert(121, 1);dfa.get_mut(&0).unwrap().insert(81, 1);dfa.get_mut(&0).unwrap().insert(65, 1);dfa.get_mut(&0).unwrap().insert(80, 1);dfa.get_mut(&0).unwrap().insert(117, 1);dfa.insert(2, HashMap::new());dfa.insert(1, HashMap::new());dfa.get_mut(&1).unwrap().insert(57, 1);dfa.get_mut(&1).unwrap().insert(116, 1);dfa.get_mut(&1).unwrap().insert(66, 1);dfa.get_mut(&1).unwrap().insert(73, 1);dfa.get_mut(&1).unwrap().insert(119, 1);dfa.get_mut(&1).unwrap().insert(90, 1);dfa.get_mut(&1).unwrap().insert(110, 1);dfa.get_mut(&1).unwrap().insert(71, 1);dfa.get_mut(&1).unwrap().insert(122, 1);dfa.get_mut(&1).unwrap().insert(103, 1);dfa.get_mut(&1).unwrap().insert(108, 1);dfa.get_mut(&1).unwrap().insert(99, 1);dfa.get_mut(&1).unwrap().insert(49, 1);dfa.get_mut(&1).unwrap().insert(104, 1);dfa.get_mut(&1).unwrap().insert(100, 1);dfa.get_mut(&1).unwrap().insert(111, 1);dfa.get_mut(&1).unwrap().insert(53, 1);dfa.get_mut(&1).unwrap().insert(113, 1);dfa.get_mut(&1).unwrap().insert(67, 1);dfa.get_mut(&1).unwrap().insert(107, 1);dfa.get_mut(&1).unwrap().insert(76, 1);dfa.get_mut(&1).unwrap().insert(65, 1);dfa.get_mut(&1).unwrap().insert(68, 1);dfa.get_mut(&1).unwrap().insert(112, 1);dfa.get_mut(&1).unwrap().insert(115, 1);dfa.get_mut(&1).unwrap().insert(81, 1);dfa.get_mut(&1).unwrap().insert(75, 1);dfa.get_mut(&1).unwrap().insert(80, 1);dfa.get_mut(&1).unwrap().insert(50, 1);dfa.get_mut(&1).unwrap().insert(121, 1);dfa.get_mut(&1).unwrap().insert(114, 1);dfa.get_mut(&1).unwrap().insert(101, 1);dfa.get_mut(&1).unwrap().insert(70, 1);dfa.get_mut(&1).unwrap().insert(74, 1);dfa.get_mut(&1).unwrap().insert(89, 1);dfa.get_mut(&1).unwrap().insert(51, 1);dfa.get_mut(&1).unwrap().insert(85, 1);dfa.get_mut(&1).unwrap().insert(102, 1);dfa.get_mut(&1).unwrap().insert(118, 1);dfa.get_mut(&1).unwrap().insert(98, 1);dfa.get_mut(&1).unwrap().insert(55, 1);dfa.get_mut(&1).unwrap().insert(97, 1);dfa.get_mut(&1).unwrap().insert(82, 1);dfa.get_mut(&1).unwrap().insert(106, 1);dfa.get_mut(&1).unwrap().insert(105, 1);dfa.get_mut(&1).unwrap().insert(86, 1);dfa.get_mut(&1).unwrap().insert(88, 1);dfa.get_mut(&1).unwrap().insert(117, 1);dfa.get_mut(&1).unwrap().insert(77, 1);dfa.get_mut(&1).unwrap().insert(83, 1);dfa.get_mut(&1).unwrap().insert(52, 1);dfa.get_mut(&1).unwrap().insert(78, 1);dfa.get_mut(&1).unwrap().insert(120, 1);dfa.get_mut(&1).unwrap().insert(48, 1);dfa.get_mut(&1).unwrap().insert(56, 1);dfa.get_mut(&1).unwrap().insert(87, 1);dfa.get_mut(&1).unwrap().insert(109, 1);dfa.get_mut(&1).unwrap().insert(54, 1);dfa.get_mut(&1).unwrap().insert(69, 1);dfa.get_mut(&1).unwrap().insert(79, 1);dfa.get_mut(&1).unwrap().insert(84, 1);dfa.get_mut(&1).unwrap().insert(72, 1);accepting_states.insert(1, String::from("ident"));accepting_states.insert(2, String::from("__,__"));
    let bytes: &[u8] = file.as_bytes();
    let mut curr_idx = 0;
    let mut curr_state = 0;
    let mut states: Vec<u32> = Vec::new();
    let mut curr_lexeme = String::new();
    while curr_idx < bytes.len() {
        let curr_char = bytes[curr_idx];
    if dfa[&curr_state].contains_key(&curr_char) {
        let next_state = dfa[&curr_state][&curr_char];
        curr_state = next_state;
        states.push(curr_state);
        curr_lexeme.push(curr_char as char);
        curr_idx += 1;
    }
    else if !states.is_empty() {
        while !states.is_empty() {
            let top = states.pop().unwrap();
            if accepting_states.contains_key(&top) {
    
        self.tokens.push(Token::new(
            accepting_states[&top].clone(),
            curr_lexeme.clone(),
        ));
        
    curr_state = 0;
                    states.clear();
                    curr_lexeme.clear();
                } else {
                    if states.is_empty() {
                        curr_state = 0;
                        curr_lexeme.clear();
                    } else {
                        curr_lexeme.pop();
                        curr_idx -= 1;
                    }
                }
            }
        } else {
            curr_state = 0;
            curr_idx += 1;
        }
    }
}}
    