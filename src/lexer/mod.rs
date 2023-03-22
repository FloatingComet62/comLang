use crate::Types;
use std::{fmt::Debug, ops::Deref};

mod _const;
mod _if;
mod _let;
mod _match;
mod _struct;
mod _while;
mod booleans;
mod fun;
mod fun_call;
mod include_n_return;
mod math;

#[derive(Debug)]
pub enum Operations {
    NULL,

    ADD,  // addition
    SUB,  // subtraction
    MUL,  // multiplication
    DIV,  // division
    EQ,   // equal
    EQGR, // equal or greater than
    EQLT, // equal or less than
    GR,   // greater than
    LT,   // less than
}
#[derive(Debug)]
pub enum Immutablity {
    True,
    False,
}
#[derive(Debug, Clone)]
pub struct Argument {
    pub identifier: String,
    pub a_type: Types,
}
#[derive(Debug)]
pub struct Literal {
    pub literal: String,
    pub l_type: Types,
}
#[derive(Debug)]
pub struct Statement {
    pub action: String,
    pub parameters: Vec<String>, //todo Vec<Node>
}
#[derive(Debug)]
pub struct Function {
    pub identifier: Vec<String>,
    pub return_type: Types,
    pub arguments: Vec<Argument>,
    pub nodes: Vec<Node>,
}
#[derive(Debug)]
pub struct FunctionCall {
    pub identifier: Vec<String>,
    pub arguments: Vec<Vec<Node>>,
}
#[derive(Debug)]
pub struct VariableAssignment {
    pub identifier: Vec<String>,
    pub value: Box<Vec<Node>>,
    pub immutability: Immutablity,
}
#[derive(Debug)]
pub struct Expression {
    pub expr: Vec<String>, // maybe node? idk
}
#[derive(Debug)]
pub struct ConditionBlock {
    pub keyword: String,
    pub parameters: Vec<Node>,
    pub nodes: Vec<Node>,
}
#[derive(Debug)]
pub struct MatchCase {
    pub case: Vec<String>, // maybe something else, eg. ["addition", or, "add", or "a"]
    pub block: Vec<Node>,
}
#[derive(Debug)]
pub struct Match {
    pub condition: Vec<Node>,
    pub block: Vec<MatchCase>,
}
#[derive(Debug)]
pub struct StructMember {
    pub identifier: Vec<String>,
    pub t_mem: Types,
}
#[derive(Debug)]
pub struct Struct {
    pub identifier: Vec<String>,
    pub members: Vec<StructMember>,
}
#[derive(Debug)]
pub struct Math {
    pub lhs: Expression,
    pub rhs: Expression,
    pub operation: Operations,
}
//todo identifier ?
pub struct NodeData {
    pub statement: Option<Statement>,
    pub function: Option<Function>,
    pub function_call: Option<FunctionCall>,
    pub variable_assignment: Option<VariableAssignment>,
    pub expression: Option<Expression>,
    pub condition_block: Option<ConditionBlock>,
    pub _match: Option<Match>,
    pub literal: Option<Literal>,
    pub math: Option<Math>,
    pub _struct: Option<Struct>,
}
impl Debug for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(statement) = &self.statement {
            return f.write_str(&format!("{:?}", statement));
        }
        if let Some(function) = &self.function {
            return f.write_str(&format!("{:?}", function));
        }
        if let Some(function_call) = &self.function_call {
            return f.write_str(&format!("{:?}", function_call));
        }
        if let Some(variable_assignment) = &self.variable_assignment {
            return f.write_str(&format!("{:?}", variable_assignment));
        }
        if let Some(expression) = &self.expression {
            return f.write_str(&format!("{:?}", expression));
        }
        if let Some(condition_block) = &self.condition_block {
            return f.write_str(&format!("{:?}", condition_block));
        }
        if let Some(_match) = &self._match {
            return f.write_str(&format!("{:?}", _match));
        }
        if let Some(literal) = &self.literal {
            return f.write_str(&format!("{:?}", literal));
        }
        if let Some(m) = &self.math {
            return f.write_str(&format!("{:?}", m));
        }
        if let Some(s) = &self._struct {
            return f.write_str(&format!("{:?}", s));
        }

        f.write_str("{}")
    }
}
// todo: if by the end of the parser, all node has is "data", just make Node NodeData
pub struct Node {
    pub data: NodeData,
}
impl Deref for Node {
    type Target = NodeData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl Debug for Node {
    // don't display n_type
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.data))
    }
}
impl Node {
    pub fn new(
        statement: Option<Statement>,
        function: Option<Function>,
        function_call: Option<FunctionCall>,
        variable_assignment: Option<VariableAssignment>,
        expression: Option<Expression>,
        condition_block: Option<ConditionBlock>,
        _match: Option<Match>,
        literal: Option<Literal>,
        m: Option<Math>,
        s: Option<Struct>,
    ) -> Self {
        Self {
            data: NodeData {
                statement,
                function,
                function_call,
                variable_assignment,
                expression,
                condition_block,
                _match,
                literal,
                math: m,
                _struct: s,
            },
        }
    }
    pub fn blank() -> Node {
        Node::new(None, None, None, None, None, None, None, None, None, None)
    }
}

