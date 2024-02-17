use std::collections::HashMap;
// use crate::operators::Operators::{Addition, And, Assignment, BitAnd, BitOr, BitXor, Division, Eq, Exp, Ge, Gt, Keyword_DECLARE, Le, Lt, Multiplication, Neq, Or, Subtraction, TwoDots, UnaryMinus};
use crate::tokens::{Associativity, dictionary};
use crate::tokens::Associativity::{Left, Right};

// #[derive(Copy, Clone)]
// pub enum Operators {
//     Assignment,
//     Eq,
//     Le,
//     Ge,
//     Lt,
//     Gt,
//     Neq,
//     BitOr,
//     BitAnd,
//     BitXor,
//     And,
//     Or,
//     Addition,
//     Subtraction,
//     Division,
//     Multiplication,
//     Exp,
//     UnaryMinus,
//     TwoDots,
//     Keyword_DECLARE,
// }

// pub(crate) fn operator_to_type<'lifetime>(op: &'lifetime str) -> Option<(Operators, bool)> {
//     let x = dictionary!["=" => (Assignment, false), "==" => (Eq, false), "<=" => (Le, false), ">=" => (Ge, false), "<" => (Lt, false),
//             ">" => (Gt, false), "!=" => (Neq, false), "|" => (BitOr, false), "&" => (BitAnd, false), "^" => (BitXor, false),
//             "&&" => (And, false), "||" => (Or, false), "+" => (Addition, false), "-" => (Subtraction, false), "/" => (Division, false), "*" => (Multiplication, false), "^^" => (Exp, false),
//             "NEGATE" => (UnaryMinus, false), ":" => (TwoDots, false), "declare" => (Keyword_DECLARE, true)];
//     if let Some(val) = x.get(op) {
//         return Some(val.clone());
//     }
//
//     return None
// }

pub(crate) fn precedence_groups<'lifetime>() -> HashMap<&'lifetime str, isize> {
    dictionary!["=" => 9, "==" => 11, "<=" => 11, ">=" => 11, "<" => 11,
            ">" => 11, "!=" => 11, "|" => 12, "&" => 12, "^" => 12,
            "&&" => 13, "||" => 13, "+" => 14, "-" => 14, "/" => 15, "*" => 15, "^^" => 17, "NEGATE" => 16, ":" => 0, "declare" => 10]
}

pub(crate) fn associativity<'lifetime>() -> HashMap<&'lifetime str, Associativity> {
    dictionary!["=" => Left, "==" => Left, "<=" => Left, ">=" => Left, "<" => Left,
            ">" => Left, "!=" => Left, "|" => Left, "&" => Left, "^" => Left,
            "&&" => Left, "||" => Left, "+" => Left, "-" => Left, "/" => Left, "*" => Left, "^^" => Right, "NEGATE" => Left, ":" => Left]
}


pub(crate) fn num_pars<'lifetime>() -> HashMap<&'lifetime str, usize> {
    dictionary!["=" => 2, "==" => 2, "<=" => 2, ">=" => 2, "<" => 2,
            ">" => 2, "!=" => 2, "|" => 2, "&" => 2, "^" => 2,
            "&&" => 2, "||" => 2, "+" => 2, "-" => 2, "/" => 2, "*" => 2, "^^" => 2, "NEGATE" => 1, ":" => 2, "declare" => 1,"func" => 3]
}
