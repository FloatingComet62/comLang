use comrade::{lexer::Parser, node, Literal, Node, Types, VariableAssignment};

#[test]
fn test() {
    let lexer = Parser::new("let a = 5".to_string());
    let program = lexer.parse(false, false);
    assert_eq!(
        program,
        vec![node!(
            variable_assignment,
            VariableAssignment {
                identifier: vec!["a".to_string()],
                immutability: false,
                publicity: false,
                value: Box::new(vec![node!(
                    literal,
                    Literal {
                        literal: "5".to_string(),
                        l_type: Types::I32
                    }
                )])
            }
        )]
    )
}
