
use std::collections::HashMap;
use std::env;
use std::process;
use std::fs;

#[derive(Debug, Clone)]
pub struct Token {
    name: String,
    lexeme: String,
}

impl Token {
    fn new(name: String, lexeme: String) -> Token {
        Token { name, lexeme }
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
    dfa.insert(2, HashMap::new());dfa.get_mut(&2).unwrap().insert(48, 2);dfa.get_mut(&2).unwrap().insert(51, 2);dfa.get_mut(&2).unwrap().insert(53, 2);dfa.get_mut(&2).unwrap().insert(49, 2);dfa.get_mut(&2).unwrap().insert(46, 3);dfa.get_mut(&2).unwrap().insert(54, 2);dfa.get_mut(&2).unwrap().insert(50, 2);dfa.get_mut(&2).unwrap().insert(52, 2);dfa.get_mut(&2).unwrap().insert(55, 2);dfa.get_mut(&2).unwrap().insert(56, 2);dfa.get_mut(&2).unwrap().insert(57, 2);dfa.insert(3, HashMap::new());dfa.get_mut(&3).unwrap().insert(57, 4);dfa.get_mut(&3).unwrap().insert(52, 4);dfa.get_mut(&3).unwrap().insert(53, 4);dfa.get_mut(&3).unwrap().insert(50, 4);dfa.get_mut(&3).unwrap().insert(48, 4);dfa.get_mut(&3).unwrap().insert(49, 4);dfa.get_mut(&3).unwrap().insert(56, 4);dfa.get_mut(&3).unwrap().insert(51, 4);dfa.get_mut(&3).unwrap().insert(55, 4);dfa.get_mut(&3).unwrap().insert(54, 4);dfa.insert(0, HashMap::new());dfa.get_mut(&0).unwrap().insert(55, 2);dfa.get_mut(&0).unwrap().insert(51, 2);dfa.get_mut(&0).unwrap().insert(13, 1);dfa.get_mut(&0).unwrap().insert(9, 1);dfa.get_mut(&0).unwrap().insert(56, 2);dfa.get_mut(&0).unwrap().insert(49, 2);dfa.get_mut(&0).unwrap().insert(50, 2);dfa.get_mut(&0).unwrap().insert(53, 2);dfa.get_mut(&0).unwrap().insert(32, 1);dfa.get_mut(&0).unwrap().insert(54, 2);dfa.get_mut(&0).unwrap().insert(52, 2);dfa.get_mut(&0).unwrap().insert(57, 2);dfa.get_mut(&0).unwrap().insert(48, 2);dfa.get_mut(&0).unwrap().insert(10, 1);dfa.insert(4, HashMap::new());dfa.get_mut(&4).unwrap().insert(51, 4);dfa.get_mut(&4).unwrap().insert(50, 4);dfa.get_mut(&4).unwrap().insert(55, 4);dfa.get_mut(&4).unwrap().insert(56, 4);dfa.get_mut(&4).unwrap().insert(57, 4);dfa.get_mut(&4).unwrap().insert(53, 4);dfa.get_mut(&4).unwrap().insert(48, 4);dfa.get_mut(&4).unwrap().insert(49, 4);dfa.get_mut(&4).unwrap().insert(54, 4);dfa.get_mut(&4).unwrap().insert(52, 4);dfa.insert(1, HashMap::new());dfa.get_mut(&1).unwrap().insert(32, 1);dfa.get_mut(&1).unwrap().insert(13, 1);dfa.get_mut(&1).unwrap().insert(9, 1);dfa.get_mut(&1).unwrap().insert(10, 1);accepting_states.insert(2, String::from("number"));accepting_states.insert(4, String::from("decnumber"));accepting_states.insert(1, String::from("white"));let mut keywords: HashMap<String, String> = HashMap::new();keywords.insert(String::from( "while" ), String::from("while"));keywords.insert(String::from( "do"), String::from("do"));let mut except_table: HashMap<String, bool> = HashMap::new();except_table.insert(String::from("number"), true);
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
    
        if except_table.contains_key(&accepting_states[&top]) {
            if keywords.contains_key(&curr_lexeme) {
                self.tokens.push(Token::new(String::from("keyword"), curr_lexeme.clone()));
            } else {
                self.tokens.push(Token::new(
                    accepting_states[&top].clone(),
                    curr_lexeme.clone(),
                ));    
            }
        } else {
            self.tokens.push(Token::new(
                accepting_states[&top].clone(),
                curr_lexeme.clone(),
            ));
        }
        
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
    