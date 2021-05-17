
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
    dfa.insert(9, HashMap::new());dfa.insert(1, HashMap::new());dfa.get_mut(&1).unwrap().insert(50, 1);dfa.get_mut(&1).unwrap().insert(51, 1);dfa.get_mut(&1).unwrap().insert(53, 1);dfa.get_mut(&1).unwrap().insert(56, 1);dfa.get_mut(&1).unwrap().insert(57, 1);dfa.get_mut(&1).unwrap().insert(46, 10);dfa.get_mut(&1).unwrap().insert(48, 1);dfa.get_mut(&1).unwrap().insert(55, 1);dfa.get_mut(&1).unwrap().insert(52, 1);dfa.get_mut(&1).unwrap().insert(54, 1);dfa.get_mut(&1).unwrap().insert(49, 1);dfa.insert(10, HashMap::new());dfa.get_mut(&10).unwrap().insert(57, 11);dfa.get_mut(&10).unwrap().insert(52, 11);dfa.get_mut(&10).unwrap().insert(49, 11);dfa.get_mut(&10).unwrap().insert(48, 11);dfa.get_mut(&10).unwrap().insert(54, 11);dfa.get_mut(&10).unwrap().insert(56, 11);dfa.get_mut(&10).unwrap().insert(51, 11);dfa.get_mut(&10).unwrap().insert(53, 11);dfa.get_mut(&10).unwrap().insert(55, 11);dfa.get_mut(&10).unwrap().insert(50, 11);dfa.insert(6, HashMap::new());dfa.insert(0, HashMap::new());dfa.get_mut(&0).unwrap().insert(42, 6);dfa.get_mut(&0).unwrap().insert(10, 2);dfa.get_mut(&0).unwrap().insert(52, 1);dfa.get_mut(&0).unwrap().insert(54, 1);dfa.get_mut(&0).unwrap().insert(53, 1);dfa.get_mut(&0).unwrap().insert(59, 4);dfa.get_mut(&0).unwrap().insert(9, 2);dfa.get_mut(&0).unwrap().insert(40, 5);dfa.get_mut(&0).unwrap().insert(50, 1);dfa.get_mut(&0).unwrap().insert(56, 1);dfa.get_mut(&0).unwrap().insert(43, 3);dfa.get_mut(&0).unwrap().insert(13, 2);dfa.get_mut(&0).unwrap().insert(41, 9);dfa.get_mut(&0).unwrap().insert(51, 1);dfa.get_mut(&0).unwrap().insert(49, 1);dfa.get_mut(&0).unwrap().insert(48, 1);dfa.get_mut(&0).unwrap().insert(32, 2);dfa.get_mut(&0).unwrap().insert(55, 1);dfa.get_mut(&0).unwrap().insert(57, 1);dfa.get_mut(&0).unwrap().insert(47, 8);dfa.get_mut(&0).unwrap().insert(45, 7);dfa.insert(3, HashMap::new());dfa.insert(4, HashMap::new());dfa.insert(7, HashMap::new());dfa.insert(11, HashMap::new());dfa.get_mut(&11).unwrap().insert(57, 11);dfa.get_mut(&11).unwrap().insert(53, 11);dfa.get_mut(&11).unwrap().insert(49, 11);dfa.get_mut(&11).unwrap().insert(54, 11);dfa.get_mut(&11).unwrap().insert(56, 11);dfa.get_mut(&11).unwrap().insert(55, 11);dfa.get_mut(&11).unwrap().insert(51, 11);dfa.get_mut(&11).unwrap().insert(52, 11);dfa.get_mut(&11).unwrap().insert(50, 11);dfa.get_mut(&11).unwrap().insert(48, 11);dfa.insert(8, HashMap::new());dfa.insert(2, HashMap::new());dfa.get_mut(&2).unwrap().insert(9, 2);dfa.get_mut(&2).unwrap().insert(13, 2);dfa.get_mut(&2).unwrap().insert(10, 2);dfa.get_mut(&2).unwrap().insert(32, 2);dfa.insert(5, HashMap::new());accepting_states.insert(6, String::from("__*__"));accepting_states.insert(2, String::from("white"));accepting_states.insert(7, String::from("__-__"));accepting_states.insert(1, String::from("number"));accepting_states.insert(3, String::from("__+__"));accepting_states.insert(5, String::from("__(__"));accepting_states.insert(8, String::from("__/__"));accepting_states.insert(9, String::from("__)__"));accepting_states.insert(11, String::from("decnumber"));accepting_states.insert(4, String::from("__;__"));let mut keywords: HashMap<String, String> = HashMap::new();keywords.insert(String::from( "while" ), String::from("while"));keywords.insert(String::from( "do"), String::from("do"));let mut except_table: HashMap<String, bool> = HashMap::new();except_table.insert(String::from("number"), true);
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
                self.tokens.push(Token::new(
                    String::from("keyword"),
                    curr_lexeme.clone()
                ));
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
    