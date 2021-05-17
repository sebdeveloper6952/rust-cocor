use crate::cocor_scanner;
use crate::scanner_gen;
use crate::CocolToken;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug)]
pub struct Production {
    pub head: cocor_scanner::Token,
    pub body: Vec<cocor_scanner::Token>,
}

impl Production {
    fn new(head: cocor_scanner::Token, body: Vec<cocor_scanner::Token>) -> Production {
        Production { head, body }
    }
}

/**
 * Calculate the FIRST sets of the given productions.
 */
pub fn calc_first_sets(
    prods: &Vec<Production>,
    tok_table: &HashMap<String, String>,
) -> HashMap<String, Vec<String>> {
    let mut first_sets: HashMap<String, Vec<String>> = HashMap::new();
    for p in prods {
        first_sets.insert(p.head.lexeme.clone(), Vec::new());
    }
    loop {
        let mut changed = false;
        for p in prods {
            // get first token
            let mut i = 0;
            while i < p.body.len() {
                let t = p.body.get(i).unwrap();
                if t.name != "ident" && !t.name.contains("__") {
                    i += 1;
                    continue;
                }
                // check token
                if tok_table.contains_key(&t.lexeme) {
                    if !first_sets[&p.head.lexeme].contains(&t.lexeme) {
                        changed = true;
                        first_sets
                            .get_mut(&p.head.lexeme)
                            .unwrap()
                            .push(t.lexeme.clone());
                    }
                } else if tok_table.contains_key(&t.name) {
                    if !first_sets[&p.head.lexeme].contains(&t.name) {
                        changed = true;
                        first_sets
                            .get_mut(&p.head.lexeme)
                            .unwrap()
                            .push(t.name.clone());
                    }
                } else {
                    let old_copy: HashSet<String> =
                        first_sets[&p.head.lexeme].iter().cloned().collect();
                    let to_insert: HashSet<String> =
                        first_sets[&t.lexeme].iter().cloned().collect();
                    let new_set: HashSet<String> =
                        old_copy.union(&to_insert).map(|s| s.clone()).collect();
                    if !new_set.eq(&old_copy) {
                        changed = true;
                        first_sets.insert(p.head.lexeme.clone(), new_set.into_iter().collect());
                    }
                }

                // check for next token
                i += 1;
                let mut n = p.body.get(i).unwrap();
                while i < p.body.len() - 1 {
                    n = p.body.get(i).unwrap();
                    if n.name == "s_action" || n.name == "attr" {
                        i += 1;
                    } else {
                        break;
                    }
                }
                let n = p.body.get(i).unwrap();
                if n.name == "union" || n.name == "sq_close" {
                    i += 1;
                } else {
                    break;
                }
            }
        }
        if !changed {
            break;
        }
    }
    first_sets
}

