#[derive(Debug)]
pub enum TokenUnion {
    StrOf(String),
    FloatOf(f64),
}

impl Clone for TokenUnion {
    fn clone(&self) -> Self {
        if let StrOf(s) = self {
            return StrOf(s.clone());
        } else if let FloatOf(f) = self {
            return FloatOf(f.clone());
        }

        return StrOf(String::from("nan"));
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum TokenType {
    Numeric,
    Function,
    Operator,
    OpenPar,
    ClosedPar,
    OpenBracket,
    ClosedBracket,
    OpenCurly,
    ClosedCurly,
    Comma,
    ParamName,
    VariableName
}

#[derive(Clone)]
pub struct Token {
    pub(crate) value: TokenUnion,
    pub(crate) token_type: TokenType,
    pub(crate) num_params: usize,
}

macro_rules! option {
    ($a: ty) => {
        Option<$a>
    };
}

fn remove_spaces_from_str(string: &String) -> String {
    let mut sw = 1;
    let mut strs = String::from("");
    for c in string.chars() {
        if c != ' ' {
            sw = 0;
        }
        if sw == 0 {
            strs = strs + c.to_string().as_str();
        }
    }

    while strs.chars().last() == Some(' ') {
        strs.pop();
    }

    strs
}

macro_rules! empty_in {
    ($v: ident, $vv: ident => $($a: ident; $bb: expr), *) => {
        $(
            if remove_spaces_from_str(&$a) != String::from("") && remove_spaces_from_str(&$a) != String::from(" ") {
                $v.push(remove_spaces_from_str(&$a));
                $vv.push($bb);
            }
            $a = String::from("");
        )*
    };
}
macro_rules! dictionary {
    ($ ($key : expr => $value : expr), * ) => {
        {
            use std::collections::HashMap;
            let mut dict = HashMap::new();
            $(
                dict.insert($key, $value);
            )*
            dict
        }
    };

    ($t: ty, $k: ty) => {
        {
            use std::collections::HashMap;
            let mut dict: HashMap<$t, $k> = HashMap::new();
            dict
        }
    }
}

macro_rules! tokenize {
    ($string: expr) => {
        {
            Token::tokenize_string($string)
        }
    }
}


macro_rules! string_to_rpn {
    ($string: expr) => {
        {
            let tokens = Token::tokenize_string($string);
            Token::reverse_polish_notation(tokens)
        }
    }
}

macro_rules! matches_regex {
    ($string: expr, $reg: expr) => {
        {
            let regex = Regex::new($reg).unwrap();
            regex.is_match($string)
        }
    }
}



use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Range;
pub(crate) use option;
pub(crate) use dictionary;
use regex::{Error, Regex};
pub(crate) use tokenize;
pub(crate) use string_to_rpn;
use crate::operators;
// use crate::operators::Operators;
use crate::tokens::Associativity::{Left, Right};
use crate::tokens::TokenType::{Function, ClosedBracket, ClosedCurly, ClosedPar, Comma, Numeric, OpenBracket, OpenCurly, OpenPar, Operator, ParamName, VariableName};
use crate::tokens::TokenUnion::{FloatOf, StrOf};

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{:?} : {:?}, {:?}]", self.value, self.token_type, self.num_params)
    }
}

pub type TokenizedString = Vec<Token>;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right,
    NoAsoc,
}

// pub fn new_array_from_modify_array_in_place(arr: Vec<Token>, range: Range<usize>, new_arr: Vec<Token>);