pub struct Lexer {
    pub splitted_text: Vec<String>,
    pub program: Vec<Node>,
    pub keywords: Vec<String>,
}
impl Lexer {
    pub fn new(splitted_text: Vec<String>) -> Self {
        Self {
            splitted_text,
            program: vec![],
            keywords: vec![
                String::from("include"),
                String::from("fun"),
                String::from("return"),
            ],
        }
    }
}

/*
    Returns:
    new index
    content till token
    block found
*/
pub fn get_till_token_or_block(
    token: &str,
    input: &Vec<String>,
    i: usize,
) -> (usize, Vec<String>, Vec<String>, bool) {
    let mut output: Vec<String> = vec![];
    let mut block: Vec<String> = vec![];
    let mut got_block = false;
    let mut getting_block = 0;
    let mut getting_function_call = false;
    let mut is_comment = false;
    let mut j = i + 1;
    while j < input.len() {
        let text = &input[j];
        // ! must be first
        if text == "#" {
            is_comment = true;
        }
        if text == "EOL" {
            is_comment = false;
        }
        if !is_comment {
            if text == "{" {
                getting_block += 1;
            }
            if text == "(" && token != "(" {
                // we are not getting the (, then get the entire function
                getting_function_call = true;
            }
            if text == token && getting_block == 0 && !getting_function_call {
                break;
            }
            if getting_block > 0 {
                block.push(text.to_string());
            } else {
                output.push(text.to_string());
            }
            // ! must be last
            if text == "}" {
                got_block = true;
                getting_block -= 1;
                if getting_block == 0 {
                    // got a random block?
                    // IT PROBABLY WAS IMPORTANT OR SOMETHING, ISN'T IT
                    // stop everything, just return
                    break;
                }
            }
            if text == ")" {
                getting_function_call = false;
            }
        }

        j += 1;
    }
    return (j, output, block, got_block);
}

pub fn load(input: &Vec<String>) -> Vec<Node> {
    let mut program = vec![];
    let mut previous_text;
    let mut text = &String::new();
    let mut i = 0;
    while i < input.len() {
        let data = get_till_token_or_block("EOL", input, i);
        previous_text = text.clone();
        text = &input[i];

        if text == "#" {
            i += data.0;
            continue;
        }
        // skip EOLs
        if text == "EOL" {
            i += 1;
            continue;
        }

        if text == "include" || text == "return" {
            i = include_n_return::parser(&mut program, data, text);
        } else if text == "true" || text == "false" {
            i = booleans::parser(&mut program, data, text);
        } else if text == "let" {
            i = _let::parser(&mut program, data, input, i);
        } else if text == "const" {
            i = _const::parser(&mut program, data, input, i);
        } else if text == "if" || text == "else" {
            i = _if::parser(&mut program, data, text, &previous_text, input, i);
        } else if text == "while" {
            i = _while::parser(&mut program, data);
        } else if text == "match" {
            i = _match::parser(&mut program, data);
        } else if text == "struct" {
            i = _struct::parser(&mut program, data);
        } else if text == "fun" {
            i = fun::parser(&mut program, data);
        } else if has(&data.1, vec!["(", ")"], Mode::AND) {
            // also, it's a function call when there is no fun
            i = fun_call::parser(&mut program, text, input, i);
        } else if has(
            &data.1,
            vec!["+", "-", "*", "/", ">", "<", "<=", ">="],
            Mode::OR,
        ) {
            i = math::parser(&mut program, text, data, input, i);
        } else if text.chars().next().unwrap_or('\0') == '\"' {
            program.push(Node::new(
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Literal {
                    literal: text.to_string(),
                    l_type: Types::Str,
                }),
                None,
                None,
            ));
        } else if is_digit(text.chars().next().unwrap_or('\0')) {
            program.push(Node::new(
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Literal {
                    literal: text.to_string(),
                    l_type: Types::I32, // * i32 is the default number
                }),
                None,
                None,
            ));
        }
        i += 1;
    }

    program
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

enum Mode {
    OR,
    AND,
}
fn has(data: &Vec<String>, things: Vec<&str>, mode: Mode) -> bool {
    match mode {
        Mode::AND => {
            for thing in things {
                if !data.contains(&thing.to_string()) {
                    return false;
                }
            }
            true
        }
        Mode::OR => {
            for thing in things {
                if data.contains(&thing.to_string()) {
                    return true;
                }
            }
            false
        }
    }
}