/// TODO
pub fn parse_productions(
    path: &str,
    tok_table: &mut HashMap<String, String>,
    tokens: &mut Vec<CocolToken>,
) -> Vec<Production> {
    // generate parser
    let mut title_found = false;
    let mut parsing = false;
    // let mut method = false;
    let mut coco_scanner = cocor_scanner::CocorScanner::new(path);
    let mut productions: Vec<Production> = Vec::new();
    let mut curr_production = Vec::new();
    let mut curr_head = cocor_scanner::Token::new(String::from(""), String::from(""));
    // tokens that are valid for productions
    let mut prod_tokens = HashSet::new();
    prod_tokens.insert("br_open");
    prod_tokens.insert("br_close");
    prod_tokens.insert("sq_open");
    prod_tokens.insert("sq_close");
    prod_tokens.insert("p_open");
    prod_tokens.insert("p_close");
    prod_tokens.insert("union");
    prod_tokens.insert("ident");
    prod_tokens.insert("eq");
    prod_tokens.insert("s_action");
    prod_tokens.insert("attr");
    let mut counter = 0;
    loop {
        match coco_scanner.next_token() {
            Some(token) => {
                if token.lexeme == "PRODUCTIONS" {
                    title_found = true;
                    continue;
                }
                if title_found {
                    if token.name == "ident" && !parsing {
                        parsing = true;
                        if token.lexeme == "END" {
                            break;
                        }
                        curr_head = token.clone();
                    } else if parsing {
                        if prod_tokens.contains(token.name.as_str()) {
                            curr_production.push(token);
                        } else if token.name == "string" {
                            if !tok_table.contains_key(&token.lexeme) {
                                let mut s = token.lexeme.clone();
                                let mut regex = String::from(scanner_gen::PARENTHESES_OPEN);
                                s.remove(0);
                                s.pop();
                                regex.push_str(&format!(
                                    "{}{}{}",
                                    s,
                                    scanner_gen::PARENTHESES_CLOSE,
                                    scanner_gen::CONCAT_CHAR
                                ));
                                let new_name = String::from(&format!("__{}__", s.clone()));
                                let new_token = cocor_scanner::Token::new(
                                    new_name.clone(),
                                    String::from(s.clone()),
                                );
                                tok_table.insert(new_name.clone(), token.lexeme.clone());
                                let new_cocol_token =
                                    CocolToken::new(tokens.len() as u32, new_name, regex);
                                tokens.push(new_cocol_token);
                                curr_production.push(new_token);
                            } else {
                                let mut s = token.lexeme.clone();
                                s.remove(0);
                                s.pop();
                                let cocol_token = tokens.iter().find(|t| t.name == s).unwrap();
                                curr_production.push(cocor_scanner::Token::new(
                                    String::from(&format!("anon_token_{}", cocol_token.id)),
                                    String::from(cocol_token.name.clone()),
                                ));
                            }
                        } else if token.name == "p_end" {
                            parsing = false;
                            let new_prod =
                                Production::new(curr_head.clone(), curr_production.clone());
                            productions.push(new_prod);
                            curr_production.clear();
                        }
                    }
                }
            }
            _ => break,
        }
        counter += 1;
    }
    productions
}

