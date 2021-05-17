mod cocor_scanner;
mod parser;
mod parser_gen;
mod scanner;
mod scanner_gen;

use std::collections::{HashMap, HashSet};
use std::env;
use std::process;

/// Cocol token representation.
#[derive(Debug, Clone)]
pub struct CocolToken {
    id: u32,
    name: String,
    regex: String,
}

impl CocolToken {
    pub fn new(id: u32, name: String, regex: String) -> CocolToken {
        CocolToken { id, name, regex }
    }
}

// *********************************************** Main ***********************************************
fn main() {
    // program arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: ./exec \"<grammar_file>\" \"<input_file>\"");
        process::exit(1);
    }

    // categories table
    let mut cat_table: HashMap<String, Vec<char>> = HashMap::new();
    let mut tok_table: HashMap<String, String> = HashMap::new();
    let mut keywords: HashMap<String, String> = HashMap::new();
    let mut tokens: Vec<CocolToken> = Vec::new();
    let mut except_table: HashMap<String, bool> = HashMap::new();
    let mut whitespace: HashSet<char> = HashSet::new();

    // fill the ANY category
    cat_table.insert(String::from("ANY"), Vec::new());
    for i in 32..127 as u8 {
        if i as char == scanner_gen::EXT_CHAR
            || i as char == scanner_gen::UNION_CHAR
            || i as char == scanner_gen::KLEENE_CHAR
            || i as char == scanner_gen::CONCAT_CHAR
            || i as char == scanner_gen::EPSILON
            || i as char == scanner_gen::POSITIVE_CHAR
            || i as char == scanner_gen::OPTIONAL_CHAR
        {
            continue;
        }
        cat_table.get_mut("ANY").unwrap().push(i as char);
    }

    scanner_gen::parse_cocol_file(
        &args[1],
        &mut cat_table,
        &mut keywords,
        &mut tok_table,
        &mut tokens,
        &mut except_table,
        &mut whitespace,
    );

    // initial parse of productions
    let productions = parser_gen::parse_productions(&args[1], &mut tok_table, &mut tokens);

    // EBNF => BNF
    // let bnf = ebnf_to_bnf(&productions);

    // first sets calculation
    let first_sets = parser_gen::calc_first_sets(&productions, &tok_table);

    // println!("*************** COCOL/R Scanner Generator ****************");
    // println!("* Reserved characters:");
    // println!(
    //     "* {} {} {} {} {} {} {}",
    //     EXT_CHAR, UNION_CHAR, KLEENE_CHAR, CONCAT_CHAR, EPSILON, POSITIVE_CHAR, OPTIONAL_CHAR
    // );
    // println!("******************* Scanner Info ***********************");
    // println!("********************* CHARACTERS *************************");
    // for (key, value) in cat_table {
    //     println!("* {} => {:?}", key, value);
    // }
    // println!("********************* KEYWORDS *************************");
    // for (key, value) in &keywords {
    //     println!("* {} => {}", key, value);
    // }
    // println!("********************* TOKENS *************************");
    let mut regex = String::from(scanner_gen::PARENTHESES_OPEN);
    for token in &tokens {
        // extend the current regular expression
        let mut rregex = token.regex.clone();
        let mut count = 1;
        while rregex.chars().nth(rregex.len() - count).unwrap() == scanner_gen::PARENTHESES_CLOSE {
            count += 1;
        }
        rregex.insert(rregex.len() - count + 1, scanner_gen::EXT_CHAR);
        // append the current regex to the global regex with a UNION character.
        regex.push_str(&format!("{}{}", rregex, scanner_gen::UNION_CHAR));
    }
    regex.pop();
    // println!("******************* WHITESPACE ***********************");
    // for c in &whitespace {
    //     println!("* ascii code: {}", *c as u8);
    // }
    // println!("****************** FINAL REGEX ***********************");
    regex.push(scanner_gen::PARENTHESES_CLOSE);
    // replace '?' and '+' operators by the basic operators
    let proc_regex = scanner_gen::preprocess_regex(&regex);
    // create the alphabet using the symbols in the regex
    let mut letters = proc_regex.clone();
    letters.retain(|c| (scanner_gen::is_valid_regex_symbol(&c) && c != scanner_gen::EPSILON));
    let alphabet: HashSet<char> = letters.chars().into_iter().collect();

    // followpos table
    let mut fp_table: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut pos_table: HashMap<char, HashSet<u32>> = HashMap::new();
    let tree_root = scanner_gen::parse_regex(&proc_regex, &mut fp_table, &mut pos_table);

    // regex -> dfa
    let mut accepting_states: HashMap<u32, CocolToken> = HashMap::new();
    let direct_dfa = scanner_gen::regex_dfa(
        &fp_table,
        &pos_table,
        &tokens,
        &mut accepting_states,
        &tree_root,
        &alphabet,
    );

    println!("\n\nProductions...");
    for p in &productions {
        print!("{:?} => ", p.head.lexeme);
        for token in &p.body {
            print!(" {:?} ", token.name);
        }
        println!();
    }
    println!();

    println!("\n\nFirst Sets");
    for (key, value) in &first_sets {
        println!("{:?} => {:?}", key, value);
    }
    println!("\n");

    // code generation
    let scanner_path = "./src/scanner.rs";
    scanner_gen::generate_scanner(
        scanner_path,
        &direct_dfa,
        &accepting_states,
        &keywords,
        &except_table,
        &whitespace,
    );
    println!("Scanner ({}) written correctly.", scanner_path);
    let parser_path = "./src/parser.rs";
    parser_gen::generate_parser(parser_path, &productions, &tok_table, &first_sets);
    println!("Parser ({}) written correctly.", parser_path);

    let mut parser = parser::Parser::new(&args[2]);
    parser.init();
}
