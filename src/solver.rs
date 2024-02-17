use std::collections::HashMap;
use std::ops::Range;
use once_cell::unsync::Lazy;
use crate::tokens::{dictionary, option, TokenizedString};
use crate::tokens::Token;


use crate::tokens::TokenType::{Numeric, Operator, VariableName};
use crate::tokens::TokenUnion::{FloatOf, StrOf};
pub fn solve_reverse_polish_notation(string: &TokenizedString) -> Option<Token> {
    let str_clone = string.clone();
    let mut stack = vec![];
    for element in str_clone {
        if element.token_type != Operator {
            stack.push(element.clone());
        } else if element.token_type == Operator {
            let mut vector = vec![];
            let mut i = 0;
            let num_params = element.num_params;

            while let Some(numeric) = stack.last() {
                vector.push(stack.pop().unwrap());
                i += 1;
                if i >= num_params {
                    break;
                }
            }

            let new_token = new_token_from_applying(&element, vector);

            if let Some(new_token) = new_token {
                stack.push(new_token);
            }
        }
    }
    if stack.last().is_none() == true {
        return None;
    }
    return Some(stack.last().unwrap().clone());
}


fn get_unsafe_float_value(float_tok: &Token) -> f64 {
    if let FloatOf(f) = float_tok.value {
        return f.clone();
    }

    return 0.0;
}

fn get_unsafe_literal_value(float_tok: &Token) -> String {
    if let StrOf(f) = &(float_tok.value) {
        return f.clone();
    }

    return String::from("");
}

pub fn new_token_from_applying(o: &Token, parameters: Vec<Token>) -> option!(Token) {
    if o.token_type != Operator {
        return None;
    }
    // println!("{:?}, {}", o.value, parameters.len());
    if parameters.len() == 2 {
        if let StrOf(operator_value) = &(o.value) {
            let asstr = operator_value.as_str();
            match asstr {
                "+" => {
                    let a = get_unsafe_float_value(parameters.get(0).unwrap());
                    let b = get_unsafe_float_value(parameters.get(1).unwrap());
                    return Some(Token {
                        value: FloatOf(a + b),
                        token_type: Numeric,
                        num_params: 0,
                    });
                }

                "-" => {
                    let a = get_unsafe_float_value(parameters.get(0).unwrap());
                    let b = get_unsafe_float_value(parameters.get(1).unwrap());
                    return Some(Token {
                        value: FloatOf(b - a),
                        token_type: Numeric,
                        num_params: 0,
                    });
                }

                "*" => {
                    let a = get_unsafe_float_value(parameters.get(0).unwrap());
                    let b = get_unsafe_float_value(parameters.get(1).unwrap());
                    return Some(Token {
                        value: FloatOf(a * b),
                        token_type: Numeric,
                        num_params: 0,
                    });
                }

                "/" => {
                    let a = get_unsafe_float_value(parameters.get(0).unwrap());
                    let b = get_unsafe_float_value(parameters.get(1).unwrap());
                    return Some(Token {
                        value: FloatOf(b / a),
                        token_type: Numeric,
                        num_params: 0,
                    });
                }

                "^^" => {
                    let a = get_unsafe_float_value(parameters.get(0).unwrap());
                    let b = get_unsafe_float_value(parameters.get(1).unwrap());
                    return Some(Token {
                        value: FloatOf(b.powf(a)),
                        token_type: Numeric,
                        num_params: 0,
                    });
                }

                "=" => {
                    let a = get_unsafe_float_value(parameters.get(0).unwrap());
                    let b = get_unsafe_literal_value(parameters.get(1).unwrap());
                    // println!("set value of {:?} to {:?};", b, a);
                    return Some(Token {
                        value: StrOf(String::from("")),
                        token_type: Numeric,
                        num_params: 0,
                    });
                }

                _ => {}
            }
        }
    } else if parameters.len() == 1 {
        if let StrOf(operator_value) = &(o.value) {
            let asstr = operator_value.as_str();
            match asstr {
                "NEGATE" => {
                    let a = get_unsafe_float_value(parameters.get(0).unwrap());
                    return Some(Token {
                        value: FloatOf(-a),
                        token_type: Numeric,
                        num_params: 0,
                    });
                }

                "declare" => {
                    let a = get_unsafe_literal_value(parameters.get(0).unwrap());
                    // println!("declared variable {a};");
                    return return Some(Token {
                        value: StrOf(a),
                        token_type: VariableName,
                        num_params: 0,
                    });
                }

                _ => {}
            }
        }
    }

    return None;
}