/// TODO
pub fn generate_parser(
    path: &str,
    prods: &Vec<Production>,
    tok_table: &HashMap<String, String>,
    first: &HashMap<String, Vec<String>>,
) {
    let initial_symbol = prods.first().unwrap().head.lexeme.clone();
    let mut code = String::new();
    code.push_str(&format!(
        "use crate::scanner::{{Scanner, Token}};
pub struct Parser {{
    pub next: Token,
    pub curr: Token,
    scanner: Scanner,
}}
impl Parser {{
    pub fn new(path: &str) -> Parser {{
        Parser {{
            next: Token::empty(), 
            curr: Token::empty(),
            scanner: Scanner::new(path),
        }}
    }}

    pub fn init(&mut self) {{
        self.curr = Token::empty();
        self.next = self.scanner.next_token().unwrap();
        self.{}();
    }}

    fn m(&mut self, t: &str) {{
        // println!(\"M: next {{}} => read: {{}}\", self.next.name, t);
        if self.next.name == t {{
            self.curr = self.next.clone();
            match self.scanner.next_token() {{
                Some(token) => {{
                    self.next = token;
                }},
                _ => self.next = Token::empty(),
            }}
        }} else {{
            println!(\"ERROR: next {{}} t {{}}\", self.next.name, t);
            panic!(\"input error!\");
        }}
    }}\n",
        initial_symbol
    ));

    // for each production
    for (i, e) in prods.iter().enumerate() {
        let mut head: Vec<cocor_scanner::Token> = Vec::new();
        let mut body: Vec<cocor_scanner::Token> = Vec::new();
        let mut eq_found = false;
        for t in &e.body {
            if eq_found {
                body.push(t.clone());
            } else {
                head.push(t.clone());
            }
            if t.name == "eq" {
                eq_found = true;
            }
        }

        code.push_str(&format!("fn {}(&mut self", e.head.lexeme));
        for t in head {
            if t.name == "attr" {
                let mut m = t.lexeme.clone();
                m.remove(0);
                m.pop();
                code.push_str(&format!(",{}", m));
            } else if t.name == "eq" {
                code.push_str(&format!("){{"));
            }
        }

        let root = create_prod_tree(&body, &tok_table, &first);
        let prod_code = codegen_preorder_traversal(&root);
        code.push_str(&format!("{}}}", prod_code));
    }
    // close impl block
    code.push_str("}\n");
    println!("\n\n{}", code);
    fs::write(path, code).expect(&format!("Error writing file: {}.", path));
}

/// Kinds of nodes that make up the parse tree of a production.
#[derive(Debug)]
enum PKind {
    Nt,
    Nt2,
    T,
    While,
    Or,
    Action,
    Attr,
    Concat,
    BlockEnd,
    OrEnd,
    Eq,
    PEnd,
}

/// Representation of a node in a production parse tree.
#[derive(Debug)]
struct PNode {
    k: PKind,
    t: cocor_scanner::Token,
    c: Vec<PNode>,
    o: bool,
    first: Vec<String>,
    visited: bool,
}

impl PNode {
    fn new(k: PKind, c: Vec<PNode>, first: Vec<String>) -> PNode {
        let t = cocor_scanner::Token::new(format!(""), format!(""));
        PNode {
            k,
            t,
            c,
            o: false,
            first,
            visited: false,
        }
    }

    fn new_with_token(
        k: PKind,
        t: cocor_scanner::Token,
        c: Vec<PNode>,
        o: bool,
        first: Vec<String>,
    ) -> PNode {
        PNode {
            k,
            t,
            c,
            o,
            first,
            visited: false,
        }
    }

    fn new_end_block() -> PNode {
        let t = cocor_scanner::Token::new(format!(""), format!(""));
        PNode {
            k: PKind::BlockEnd,
            t,
            c: vec![],
            o: false,
            first: vec![],
            visited: false,
        }
    }

    fn new_end_method() -> PNode {
        let t = cocor_scanner::Token::new(format!(")"), format!(")"));
        PNode {
            k: PKind::PEnd,
            t,
            c: vec![],
            o: false,
            first: vec![],
            visited: false,
        }
    }

    fn new_or_block() -> PNode {
        let t = cocor_scanner::Token::new(format!("OR"), format!("OR"));
        PNode {
            k: PKind::OrEnd,
            t,
            c: vec![],
            o: false,
            first: vec![],
            visited: false,
        }
    }

    fn set_o(&mut self, o: bool) {
        self.o = o;
    }
}

/// Create a parse tree for the production body.
fn create_prod_tree(
    // prod: &Production,
    body: &Vec<cocor_scanner::Token>,
    tok_table: &HashMap<String, String>,
    first: &HashMap<String, Vec<String>>,
) -> PNode {
    let mut prec = HashMap::new();
    prec.insert('(', 0);
    prec.insert('|', 2);
    prec.insert('.', 3);
    prec.insert('*', 4);
    prec.insert('~', 4);
    let mut nodes: Vec<PNode> = Vec::new();
    let mut op: Vec<char> = Vec::new();
    let mut index = 0;
    while index < body.len() {
        let curr = &body[index];
        // push concat op if necessary
        if index > 0 {
            let prev = &body[index - 1];
            if (prev.name == "ident"
                || prev.name == "s_action"
                || prev.name == "attr"
                || prev.name == "eq"
                || prev.name.contains("__")
                || prev.name.contains("close"))
                && (curr.name == "ident"
                    || curr.name == "s_action"
                    || curr.name == "attr"
                    || curr.name == "eq"
                    || curr.name.contains("__")
                    || curr.name.contains("open"))
            {
                while op.len() > 0 && prec[op.last().unwrap()] >= prec[&'.'] {
                    let top = op.pop().unwrap();
                    if top == '.' {
                        let c1 = nodes.pop().unwrap();
                        let c0 = nodes.pop().unwrap();
                        let mut fs = c0.first.clone();
                        if fs.len() == 0 {
                            fs.extend(c1.first.clone());
                        }
                        let c2 = PNode::new(PKind::Concat, vec![c1, c0], fs);
                        nodes.push(c2);
                    } else if top == '*' {
                        let c0 = nodes.pop().unwrap();
                        let fs = c0.first.clone();
                        let c1 = PNode::new(PKind::While, vec![PNode::new_end_block(), c0], fs);
                        nodes.push(c1);
                    } else if top == '~' {
                        let mut c0 = nodes.pop().unwrap();
                        c0.set_o(true);
                        let fs = c0.first.clone();
                        let c1 = PNode::new(
                            PKind::Or,
                            vec![PNode::new_or_block(), PNode::new_end_block(), c0],
                            fs,
                        );
                        nodes.push(c1);
                    }
                }
                op.push('.');
            }
        }

        if curr.name.contains("open") {
            op.push('(');
        } else if curr.name == "br_close" {
            while op.len() > 0 && *op.last().unwrap() != '(' {
                let top = op.pop().unwrap();
                if top == '.' {
                    let c1 = nodes.pop().unwrap();
                    let c0 = nodes.pop().unwrap();
                    let mut fs = c0.first.clone();
                    if fs.len() == 0 {
                        fs.extend(c1.first.clone());
                    }
                    let c2 = PNode::new(PKind::Concat, vec![c1, c0], fs);
                    nodes.push(c2);
                } else if top == '*' {
                    let c0 = nodes.pop().unwrap();
                    let fs = c0.first.clone();
                    let c1 = PNode::new(PKind::While, vec![PNode::new_end_block(), c0], fs);
                    nodes.push(c1);
                } else if top == '|' {
                    let mut c1 = nodes.pop().unwrap();
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    c1.set_o(true);
                    let mut fs = c0.first.clone();
                    fs.extend(c1.first.clone());
                    let c2 = PNode::new(
                        PKind::Or,
                        vec![
                            PNode::new_or_block(),
                            PNode::new_end_block(),
                            c1,
                            PNode::new_end_block(),
                            c0,
                        ],
                        fs,
                    );
                    nodes.push(c2);
                } else if top == '~' {
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    let fs = c0.first.clone();
                    let c1 = PNode::new(
                        PKind::Or,
                        vec![PNode::new_or_block(), PNode::new_end_block(), c0],
                        fs,
                    );
                    nodes.push(c1);
                }
            }
            // pop the opening '('
            op.pop().unwrap();
            while op.len() > 0 && prec[op.last().unwrap()] >= prec[&'*'] {
                let top = op.pop().unwrap();
                if top == '.' {
                    let c1 = nodes.pop().unwrap();
                    let c0 = nodes.pop().unwrap();
                    let mut fs = c0.first.clone();
                    if fs.len() == 0 {
                        fs.extend(c1.first.clone());
                    }
                    let c2 = PNode::new(PKind::Concat, vec![c1, c0], fs);
                    nodes.push(c2);
                } else if top == '*' {
                    let c0 = nodes.pop().unwrap();
                    let fs = c0.first.clone();
                    let c1 = PNode::new(PKind::While, vec![PNode::new_end_block(), c0], fs);
                    nodes.push(c1);
                } else if top == '|' {
                    let mut c1 = nodes.pop().unwrap();
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    c1.set_o(true);
                    let mut fs = c0.first.clone();
                    fs.extend(c1.first.clone());
                    let c2 = PNode::new(
                        PKind::Or,
                        vec![
                            PNode::new_or_block(),
                            PNode::new_end_block(),
                            c1,
                            PNode::new_end_block(),
                            c0,
                        ],
                        fs,
                    );
                    nodes.push(c2);
                } else if top == '~' {
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    let fs = c0.first.clone();
                    let c1 = PNode::new(
                        PKind::Or,
                        vec![PNode::new_or_block(), PNode::new_end_block(), c0],
                        fs,
                    );
                    nodes.push(c1);
                }
            }
            op.push('*');
        } else if curr.name == "p_close" {
            while op.len() > 0 && *op.last().unwrap() != '(' {
                let top = op.pop().unwrap();
                if top == '.' {
                    let c1 = nodes.pop().unwrap();
                    let c0 = nodes.pop().unwrap();
                    let mut fs = c0.first.clone();
                    if fs.len() == 0 {
                        fs.extend(c1.first.clone());
                    }
                    let c2 = PNode::new(PKind::Concat, vec![c1, c0], fs);
                    nodes.push(c2);
                } else if top == '*' {
                    let c0 = nodes.pop().unwrap();
                    let fs = c0.first.clone();
                    let c1 = PNode::new(PKind::While, vec![PNode::new_end_block(), c0], fs);
                    nodes.push(c1);
                } else if top == '|' {
                    let mut c1 = nodes.pop().unwrap();
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    c1.set_o(true);
                    let mut fs = c0.first.clone();
                    fs.extend(c1.first.clone());
                    let c2 = PNode::new(
                        PKind::Or,
                        vec![
                            PNode::new_or_block(),
                            PNode::new_end_block(),
                            c1,
                            PNode::new_end_block(),
                            c0,
                        ],
                        fs,
                    );
                    nodes.push(c2);
                } else if top == '~' {
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    let fs = c0.first.clone();
                    let c1 = PNode::new(
                        PKind::Or,
                        vec![PNode::new_or_block(), PNode::new_end_block(), c0],
                        fs,
                    );
                    nodes.push(c1);
                }
            }
            // pop the opening '('
            op.pop().unwrap();
        } else if curr.name == "union" {
            while op.len() > 0 && prec[op.last().unwrap()] >= prec[&'|'] {
                let top = op.pop().unwrap();
                if top == '.' {
                    let c1 = nodes.pop().unwrap();
                    let c0 = nodes.pop().unwrap();
                    let mut fs = c0.first.clone();
                    if fs.len() == 0 {
                        fs.extend(c1.first.clone());
                    }
                    let c2 = PNode::new(PKind::Concat, vec![c1, c0], fs);
                    nodes.push(c2);
                } else if top == '*' {
                    let c0 = nodes.pop().unwrap();
                    let fs = c0.first.clone();
                    let c1 = PNode::new(PKind::While, vec![PNode::new_end_block(), c0], fs);
                    nodes.push(c1);
                } else if top == '|' {
                    let mut c1 = nodes.pop().unwrap();
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    c1.set_o(true);
                    let mut fs = c0.first.clone();
                    fs.extend(c1.first.clone());
                    let c2 = PNode::new(
                        PKind::Or,
                        vec![
                            PNode::new_or_block(),
                            PNode::new_end_block(),
                            c1,
                            PNode::new_end_block(),
                            c0,
                        ],
                        fs,
                    );
                    nodes.push(c2);
                } else if top == '~' {
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    let fs = c0.first.clone();
                    let c1 = PNode::new(
                        PKind::Or,
                        vec![PNode::new_or_block(), PNode::new_end_block(), c0],
                        fs,
                    );
                    nodes.push(c1);
                }
            }
            op.push('|');
        } else if curr.name == "sq_close" {
            while op.len() > 0 && *op.last().unwrap() != '(' {
                let top = op.pop().unwrap();
                if top == '.' {
                    let c1 = nodes.pop().unwrap();
                    let c0 = nodes.pop().unwrap();
                    let mut fs = c0.first.clone();
                    if fs.len() == 0 {
                        fs.extend(c1.first.clone());
                    }
                    let c2 = PNode::new(PKind::Concat, vec![c1, c0], fs);
                    nodes.push(c2);
                } else if top == '*' {
                    let c0 = nodes.pop().unwrap();
                    let fs = c0.first.clone();
                    let c1 = PNode::new(PKind::While, vec![PNode::new_end_block(), c0], fs);
                    nodes.push(c1);
                } else if top == '|' {
                    let mut c1 = nodes.pop().unwrap();
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    c1.set_o(true);
                    let mut fs = c0.first.clone();
                    fs.extend(c1.first.clone());
                    let c2 = PNode::new(
                        PKind::Or,
                        vec![
                            PNode::new_or_block(),
                            PNode::new_end_block(),
                            c1,
                            PNode::new_end_block(),
                            c0,
                        ],
                        fs,
                    );
                    nodes.push(c2);
                } else if top == '~' {
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    let fs = c0.first.clone();
                    let c1 = PNode::new(
                        PKind::Or,
                        vec![PNode::new_or_block(), PNode::new_end_block(), c0],
                        fs,
                    );
                    nodes.push(c1);
                }
            }
            // pop the opening '('
            op.pop().unwrap();
            while op.len() > 0 && prec[op.last().unwrap()] >= prec[&'~'] {
                let top = op.pop().unwrap();
                if top == '.' {
                    let c1 = nodes.pop().unwrap();
                    let c0 = nodes.pop().unwrap();
                    let mut fs = c0.first.clone();
                    if fs.len() == 0 {
                        fs.extend(c1.first.clone());
                    }
                    let c2 = PNode::new(PKind::Concat, vec![c1, c0], fs);
                    nodes.push(c2);
                } else if top == '*' {
                    let c0 = nodes.pop().unwrap();
                    let fs = c0.first.clone();
                    let c1 = PNode::new(PKind::While, vec![PNode::new_end_block(), c0], fs);
                    nodes.push(c1);
                } else if top == '|' {
                    let mut c1 = nodes.pop().unwrap();
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    c1.set_o(true);
                    let mut fs = c0.first.clone();
                    fs.extend(c1.first.clone());
                    let c2 = PNode::new(
                        PKind::Or,
                        vec![
                            PNode::new_or_block(),
                            PNode::new_end_block(),
                            c1,
                            PNode::new_end_block(),
                            c0,
                        ],
                        fs,
                    );
                    nodes.push(c2);
                } else if top == '~' {
                    let mut c0 = nodes.pop().unwrap();
                    c0.set_o(true);
                    let fs = c0.first.clone();
                    let c1 = PNode::new(
                        PKind::Or,
                        vec![PNode::new_or_block(), PNode::new_end_block(), c0],
                        fs,
                    );
                    nodes.push(c1);
                }
            }
            op.push('~');
        } else if tok_table.contains_key(&curr.lexeme) {
            nodes.push(PNode::new_with_token(
                PKind::T,
                curr.clone(),
                vec![],
                false,
                vec![curr.lexeme.clone()],
            ));
        } else if tok_table.contains_key(&curr.name) {
            nodes.push(PNode::new_with_token(
                PKind::T,
                curr.clone(),
                vec![],
                false,
                vec![curr.name.clone()],
            ));
        } else if curr.name == "ident" {
            match body.get(index + 1) {
                Some(token) => {
                    if token.name == "attr" {
                        nodes.push(PNode::new_with_token(
                            PKind::Nt,
                            curr.clone(),
                            vec![],
                            false,
                            first[&curr.lexeme].clone(),
                        ));
                    } else {
                        nodes.push(PNode::new_with_token(
                            PKind::Nt2,
                            curr.clone(),
                            vec![],
                            false,
                            first[&curr.lexeme].clone(),
                        ));
                    }
                }
                _ => (),
            }
        } else if curr.name == "s_action" {
            let mut trimmed = curr.lexeme.clone();
            trimmed.remove(0);
            trimmed.remove(0);
            trimmed.pop();
            trimmed.pop();
            let new_token = cocor_scanner::Token::new(curr.name.clone(), trimmed);
            nodes.push(PNode::new_with_token(
                PKind::Action,
                new_token,
                vec![],
                false,
                vec![],
            ))
        } else if curr.name == "attr" {
            let mut trimmed = curr.lexeme.clone();
            trimmed.remove(0);
            trimmed.pop();
            let new_token = cocor_scanner::Token::new(curr.name.clone(), trimmed);
            nodes.push(PNode::new_with_token(
                PKind::Attr,
                new_token,
                vec![],
                false,
                vec![],
            ));
        } else if curr.name == "eq" {
            nodes.push(PNode::new_with_token(
                PKind::Eq,
                curr.clone(),
                vec![],
                false,
                vec![],
            ));
        }

        index += 1;
    }

    while op.len() > 0 {
        let top = op.pop().unwrap();
        if top == '.' {
            let c1 = nodes.pop().unwrap();
            let c0 = nodes.pop().unwrap();
            let mut fs = c0.first.clone();
            if fs.len() == 0 {
                fs.extend(c1.first.clone());
            }
            let c2 = PNode::new(PKind::Concat, vec![c1, c0], fs);
            nodes.push(c2);
        } else if top == '*' {
            let c0 = nodes.pop().unwrap();
            let fs = c0.first.clone();
            let c1 = PNode::new(PKind::While, vec![PNode::new_end_block(), c0], fs);
            nodes.push(c1);
        } else if top == '|' {
            let mut c1 = nodes.pop().unwrap();
            let mut c0 = nodes.pop().unwrap();
            c0.set_o(true);
            c1.set_o(true);
            let mut fs = c0.first.clone();
            fs.extend(c1.first.clone());
            let c2 = PNode::new(
                PKind::Or,
                vec![
                    PNode::new_or_block(),
                    PNode::new_end_block(),
                    c1,
                    PNode::new_end_block(),
                    c0,
                ],
                fs,
            );
            nodes.push(c2);
        } else if top == '~' {
            let mut c0 = nodes.pop().unwrap();
            c0.set_o(true);
            let fs = c0.first.clone();
            let c1 = PNode::new(
                PKind::Or,
                vec![PNode::new_or_block(), PNode::new_end_block(), c0],
                fs,
            );
            nodes.push(c1);
        }
    }
    nodes.pop().unwrap()
}

/// Generate code for WHILE node.
fn gen_while_code(node: &PNode) -> String {
    let mut cond: String = node
        .first
        .iter()
        .map(|t| format!("self.next.name == \"{}\" || ", t))
        .collect();
    let cond: String = cond.drain(0..cond.len() - 3).collect();
    format!("while {} {{", cond)
}

/// Generate code for CONCAT node.
fn gen_concat_code(node: &PNode) -> String {
    let mut code = String::new();
    if node.o {
        for i in &node.first {
            code.push_str(&format!("\"{}\"|", i));
        }
        code.pop();
        code.push_str(&format!("=>{{"));
    }

    code
}

/// TODO
fn gen_t_code(node: &PNode) -> String {
    let mut code = String::new();
    if node.o {
        if node.t.name.contains("__") {
            code.push_str(&format!(
                "\"{}\"=>{{ self.m(\"{}\");",
                node.t.name, node.t.name
            ));
        } else {
            code.push_str(&format!(
                "\"{}\"=>{{ self.m(\"{}\");",
                node.t.lexeme, node.t.lexeme
            ));
        }
    } else {
        if node.t.name.contains("__") {
            code.push_str(&format!("self.m(\"{}\");", node.t.name));
        } else {
            code.push_str(&format!("self.m(\"{}\");", node.t.lexeme));
        }
    }
    code
}

/// TODO
fn gen_nt_code(node: &PNode) -> String {
    let mut code = String::new();
    if node.o {
        code.push_str(&format!(
            "\"{}\"=>{{ self.m(\"{}\");",
            node.t.name, node.t.name
        ));
    } else {
        code.push_str(&format!("self.{}(", node.t.lexeme));
    }
    match node.k {
        PKind::Nt2 => code.push_str(&format!(");")),
        _ => (),
    }
    code
}

/// TODO
fn gen_prod_code(node: &PNode) -> String {
    match node.k {
        PKind::While => gen_while_code(node),
        PKind::BlockEnd => format!("}}"),
        PKind::Or => String::from("match self.next.name.as_str() {"),
        PKind::OrEnd => String::from("_=>(),}"),
        PKind::Nt | PKind::Nt2 => gen_nt_code(node),
        PKind::T => gen_t_code(node),
        PKind::Action => node.t.lexeme.clone(),
        PKind::Attr => format!("{});", node.t.lexeme.clone()),
        PKind::PEnd => format!(")"),
        PKind::Eq => format!("{{"),
        PKind::Concat => gen_concat_code(node),
        _ => String::new(),
    }
}

/// Code generation inorder traversal of tree.
fn codegen_preorder_traversal(root: &PNode) -> String {
    let mut code = String::new();
    let mut t = vec![root];
    while t.len() > 0 {
        let f = t.pop().unwrap();
        println!("node: {:?} -> first: {:?}", f.k, f.first);
        let segment = gen_prod_code(f);
        code.push_str(&segment);
        for i in f.c.iter() {
            t.push(i);
        }
    }
    code
}