impl<'lifetime> Token {
    pub fn value_as_string(&self) -> String {
        if let StrOf(s) = &(self.value) {
            return s.clone();
        } else if let FloatOf(f) = self.value {
            return format!("{}", f);
        }
        return String::from("");
    }
    pub fn tokenize_string(string: &'lifetime str) -> TokenizedString {
        let mut number: String = String::from("");
        let mut variable: String = String::from("");
        let mut operator: String = String::from("");

        let mut vector = vec![];
        let mut v2 = vec![];

        for (index, character) in string.chars().enumerate() {
            let next_char = string.chars().nth(index + 1);
            let mut prev_char = Some(' ');
            if index > 0 {
                prev_char = string.chars().nth(index - 1);
            }

            if matches_regex!(variable.as_str(), r"\s*declare\s*") {
                empty_in!(vector, v2 => variable; Operator, number; Numeric, operator; Operator);
            } else if character == '-' && next_char.is_none() == false && prev_char.is_none() == false &&
                next_char.unwrap() != ' ' && (prev_char.unwrap() == ' ' || prev_char.unwrap() == '(') {
                empty_in!(vector, v2 => variable; Function, number; Numeric, operator; Operator);
                vector.push(String::from("NEGATE"));
                v2.push(Operator);
            } else if character == ',' {
                empty_in!(vector, v2 => variable; Function, number; Numeric, operator; Operator);
                vector.push(String::from(","));
                v2.push(Comma);
            } else if character.is_numeric() || character == '.' {
                number = number + character.to_string().as_str();
                empty_in!(vector, v2 => variable; Function, operator; Operator);
            } else if character.is_alphabetic() || character == ' ' {
                variable = variable + character.to_string().as_str();
                empty_in!(vector, v2 => number; Numeric, operator; Operator);
            } else if character == '(' || character == ')' {
                empty_in!(vector, v2 => operator; Operator);
                operator = operator + character.to_string().as_str();
                let typ = if character == '(' { OpenPar } else { ClosedPar };
                empty_in!(vector, v2 => variable; Function, number; Numeric, operator; typ);
            } else if character == '[' || character == ']' {
                empty_in!(vector, v2 => operator; Operator);
                operator = operator + character.to_string().as_str();
                let typ = if character == '[' { OpenBracket } else { ClosedBracket };
                empty_in!(vector, v2 => variable; Function, number; Numeric, operator; typ);
            } else if character == '{' || character == '}' {
                empty_in!(vector, v2 => operator; Operator);
                operator = operator + character.to_string().as_str();
                let typ = if character == '{' { OpenCurly } else { ClosedCurly };
                empty_in!(vector, v2 => variable; Function, number; Numeric, operator; typ);
            } else {
                operator = operator + character.to_string().as_str();
                empty_in!(vector, v2 => variable; Function, number; Numeric);
            }
        }
        empty_in!(vector, v2 => variable; Function, number; Numeric, operator; Operator);

        let mut vfin: Vec<Token> = vec![];
        for i in 0..vector.len() {
            let typ = v2.get(i);
            let string = vector.get(i);
            if let Some(typ) = typ {
                if let Some(string) = string {
                    if let Some(str) = vector.get(i + 1) {
                        if str == ":" {
                            let opt = Self::get_num_pars(string);
                            vfin.push(Token {
                                value: if *typ == Numeric { FloatOf(string.clone().parse().unwrap()) } else { StrOf(string.clone()) },
                                token_type: ParamName,
                                num_params: if opt.is_none() { 0 } else { opt.unwrap() },
                            });
                            continue;
                        } else if str != "(" && (*typ) == Function {
                            vfin.push(Token {
                                value: StrOf(string.clone()),
                                token_type: VariableName,
                                num_params: 0,
                            });
                            continue;
                        }
                    }
                    let opt = Self::get_num_pars(string);
                    vfin.push(Token {
                        value: if *typ == Numeric { FloatOf(string.clone().parse().unwrap()) } else { StrOf(string.clone()) },
                        token_type: *typ,
                        num_params: if opt.is_none() { 0 } else { opt.unwrap() },
                    })
                }
            }
        }


        return vfin;
    }

    pub fn tokens_to_string(tokenized_string: &TokenizedString) -> String {
        let mut string = String::from("");
        for elem in tokenized_string {
            if let StrOf(s) = &(elem.value) {
                string += s.clone().as_str();
                string += " ";
            } else if let FloatOf(f) = &(elem.value) {
                string += format!("{} ", f).as_str();
            }
        }
        return string;
    }

    pub fn reverse_polish_notation(tokenized_string: TokenizedString) -> Result<TokenizedString, &'lifetime str> {
        let mut output_queue: TokenizedString = vec![];
        let mut operator_stack: TokenizedString = vec![];
        for token in tokenized_string {
            match token.token_type {
                Numeric | VariableName => {
                    output_queue.push(token);
                }

                Function => {
                    operator_stack.push(token);
                }

                ParamName => {
                    output_queue.push(token);
                }

                Comma => {
                    while let Some(o2) = operator_stack.last() {
                        if o2.token_type != OpenPar {
                            output_queue.push(operator_stack.pop().clone().unwrap());
                        } else {
                            break;
                        }
                    }
                }

                Operator => {
                    while let Some(o2) = operator_stack.last() {
                        if !(o2.token_type == Operator || o2.token_type == ClosedPar) {
                            break;
                        }

                        let precedence_o1 = Self::get_precedence_group_t(&token).unwrap();
                        let precedence_o2 = Self::get_precedence_group_t(o2).unwrap();
                        let assoc_o1 = Self::get_associativity_t(&token).unwrap();

                        if precedence_o2 > precedence_o1 || (precedence_o2 == precedence_o1 && assoc_o1 == Left) {
                            output_queue.push(o2.clone());
                            operator_stack.pop();
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(token);
                }

                OpenPar => {
                    operator_stack.push(token);
                }

                ClosedPar => {
                    if operator_stack.last().is_none() {
                        return Err("Error. Bad expression.");
                    }
                    while let Some(o2) = operator_stack.last() {
                        if !(o2.token_type != OpenPar) {
                            break;
                        }
                        output_queue.push(o2.clone());
                        operator_stack.pop();
                    }

                    let top = operator_stack.last();


                    if let Some(top) = top {
                        if top.token_type == OpenPar {
                            operator_stack.pop();
                            /*
                            if there is a function token at the top of the operator stack, then:
                            pop the function from the operator stack into the output queue
                             */
                            let tp = operator_stack.last();
                            if let Some(tp) = tp {
                                if tp.token_type == Function {
                                    output_queue.push(operator_stack.pop().unwrap().clone());
                                }
                            }
                        } else {
                            return Err("Error. Bad expression.");
                        }
                    } else {
                        return Err("Error. Bad expression.");
                    }
                }

                _ => {}
            }
        }

        while operator_stack.is_empty() == false {
            output_queue.push(operator_stack.pop().unwrap());
        }

        return Ok(output_queue);
    }

    fn get_num_pars(operator: &str) -> option!(usize) {
        if let Some(t) = operators::num_pars().get(operator) {
            return Some(t.clone());
        }
        return Some(2);
    }

    fn get_associativity(operator: &str) -> option!(Associativity) {
        if let Some(t) = operators::associativity().get(operator) {
            return Some(t.clone());
        }
        return Some(Left);
    }


    fn get_precedence_group_t(operator: &Token) -> option!(isize) {
        if let StrOf(val) = &(operator.value) {
            let v2: &str = val.as_str();
            if let Some(t) = operators::precedence_groups().get(v2) {
                return Some(t.clone());
            }
        }
        return Some(90);
    }

    fn get_associativity_t(operator: &Token) -> option!(Associativity) {
        if let StrOf(val) = &(operator.value) {
            let v2: &str = val.as_str();
            if let Some(t) = operators::associativity().get(v2) {
                return Some(t.clone());
            }
        }
        return Some(Left);
    }
}