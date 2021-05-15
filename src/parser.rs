use crate::scanner::{Scanner, Token};
pub struct Parser {
    pub next: Token,
    pub curr: Token,
    scanner: Scanner,
}
impl Parser {
    pub fn new(path: &str) -> Parser {
        Parser {
            next: Token::empty(),
            curr: Token::empty(),
            scanner: Scanner::new(path),
        }
    }

    pub fn init(&mut self) {
        self.curr = Token::empty();
        self.next = self.scanner.next_token().unwrap();
        // self.IdentList();
    }

    fn m(&mut self, t: &str) {
        println!("next {} t {}", self.next.lexeme, t);
        if self.next.name == t {
            self.curr = self.next.clone();
            match self.scanner.next_token() {
                Some(token) => {
                    self.next = token;
                }
                _ => self.next = Token::empty(),
            }
        } else {
            println!("ERROR: next {} t {}", self.next.lexeme, t);
            panic!("input error!");
        }
    }
    // fn IdentList(&mut self) {
    //     self.m("ident");
    //     let mut n = 1;
    //     while self.next.name == "__,__" || self.next.name == "__.__" {
    //         match self.next.name.as_str() {
    //             "__,__" => {
    //                 self.m("__,__");
    //                 self.m("ident");
    //                 n += 1;
    //             }
    //             "__.__" => {
    //                 self.m("__.__");
    //                 self.m("ident");
    //                 n += 1;
    //             }
    //             _ => (),
    //         }
    //         println!("Hay {} elementos en la lista.", n);
    //     }
    // }
}
