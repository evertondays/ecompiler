use crate::lexical;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Instruction {
    pub tokens: String,
    pub make: String,
    pub is_final: bool,
}

lazy_static! {
    pub static ref INSTRUCTIONS: HashMap<String, Instruction> = {
        let mut m = HashMap::new();
        m.insert(
            "IDENT COLON INT".to_string(),
            Instruction {
                tokens: "IDENT COLON INT".to_string(),
                make: "INT_DECLARATION".to_string(),
                is_final: true,
            },
        );
        m.insert(
            "IDENT COLON BOOL".to_string(),
            Instruction {
                tokens: "IDENT COLON BOOL".to_string(),
                make: "BOOL_DECLARATION".to_string(),
                is_final: true,
            },
        );
        m
    };
}

pub enum ASTNode {
    IntDeclaration(String),  // Variável name
    BoolDeclaration(String), // Variável name
}

impl ASTNode {
    pub fn codegen<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        _module: &Module<'ctx>,
    ) {
        match self {
            ASTNode::IntDeclaration(name) => {
                let i32_type = context.i32_type();
                // Create alloca (allocate stack memory for integer)
                // We name the variable in LLVM IR
                match builder.build_alloca(i32_type, name) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Failed to create int declaration: {:?}", e),
                }
            }
            ASTNode::BoolDeclaration(name) => {
                let i1_type = context.bool_type();
                // Create alloca (allocate stack memory for boolean)
                // We name the variable in LLVM IR
                match builder.build_alloca(i1_type, name) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Failed to create bool declaration: {:?}", e),
                }
            }
        }
    }
}

pub fn codegen(tokens: &mut Vec<lexical::Token>) {
    let context = Context::create();
    let module = context.create_module("program");
    let builder = context.create_builder();

    // Create a generic main function block to hold instructions for testing
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");

    builder.position_at_end(basic_block);

    let mut stack: Vec<lexical::Token> = Vec::new();

    // Consume tokens one by one
    for token in tokens.drain(..) {
        stack.push(token);

        'check_rules: loop {
            let mut reduced = false;

            const MAX_RULE_LEN: usize = 5;
            let current_stack_len = stack.len();
            let check_len = std::cmp::min(current_stack_len, MAX_RULE_LEN);

            for len in (1..=check_len).rev() {
                let start_idx = current_stack_len - len;
                let suffix = &stack[start_idx..];

                // Build key: "TOKEN1 TOKEN2"
                let key = suffix
                    .iter()
                    .map(|t| t.token.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ");

                if let Some(instruction) = INSTRUCTIONS.get(&key) {
                    match instruction.make.as_str() {
                        "INT_DECLARATION" => {
                            process_int_declaration(
                                &mut stack,
                                start_idx,
                                &instruction.make,
                                &context,
                                &builder,
                                &module,
                            );
                            reduced = true;
                            break;
                        }
                        "BOOL_DECLARATION" => {
                            process_bool_declaration(
                                &mut stack,
                                start_idx,
                                &instruction.make,
                                &context,
                                &builder,
                                &module,
                            );
                            reduced = true;
                            break;
                        }
                        _ => {}
                    }
                }
            }

            if !reduced {
                break 'check_rules;
            }
        }
    }

    module.print_to_stderr();
}

fn process_int_declaration<'ctx>(
    stack: &mut Vec<lexical::Token>,
    start_idx: usize,
    token_make: &str,
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    _module: &Module<'ctx>,
) {
    // Pattern: IDENT COLON INT
    // Remove matched tokens from stack and take ownership
    let mut matched_tokens: Vec<lexical::Token> = stack.drain(start_idx..).collect();

    // IDENT is guaranteed to have a value by the lexer
    let name = matched_tokens[0]
        .value
        .take()
        .expect("IDENT matched in parser must have a value");

    let node = ASTNode::IntDeclaration(name);
    node.codegen(context, builder, _module);

    // Push resulting non-terminal token
    stack.push(lexical::Token::new(token_make.to_string(), None, 0, 0));
}

fn process_bool_declaration<'ctx>(
    stack: &mut Vec<lexical::Token>,
    start_idx: usize,
    token_make: &str,
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    _module: &Module<'ctx>,
) {
    // Pattern: IDENT COLON BOOL
    // Remove matched tokens from stack and take ownership
    let mut matched_tokens: Vec<lexical::Token> = stack.drain(start_idx..).collect();

    // IDENT is guaranteed to have a value by the lexer
    let name = matched_tokens[0]
        .value
        .take()
        .expect("IDENT matched in parser must have a value");

    let node = ASTNode::BoolDeclaration(name);
    node.codegen(context, builder, _module);

    // Push resulting non-terminal token
    stack.push(lexical::Token::new(token_make.to_string(), None, 0, 0));
}
