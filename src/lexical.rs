use std::fs::File;
use std::io::{BufReader, Write};
use std::io::BufRead;
use std::process;
use std::fs;
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

        insert_end_line_token(&mut tokens, line_number);
        line_number += 1;
    }

    write_tokens_to_file(&tokens);
}

// TODO adicionar ILLEGAL e IDENT
fn read_line(
    line: &String,
    tokens: &mut Vec<Token>,
    word: &mut String,
    line_number: &i32,
    tokens_table: &HashMap<String,String>,
    terminal_characters: &HashSet<char>,

) {
    let mut chars = line.chars().peekable();
    let mut i: usize = 0;

    while let Some(c) = chars.next() {
        i += 1;
        /* != and == especial case */
        if (c == '!' || c == '=') && let Some('=') = chars.peek() {
            flush_token(word, tokens, tokens_table, *line_number, i);

            word.push(c);
            word.push('=');

            flush_token(word, tokens, tokens_table, *line_number, i);
            chars.next();
            continue;
        }
        
        if c == ' ' {
            flush_token(word, tokens, tokens_table, *line_number, i);
            continue;
        }

        if terminal_characters.contains(&c) {
            flush_token(word, tokens, tokens_table, *line_number, i);
            word.push(c);
            flush_token(word, tokens, tokens_table, *line_number, i);
            continue;
        }

        chars.next();
        word.push(c);
    }

    flush_token(word, tokens, tokens_table, *line_number, i);
}

fn flush_token(word: &mut String, tokens: &mut Vec<Token>, tokens_table: &HashMap<String,String>, line_number: i32, column_number: usize) {
    if word.is_empty() {
        return;
    }

    let token = match tokens_table.get(word) {
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