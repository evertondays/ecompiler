use std::fs::File;
use std::io::{BufReader};
use std::io::BufRead;
use std::process;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Token {
    pub token: String,
    pub value: Option<String>,
    pub line: i32,
    pub column: i32,
}

impl Token {
    pub fn new(
        token: String,
        value: Option<String>,
        line: i32,
        column: i32,
    ) -> Self {
        Self {
            token,
            value,
            line,
            column,
        }
    }
}

pub fn get_tokens(source_code_content: &mut BufReader<File>) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut word: String = String::new();

    let tokens_table: HashMap<String,String> = initialize_tokens_hashmap();
    let terminal_characters: HashSet<char> = initialize_terminal_characters();
    let mut line_number: i32 = 1;
    
    for line in source_code_content.lines() {
        match line {
            Ok(content) => read_line(&content, &mut tokens, &mut word, &line_number, &tokens_table, &terminal_characters),
            Err(err) => {
                eprintln!("Erro ao ler linha {}: {}", line_number, err);
                process::exit(1);
            }
        }
        line_number += 1;
    }
}

fn read_line(
    line: &String,
    tokens: &mut Vec<Token>,
    word: &mut String,
    line_number: &i32,
    tokens_table: &HashMap<String,String>,
    terminal_characters: &HashSet<char>,

) {
    let chars: Vec<char> = line.chars().collect();

    for i in 0..chars.len() {
        let c: char = chars[i];

        /* != and == especial case */
        if (c == '!' && chars[i+1] == '=') || (c == '=' && chars[i+1] == '=') {
            flush_token(word, tokens, tokens_table, *line_number, i);
            word.push(c);
            continue;
        }
        
        if c == ' ' {
            // flush_token();
            continue;
        }

        if terminal_characters.contains(&c) {
            // flush_token();
            word.push(c);
            // flush_token();
            continue;
        }

        word.push(c);
    }
}

fn flush_token(word: &mut String, tokens: &mut Vec<Token>, tokens_table: &HashMap<String,String>, line_number: i32, column_number: usize) {
    if !word.is_empty() {
        let token = match tokens_table.get(word) {
            Some(token) => token.clone(),
            None => "UNKNOWN".to_string(),
        };

        let new_token = Token::new(
            token,
            None,
            line_number,
            column_number as i32 + 1,
        );

        tokens.push(new_token);
    }
    
    word.clear();
}

fn initialize_tokens_hashmap() -> HashMap<String,String> {
    let mut tokens_table: HashMap<String,String> = HashMap::new();

    tokens_table.insert(String::from(":"), String::from("COLON"));
    tokens_table.insert(String::from("="), String::from("ASSIGN"));

    tokens_table.insert(String::from("+"), String::from("PLUS"));
    tokens_table.insert(String::from("-"), String::from("MINUS"));
    tokens_table.insert(String::from("*"), String::from("ASTERISK"));
    tokens_table.insert(String::from("/"), String::from("DIVIDE"));
    tokens_table.insert(String::from("!"), String::from("BANG"));

    tokens_table.insert(String::from("=="), String::from("EQUAL"));
    tokens_table.insert(String::from("!="), String::from("DIFFERENT"));

    tokens_table.insert(String::from("("), String::from("LPAREN"));
    tokens_table.insert(String::from(")"), String::from("RPAREN"));
    tokens_table.insert(String::from("{"), String::from("LBRACE"));
    tokens_table.insert(String::from("}"), String::from("RBRACE"));

    return tokens_table;
}

fn initialize_terminal_characters() -> HashSet<char> {
    let mut terminal_characters: HashSet<char> = HashSet::new();

    terminal_characters.insert(':');
    terminal_characters.insert('=');

    terminal_characters.insert('+');
    terminal_characters.insert('-');
    terminal_characters.insert('*');
    terminal_characters.insert('/');
    terminal_characters.insert('!');

    terminal_characters.insert('(');
    terminal_characters.insert(')');
    terminal_characters.insert('{');
    terminal_characters.insert('}');

    return terminal_characters;
}