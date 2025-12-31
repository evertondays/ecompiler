use std::fs::File;
use std::io::{BufReader, Write};
use std::io::BufRead;
use std::process;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::Chars;
use std::iter::Peekable;
use lazy_static::lazy_static;

lazy_static! {
    static ref DIGITS: HashSet<char> = {
        let mut digits = HashSet::new();
        digits.insert('0');
        digits.insert('1');
        digits.insert('2');
        digits.insert('3');
        digits.insert('4');
        digits.insert('5');
        digits.insert('6');
        digits.insert('7');
        digits.insert('8');
        digits.insert('9');
        digits
    };

    static ref TOKENS_TABLE: HashMap<String, String> = {
        let mut tokens_table = HashMap::new();
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

        tokens_table.insert(String::from("for"), String::from("RBRACE"));
        tokens_table.insert(String::from("if"), String::from("IF"));
        tokens_table.insert(String::from("else"), String::from("ELSE"));
        tokens_table.insert(String::from("int"), String::from("INT"));
        tokens_table.insert(String::from("bool"), String::from("BOOL"));
        tokens_table.insert(String::from("function"), String::from("FUNCTION"));
        tokens_table
    };

    static ref TERMINAL_CHARACTERS: HashSet<char> = {
        let mut terminal_characters = HashSet::new();
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
        terminal_characters.insert(' ');
        terminal_characters
    };
}

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

    let mut line_number: i32 = 1;
    
    for line in source_code_content.lines() {
        match line {
            Ok(content) => read_line(&content, &mut tokens, &mut word, &line_number),
            Err(err) => {
                eprintln!("Erro ao ler linha {}: {}", line_number, err);
                process::exit(1);
            }
        }

        insert_end_line_token(&mut tokens, line_number);
        line_number += 1;
    }

    write_tokens_to_file(&tokens);
}

fn read_line(
    line: &String,
    tokens: &mut Vec<Token>,
    word: &mut String,
    line_number: &i32,
) {
    let mut state: i8 = 0;

    // 0 - Unknown token
    // 1 - String
    // 2 - Number
    // 3 - Terminal character
    // 4 - Word (variables or keywords)

    let mut chars = line.chars().peekable();
    let mut i: usize = 0;

    while let Some(c) = chars.peek() {
        i += 1;

        if *c == ' ' {
            chars.next();
            continue;
        }

        if state == 0 {
            state = identify_state(&c);
        }

        if state == 1 {
            process_string(&mut chars, tokens);
        } else if state == 2 {
            process_number(&mut chars, tokens);
        } else if state == 3 {
            process_terminal_char(&mut chars, tokens);
        } else {
            // process_word(&mut chars, tokens);
            chars.next();
        }

        state = 0
    }

    flush_token(word, tokens, *line_number, i);
}

fn identify_state(c: &char) -> i8 {
    if *c == '"' {
        return 1
    } else if is_number(&c) {
        return 2
    } else if is_terminal_char(&c) {
        return 3
    } else {
        return 4
    }
}

fn process_string(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let mut open_string = false;
    let mut string_value: String = String::new();

    while let Some(c) = chars.next() {
        // Start of string
        if c == '"' && !open_string {
            open_string = true;
            continue;
        }

        // End of string
        if c == '"' && open_string {
            create_token("STRING", Some(string_value), 0, 0, tokens);
            return;
        }

        string_value.push(c);
    }

    // ! generate string token error
}

fn process_number(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let mut string_value: String = String::new();

    while let Some(c) = chars.peek() {
        if is_number(&c) {
            let c = chars.next().unwrap();
            string_value.push(c);
        } else if is_terminal_char(&c) {
            create_token("NUMBER", Some(string_value), 0, 0, tokens);
            return;
        } else {
            let c = chars.next().unwrap();
            string_value.push(c);
            // create_invalid_token()
        }
    }
    // ! new line - tem que resolver isso depois de mexer no buffer
    create_token("NUMBER", Some(string_value), 0, 0, tokens);
}

fn process_terminal_char(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let mut word = String::new();

    let c = chars.next().unwrap();
    word.push(c);

    /* != and == especial case */
    if (c == '!' || c == '=') && let Some('=') = chars.peek() {
        let next_char = chars.next().unwrap(); // is '='
        word.push(next_char);

        flush_token(&mut word, tokens, 0, 0);
        return;
    }

    flush_token(&mut word, tokens, 0, 0);
}

// TODO adicionar IDENT
fn process_word(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let mut word: String = String::new();

    // TODO != and == especial case

    while let Some(c) = chars.peek() {
    }
}

fn flush_token(word: &mut String, tokens: &mut Vec<Token>, line_number: i32, column_number: usize) {
    if word.is_empty() {
        return;
    }

    let token = match TOKENS_TABLE.get(word) {
        Some(token) => token.clone(),
        None => "ILLEGAL".to_string(),
    };

    let new_token = Token::new(
        token,
        None,
        line_number,
        column_number as i32 + 1,
    );

    tokens.push(new_token);
    word.clear();
}

fn write_tokens_to_file(tokens: &Vec<Token>) {
    let path = "build/lexical.txt";
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Erro ao criar diretório de build!");
        }
    }

    let mut file = File::create(path).expect("Erro ao criar arquivo de saída do lexer");

    for item in tokens {
        if item.token == "END_LINE" {
            writeln!(file)
                .expect("Não foi possível escrever quebra de linha");
            continue;
        }

        if item.value.is_some() {
            write!(
                file,
                "{}|{}({}:{})",
                item.token,
                item.value.as_ref().unwrap(),
                item.line,
                item.column
            )
            .expect("Não foi possível escrever saída do lexer");
            continue;
        }

        write!(
            file,
            "{}({}:{})",
            item.token,
            item.line,
            item.column
        )
        .expect("Não foi possível escrever saída do lexer");
    }
}

// Useful functions 
fn is_number(c: &char) -> bool {
    DIGITS.contains(c)
}

fn is_terminal_char(c: &char) -> bool {
    TERMINAL_CHARACTERS.contains(c)
}

fn create_token(token: &str, value: Option<String>, line: i32, column: i32, tokens: &mut Vec<Token>) {
    let new_token = Token::new(
        token.to_string(),
        value,
        line,
        column,
    );

    tokens.push(new_token);
}

// fn create_invalid_token(token: &str, value: Option<String>, line: i32, column: i32, tokens: &mut Vec<Token>) {

// }

fn insert_end_line_token(tokens: &mut Vec<Token>, line_number: i32) {
    let new_token = Token::new(
        String::from("END_LINE"),
        None,   
        line_number,
        0,
    );
    tokens.push(new_token);
}