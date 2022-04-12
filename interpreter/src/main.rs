
#[derive(Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Variable(String),
    Number(f64)
}

pub struct Environment {
    key: String,
    value: Expression
}

impl Environment {
    fn new() -> Environment {
        Environment { key: String::from(""), value: Expression::Number(0.0) }
    }
    fn value_for_key(self: &Environment, key: &String) -> &Expression {
        if &self.key == key {
            &self.value
        } else {
            panic!("No value for key found in environment")
        }
    }
}

fn evaluate(expression: &Expression, environment: &Environment) -> f64 {
    match expression {
        Expression::Number(value) => *value, 
        Expression::Add(_) => evaluate_addition(expression, environment),
        Expression::Multiply(_) => evaluate_multiplication(expression, environment),
        Expression::Variable(name) => evaluate(&environment.value_for_key(&name), environment),
        Expression::Subtract(_) => evaluate_subtraction(expression, environment)
    }
}

fn evaluate_addition(add: &Expression, environment: &Environment) -> f64 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0.0, |total, next| total + evaluate(next, environment))
    } else {
        panic!("expected addition")
    }

}

fn evaluate_multiplication(multiply: &Expression, environment: &Environment) -> f64 {
    if let Expression::Multiply(expressions) = multiply {
        let iter = expressions.iter();
        iter.fold(1.0, |total, next| total * evaluate(next, environment))
    } else {
        panic!("expected multiplication")
    }
}
fn print_expression(expression: &Expression) {
    match expression {
        Expression::Add(expressions) => {
            print!("(+");
            for e in expressions {
                print!(" ");
                print_expression(e)
            }
            print!(")");
        },
        Expression::Multiply(expressions) => {
            print!("(*");
            for e in expressions {
                print!(" ");
                print_expression(e)
            }
            print!(")");
        },
        Expression::Subtract(expressions) => {
            print!("(-");
            for e in expressions {
                print!(" ");
                print_expression(e)
            }
            print!(")");
        }
        Expression::Variable(variable) => {
            print!("{}",variable);
        }
        Expression::Number(val) => {print!("{}", val)}
    }

}

fn evaluate_subtraction(subtract: &Expression, enviroment: &Environment) -> f64 {
    if let Expression::Subtract(expressions) = subtract {
        let mut iter = expressions.iter();
        let start = iter.next();
        if let Some(Expression::Number(val)) = start {
            iter.fold(*val, |total, next| total - evaluate(next, enviroment))
        } else {
            0.0
        }
        
    } else {
        panic!("expected subtraction")
    }
}

fn main() {

}

#[cfg(test)]
mod testing {

    fn num(value: f64) -> crate::Expression{
        crate::Expression::Number(value)
    }

    #[test]
    fn test_bootstrap() {
        assert_eq!(0,0)
    }

    #[test]
    fn test_evaluate_number() {
        // arrange
        let number = num(37.0);
        // act
        let value = crate::evaluate(&number, &crate::Environment::new());
        //assert
        assert_eq!(value, 37.0)
    }

    #[test]
    fn test_environment_contains_variable() {
        let environment = crate::Environment {key: String::from("blegh"), value: num(5.0)};

        let key = environment.key;
        let value = crate::evaluate(&environment.value,  &crate::Environment::new());

        assert_eq!(key, String::from("blegh"));
        assert_eq!(value, 5.0)
    }

    #[test]
    fn test_environment_holds_value() {
        let environment = crate::Environment{key: String::from("foo"), value: num(42.0)};

        let expression = environment.value_for_key(&String::from("foo"));
        let value = crate::evaluate(&expression,  &crate::Environment::new());

        assert_eq!(value, 42.0)
    }

    #[test]
    fn test_addition() {
        let addition = crate::Expression::Add(vec![num(2.0), num(2.0)]);
        let enviroment = &crate::Environment::new();

        let value = crate::evaluate(&addition, enviroment);

        assert_eq!(value, 4.0)
    }
    #[test]
    fn test_variable() {
        let environment = crate::Environment{key: String::from("foo"), value: num(42.0)};
        let variable = crate::Expression::Variable(String::from("foo"));

        let value = crate::evaluate(&variable, &environment);

        assert_eq!(value, 42.0)
    }
    #[test]
    fn test_subtraction() {
        let subtraction = crate::Expression::Subtract(vec![num(2.0), num(2.0), num(2.0)]);
        let enviroment = crate::Environment::new();

        let value = crate::evaluate(&subtraction, &enviroment);

        assert_eq!(value, -2.0)
    }
}