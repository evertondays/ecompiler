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

// TODO adicionar IDENT
fn read_line(
    line: &String,
    tokens: &mut Vec<Token>,
    word: &mut String,
    line_number: &i32,
) {
    let mut chars = line.chars().peekable();
    let mut i: usize = 0;

    while let Some(c) = chars.next() {
        i += 1;

        /* != and == especial case */
        if (c == '!' || c == '=') && let Some('=') = chars.peek() {
            flush_token(word, tokens, *line_number, i);

            word.push(c);
            word.push('=');

            flush_token(word, tokens, *line_number, i);
            chars.next();
            continue;
        }
        
        if c == ' ' {
            flush_token(word, tokens, *line_number, i);
            continue;
        }

        if TERMINAL_CHARACTERS.contains(&c) {
            flush_token(word, tokens, *line_number, i);
            word.push(c);
            flush_token(word, tokens, *line_number, i);
            continue;
        }

        word.push(c);
    }

    flush_token(word, tokens, *line_number, i);
}

fn flush_token(word: &mut String, tokens: &mut Vec<Token>, line_number: i32, column_number: usize) {
    if word.is_empty() {
        return;
    }

    // Check if is a number
    if word.chars().all(|c| DIGITS.contains(&c)) {
        let new_token = Token::new(
            String::from("NUMBER"),
            Some(word.clone()),
            line_number,
            column_number as i32 + 1,
        );
        tokens.push(new_token);
        word.clear();
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

fn insert_end_line_token(tokens: &mut Vec<Token>, line_number: i32) {
    let new_token = Token::new(
        String::from("END_LINE"),
        None,   
        line_number,
        0,
    );
    tokens.push(new_token);
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