pub struct Parser {
    pub next: Option<Token>,
    pub curr: Option<Token>,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            next: None,
            curr: None,
        }
    }

    fn m(t: &str) {
        println!("{}", t);
    }
    fn Expr() {
        while (true) {
            self.Stat();
            while (true) {
                self.m("white");
            }
        }
        while (true) {
            self.m("white");
        }
    }
    fn Stat() {
        let mut value: f32 = 0.0;
        self.Expression(&mut value);
        println!("Resultado: {}", value);
    }
    fn Expression(result: &mut f32) {
        let mut result1: f32 = 0.0;
        let mut result2: f32 = 0.0;
        self.Term(&result1);
        while (true) {
            self.Term(&mut result2);
            result1 += result2;
            self.Term(&mut result2);
            result1 -= result2;
        }
        result = result1;
    }
    fn Term(result: &mut f32) {
        let mut result1: f32 = 0.0;
        let mut result2: f32 = 0.0;
        self.Factor(&mut result1);
        while (true) {
            self.Factor(&mut result2);
            result1 *= result2;
            self.Factor(&mut result2);
            result1 /= result2;
        }
        result = result1;
    }
    fn Factor(result: &mut f32) {
        let mut sign: f32 = 1.0;
        sign = -1.0;
        self.Number(&mut result);
        self.Expression(&mut result);
        result *= sign;
    }
    fn Number(result: &mut f32) {
        self.m("number");
        self.m("decnumber");
        result = LastToken.Value.parse::<f32>().unwrap();
    }
}
