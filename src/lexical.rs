// use std::fs::File;
// use std::io::{BufReader};
// use std::io::BufRead;

// pub struct Token {
//     pub token: String,
//     pub value: String,
//     pub line: i32,
//     pub column: i32,
// }

// pub fn lexer(source_code_content: &mut BufReader<File>) {
//     let mut line_number: i32 = 1;
//     // let mut column_number: i32 = 1;

//     for line in source_code_content.lines() {
//         match line {
//             Ok(content) => read_line(&content, &line_number),
//             Err(err) => {
//                 eprintln!("Erro ao ler linha {}: {}", line_number, err);
//                 break;
//             }
//         }
//         line_number += 1;
//     }
// }

// fn read_line(line: &String, line_number: &i32) {
    
// }