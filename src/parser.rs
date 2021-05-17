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
        self.Expr();
    }

    fn m(&mut self, t: &str) {
        // println!("M: next {} => read: {}", self.next.name, t);
        if self.next.name == t {
            self.curr = self.next.clone();
            match self.scanner.next_token() {
                Some(token) => {
                    self.next = token;
                },
                _ => self.next = Token::empty(),
            }
        } else {
            println!("ERROR: next {} t {}", self.next.name, t);
            panic!("input error!");
        }
    }
fn Expr(&mut self){while self.next.name == "__-__" || self.next.name == "__(__" || self.next.name == "number" || self.next.name == "decnumber"  {self.Stat();self.m("__;__");while self.next.name == "white"  {self.m("white");}}while self.next.name == "white"  {self.m("white");}}fn Stat(&mut self){let mut value: f32 = 0.0;self.Expression(&mut value);println!("Resultado: {}",value);}fn Expression(&mut self,result: &mut f32){let mut result1:f32=0.0;let mut result2:f32=0.0;self.Term(&mut result1);while self.next.name == "__+__" || self.next.name == "__-__"  {match self.next.name.as_str() {"__+__"=>{self.m("__+__");self.Term(&mut result2);result1+=result2;}"__-__"=>{self.m("__-__");self.Term(&mut result2);result1-=result2;}_=>(),}}*result=result1;}fn Term(&mut self,result: &mut f32){let mut result1: &mut f32 = &mut 0.0; let mut result2: &mut f32 = &mut 0.0;self.Factor(&mut result1);while self.next.name == "__*__" || self.next.name == "__/__"  {match self.next.name.as_str() {"__*__"=>{self.m("__*__");self.Factor(&mut result2);*result1*=*result2;}"__/__"=>{self.m("__/__");self.Factor(&mut result2);*result1/=*result2;}_=>(),}}*result=*result1;}fn Factor(&mut self,result: &mut f32){let mut sign: f32 = 1.0;match self.next.name.as_str() {"__-__"=>{self.m("__-__");sign = -1.0;}_=>(),}match self.next.name.as_str() {"number"|"decnumber"=>{self.Number(result);}"__(__"=>{self.m("__(__");self.Expression(result);self.m("__)__");}_=>(),}*result*=sign;}fn Number(&mut self,result: &mut f32){match self.next.name.as_str() {"number"=>{ self.m("number");}"decnumber"=>{ self.m("decnumber");}_=>(),}*result = self.curr.lexeme.parse::<f32>().unwrap();}}
