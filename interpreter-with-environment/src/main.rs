use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Variable(String),
    Number(f64)
}

pub struct Environment {
     hash_map: HashMap<String, Expression>,
     parent: Box<Option<Environment>>,
}

impl Environment {
    fn new(parent: Environment) -> Environment {
        Environment { hash_map: HashMap::new(), parent: Box::new(Option::Some(parent)) }
    }

    fn root() -> Environment {
        Environment{hash_map: HashMap::new(), parent: Box::new(Option::None)}
    }

    fn insert(self: &mut Environment, key: String, expression: Expression) {
        self.hash_map.insert(key, expression);

    }
    fn value_for_key(self: &Environment, key: &String) -> &Expression {
        match self.hash_map.get(key) {
            Some(expression) => expression,
            None => match &*self.parent {
                Some(environment) => environment.value_for_key(key),
                None => panic!("not found in any environment")
            }
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
    use crate::HashMap;
    use crate::Environment;
    use crate::Expression;
    use crate::evaluate;

    fn num(value: f64) -> Expression{
        Expression::Number(value)
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
        let value = crate::evaluate(&number, &crate::Environment::root());
        //assert
        assert_eq!(value, 37.0)
    }



    #[test]
    fn test_addition() {
        let addition = crate::Expression::Add(vec![num(2.0), num(2.0)]);
        let enviroment = &crate::Environment::root();

        let value = crate::evaluate(&addition, enviroment);

        assert_eq!(value, 4.0)
    }
    #[test]
    fn test_variable() {
        let mut environment = Environment::root();
        environment.insert("foo".to_string(), num(5.0));

        let key = "foo".to_string();
        let variable = Expression::Variable("foo".to_string());
        let value = evaluate(&variable, &environment );

        assert_eq!(value, 5.0)
    }
    #[test]
    fn test_subtraction() { 
        let subtraction = crate::Expression::Subtract(vec![num(2.0), num(2.0), num(2.0)]);
        let enviroment = crate::Environment::root();

        let value = crate::evaluate(&subtraction, &enviroment);

        assert_eq!(value, -2.0)
    }
    #[test]
    fn test_variable_in_addition() {
        let variable = Expression::Variable(String::from("foo"));
        let mut environment = Environment::root();
        environment.insert("foo".to_string(), num(5.0));
        let expression = Expression::Add(vec![num(2.0), variable]);
        let value = evaluate(&expression, &environment);

        assert_eq!(value, 7.0)


    }

    #[test]
    fn test_multilevel_environment() {
        // arrange
        let mut root = Environment::root();
        root.insert("foo".to_string(), num(42.0));
        let mut sub = Environment::new(root);
        sub.insert("bar".to_string(), num(5.0));

        // act 
        let should_be_42 = evaluate(&Expression::Variable("foo".to_string()), &sub);


        // assert
        assert_eq!(should_be_42, 42.0)

    }
    #[test]
    fn test_symmetrical_variables() {
        // arrange
        let mut root = Environment::root();
        root.insert("x".to_string(), num(42.0));
        let mut sub = Environment::new(root);
        sub.insert("x".to_string(), num(5.0));

        // act
        let should_be_5 = evaluate(&Expression::Variable("x".to_string()), &sub);

        // assert
        assert_eq!(should_be_5, 5.0)

        
    }

}