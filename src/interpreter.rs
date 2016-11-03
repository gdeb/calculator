use super::tokenizer::tokenize;
use super::tokenizer::Operator;
use super::parser::parse;
use super::parser::AST;
use super::parser::ParseError;

fn evaluate_ast(ast: &AST) -> isize {
    match ast {
        &AST::Value(n) => n as isize,
        &AST::Operator(Operator::Add, ref left, ref right) => {
            evaluate_ast(&*left) + evaluate_ast(&*right)
        },
        &AST::Operator(Operator::Substract, ref left, ref right) => {
            evaluate_ast(&*left) - evaluate_ast(&*right)
        },
        &AST::Operator(Operator::Multiply, ref left, ref right) => {
            evaluate_ast(&*left) * evaluate_ast(&*right)
        },
    }
}

pub fn evaluate(expr: &str) -> Result<isize, ParseError> {
    let mut tokens = tokenize(expr);
    let ast = try!(parse(&mut tokens));
    Ok(evaluate_ast(&ast))
}

#[cfg(test)]
mod tests {
    use super::evaluate;

    #[test]
    fn basic_eval_test() {
        assert_eq!(evaluate("42"), Ok(42));
        assert_eq!(evaluate("1 + 1"), Ok(2));
        assert_eq!(evaluate("2 * 3 + 1"), Ok(7));
        assert_eq!(evaluate("1 + 2 * 3"), Ok(7));
        assert_eq!(evaluate("6 * 10 + 2 - 7 * 3"), Ok(41));
    }

    #[test]
    fn eval_with_parenthesis() {
        assert_eq!(evaluate("((42))"), Ok(42));
        assert_eq!(evaluate("(1 + 2) * 3"), Ok(9));
        assert_eq!(evaluate("(2 * 3) - (14 + 3*2) * ((4))"), Ok(-74));
    }

    #[test]
    fn eval_with_prefix_minus() {
        assert_eq!(evaluate("-4"), Ok(-4));
        assert_eq!(evaluate("-4 + 4"), Ok(0));
        assert_eq!(evaluate("1 + -4 * 2"), Ok(-7));
        assert_eq!(evaluate("3*-4 "), Ok(-12));
    }

}

