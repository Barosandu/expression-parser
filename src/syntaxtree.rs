#[derive(Clone)]
pub struct SyntaxTreeNode {
    value: TokenUnion,
    children: Vec<SyntaxTreeNode>,
}

impl SyntaxTreeNode {
    pub fn add_child(&mut self, child: SyntaxTreeNode) {
        self.children.push(child);
    }

    pub fn value_as_string(&self) -> String {
        if let StrOf(s) = &(self.value) {
            return s.clone();
        } else if let FloatOf(f) = self.value {
            return format!("{}", f);
        }
        return String::from("");
    }

    pub fn print(&self) {
        self.pretty_print(0);
    }

    fn pretty_print(&self, level: usize) {
        let string = "\t".repeat(level);
        println!("{}- Name: {}", string, self.value_as_string());
        let vec_children: Vec<String> = self.children.iter().map(|c| c.value_as_string()).collect();
        for child in &self.children {
            child.pretty_print(level + 1);
        }
    }
}

use std::fmt::{Display, Formatter};
use crate::tokens::{Token, TokenizedString, TokenType, TokenUnion};
use crate::tokens::TokenType::{Function, Numeric, Operator};
use crate::tokens::TokenUnion::{FloatOf, StrOf};


macro_rules! replace_in_array {
    ($vector: expr => $start: expr, $end: expr; $new_vector: expr) => {
        {
            let mut ret_val = vec![];
            for token in 0..(usize::from($start)) {
                ret_val.push($vector.get(token).unwrap().clone());
            }

            for token in $new_vector {
                ret_val.push(token.clone());
            }

            for token2 in (usize::from($end)+1)..($vector.len()) {
                ret_val.push($vector.get(token2).unwrap().clone());
            }

            ret_val
        }
    };
}

pub fn polish_notation_to_normal(tokenized_string: &TokenizedString) -> TokenizedString {
    let string = detokenize_tokens(tokenized_string.clone());
    return string;
}

trait Reversable {
    fn reversed(&self) -> TokenizedString;
}

impl Reversable for TokenizedString {
    fn reversed(&self) -> TokenizedString {
        let mut x = self.clone();
        x.reverse();
        return x;
    }
}
pub(crate) fn reverse_polish_string_to_tree(tokenized_string: &TokenizedString) -> Vec<SyntaxTreeNode> {
    let mut stack: Vec<SyntaxTreeNode> = vec![];
    for (index, element) in tokenized_string.iter().enumerate() {
        let mut node = SyntaxTreeNode {
            value: StrOf(element.value_as_string()),
            children: vec![]
        };

        if element.token_type == Operator || element.token_type == Function {
            let num_par = element.num_params;
            let mut children = vec![];
            for _ in 0..num_par {
                children.push(stack.pop().unwrap());
            }

            node.children = children;

        }
        stack.push(node);
    }

    return stack;
}

fn detokenize_tokens(tokenized_string: TokenizedString) -> TokenizedString {
    let filtered: Vec<&Token> = tokenized_string.iter().filter(|toke|
        {
            return toke.token_type == Operator || toke.token_type == Function;
        }
    ).collect();

    if filtered.is_empty() {
        return tokenized_string;
    } else {
        for (index, element) in tokenized_string.iter().enumerate() {
            match element.token_type {
                Operator => {
                    if (element.num_params == 2) {
                        let token = Token {
                            value: StrOf(format!("({} {} {})", tokenized_string.get(index - 2).unwrap().value_as_string(),
                                                 element.value_as_string(), tokenized_string.get(index - 1).unwrap().value_as_string())),
                            token_type: Numeric,
                            num_params: 0,
                        };
                        let new_tokenized = replace_in_array!(tokenized_string => index - element.num_params, index; vec![token]);
                        return detokenize_tokens(new_tokenized);
                    } else {
                        let token = Token {
                            value: StrOf(format!("({} {})",
                                                 element.value_as_string(), tokenized_string.get(index - 1).unwrap().value_as_string())),
                            token_type: Numeric,
                            num_params: 0,
                        };
                        let new_tokenized = replace_in_array!(tokenized_string => index - element.num_params, index; vec![token]);
                        return detokenize_tokens(new_tokenized);
                    }
                }

                Function => {
                    let mut index_copy = index;
                    let mut value = String::from("[");
                    value += element.value_as_string().as_str();
                    value += "; ";
                    while index_copy != index - element.num_params {
                        index_copy -= 1;
                        value = value + tokenized_string.get(index_copy).unwrap().value_as_string().as_str();
                        if index_copy != index - element.num_params {
                            value = value + "; ";
                        }
                    }
                    value += "]";

                    // println!("value: {}", value);
                    let token = Token {
                        value: StrOf(value),
                        token_type: Numeric,
                        num_params: 0,
                    };
                    let new_tokenized = replace_in_array!(tokenized_string => index - element.num_params, index; vec![token]);
                    return detokenize_tokens(new_tokenized);
                }
                _ => {}
            }
        }
        return tokenized_string;
    }
}

