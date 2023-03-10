use rs_lox_tw::errors::*;
use rs_lox_tw::parser::Parser;
use rs_lox_tw::resolver::Resolver;
use rs_lox_tw::token::Token;
use rs_lox_tw::token_type::TokenType;

mod common;

#[test]
fn test_error_variable_already_exists() {
    let source = "fun bad() {
      var a = \"first\";
      var a = \"second\";
    }";

    let (mut scanner, mut interpreter) = common::scanner_and_interpreter(source);
    let mut resolver = Resolver::new(&mut interpreter);
    if let Ok(tokens) = scanner.scan_tokens() {
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(stmts) => {
                match resolver.resolve_stmts(&stmts) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{e}");
                        return;
                    }
                }

                assert_eq!(
                    interpreter.interpret(&stmts),
                    Err(LoxResult::Resolver {
                        token: Token::identifier(0, 0, 0, TokenType::Identifier, "a"),
                        error_type: ResolverErrorType::VariableAlreadyExists,
                    })
                )
            }
            Err(e) => {
                eprintln!("There was an error: {}", e)
            }
        }
    }
}

#[test]
fn test_invalid_this_top_level() {
    let source = "print this;";

    let (mut scanner, mut interpreter) = common::scanner_and_interpreter(source);
    let mut resolver = Resolver::new(&mut interpreter);
    if let Ok(tokens) = scanner.scan_tokens() {
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(stmts) => {
                assert_eq!(
                    resolver.resolve_stmts(&stmts),
                    Err(LoxResult::Resolver {
                        token: Token {
                            src_end: 0,
                            ttype: TokenType::This,
                            src_line: 0,
                            src_start: 0,
                            lexeme: "this".to_string(),
                            literal: None
                        },
                        error_type: ResolverErrorType::ThisOutsideClass
                    })
                )
            }
            Err(e) => {
                eprintln!("There was an error: {}", e)
            }
        }
    }
}

#[test]
fn test_invalid_this_in_function() {
    let source = "fun notAMethod() {
        print this;
    }";

    let (mut scanner, mut interpreter) = common::scanner_and_interpreter(source);
    let mut resolver = Resolver::new(&mut interpreter);
    if let Ok(tokens) = scanner.scan_tokens() {
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(stmts) => {
                assert_eq!(
                    resolver.resolve_stmts(&stmts),
                    Err(LoxResult::Resolver {
                        token: Token {
                            src_end: 0,
                            ttype: TokenType::This,
                            src_line: 0,
                            src_start: 0,
                            lexeme: "this".to_string(),
                            literal: None
                        },
                        error_type: ResolverErrorType::ThisOutsideClass
                    })
                )
            }
            Err(e) => {
                eprintln!("There was an error: {}", e)
            }
        }
    }
}

#[test]
fn test_return_from_init() {
    let source = "class Foo {
        init() {
            return true;
        }
    }";

    let (mut scanner, mut interpreter) = common::scanner_and_interpreter(source);
    let mut resolver = Resolver::new(&mut interpreter);
    if let Ok(tokens) = scanner.scan_tokens() {
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(stmts) => {
                assert_eq!(
                    resolver.resolve_stmts(&stmts),
                    Err(LoxResult::Resolver {
                        token: Token {
                            src_end: 0,
                            ttype: TokenType::Return,
                            src_line: 0,
                            src_start: 0,
                            lexeme: "return".to_string(),
                            literal: None
                        },
                        error_type: ResolverErrorType::ReturnFromInit
                    })
                )
            }
            Err(e) => {
                eprintln!("There was an error: {}", e)
            }
        }
    }
}

#[test]
fn test_return_from_init() {
    let source = "class Oops < Oops {}";

    let (mut scanner, mut interpreter) = common::scanner_and_interpreter(source);
    let mut resolver = Resolver::new(&mut interpreter);
    if let Ok(tokens) = scanner.scan_tokens() {
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(stmts) => {
                assert_eq!(
                    resolver.resolve_stmts(&stmts),
                    Err(LoxResult::Resolver {
                        token: Token {
                            src_end: 0,
                            ttype: TokenType::Identifier,
                            src_line: 0,
                            src_start: 0,
                            lexeme: "Oops".to_string(),
                            literal: None
                        },
                        error_type: ResolverErrorType::ClassInheritItself
                    })
                )
            }
            Err(e) => {
                eprintln!("There was an error: {}", e)
            }
        }
    }
}
