use crate::{node, Expression, Match, Node};

use super::compiler;

pub fn compile(input: &mut Match) -> String {
    let mut output = String::new();
    let to_eval = &compiler(&mut input.condition, false);
    for (i, case) in input.block.iter().enumerate() {
        if i == 0 {
            output += "if(";
            output += to_eval;
            output += " == ";
            output += &compiler(&mut case.case.clone(), false);
            output += ") {";
            output += &compiler(&mut case.block.clone(), false);
            output += "}";
            continue;
        }
        if case.case
            == vec![node!(
                expression,
                Expression {
                    expr: vec!["default".to_string()],
                }
            )]
        {
            output += "else {";
            output += &compiler(&mut case.block.clone(), false);
            output += "}";
            continue;
        }
        output += "else if(";
        output += to_eval;
        output += " == ";
        output += &compiler(&mut case.case.clone(), false);
        output += ") {";
        output += &compiler(&mut case.block.clone(), false);
        output += "}";
    }
    output
}