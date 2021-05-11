
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
        self.curr = self.scanner.next_token().unwrap();
        self.next = self.scanner.next_token().unwrap();
        self.IdentList();
    }

    fn m(&mut self, t: &str) {
        println!("comparing {} to {}", self.next.name, t);
        if self.next.name == t {
            self.curr = self.next.clone();
            match self.scanner.next_token() {
                Some(token) => {
                    self.next = token;
                },
                _ => self.next = Token::empty(),
            }
        }
    }
fn IdentList (&mut self, ) {self.m("ident"); let mut n = 1; while  self.next.name == "__,__" {self.m("__,__");self.m("ident"); n += 1; } println!("n = {}", n) }
}
