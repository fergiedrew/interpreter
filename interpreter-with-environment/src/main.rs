use std::collections::HashMap;

pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Variable(String),
    Number(f64),
    // Box<> puts it on heap instead of stack?
    Conditional(Box<Predicate>, Box<Expression>, Box<Expression>),
}

pub enum Predicate {
    GreaterThan(Expression, Expression),
    LessThan(Expression, Expression),
    Equals(Expression, Expression)
}

pub struct Environment {
    hash_map: HashMap<String, Expression>,
    parent: Box<Option<Environment>>,
}

impl Environment{
    fn new(parent: Environment) -> Environment {
        Environment{hash_map: HashMap::new(), parent: Box::new(Option::Some(parent))}
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
    match expression{
        Expression::Number(value)=> *value, 
        Expression::Add(_)=> evaluate_addition(expression, environment), 
        Expression::Multiply(_)=> evaluate_multiplication(expression, environment),
        Expression::Subtract(_) => evaluate_subtraction(expression, environment),
        Expression::Variable(name)=> evaluate(&environment.value_for_key(&name), environment),
        Expression::Conditional(_,_,_) => evaluate_conditional(expression, environment),
        _=> panic!("Don't know how")
    }
}
fn evaluate_conditional_equals(predicate: &Predicate, environment: &Environment) -> bool {
    if let Predicate::Equals(left, right) = predicate {
        let l = evaluate(&left, environment);
        let r = evaluate(&right, environment);
        l == r


    } else  {
        panic!("Not given predicate!")
    }

}
fn evaluate_conditional(expression: &Expression, environment: &Environment) -> f64 {
    if let Expression::Conditional(predicate, consequent, alternate) = expression {
        let boolean = match **predicate {
            Predicate::Equals(_,_) => evaluate_conditional_equals(predicate, environment),
            _ => panic!("Unknown predicate type"),
        };
        if boolean {
            evaluate(consequent, environment)
        } else {
            evaluate(alternate, environment)
        }

    } else {
        panic!("Don't know how to evaluate a conditionals")
    }
    
}
fn  evaluate_subtraction(add: &Expression, environment: &Environment) -> f64 {
    if let Expression::Add(expressions) = add {
        let mut iter = expressions.iter();
        let start = iter.next();
        if let Some(Expression::Number(value)) = start{
            iter.fold(*value, |total, next| total - evaluate(next, environment))
        } else {
            0.0
        }
        
    } else {
        panic!("expected subtraction");
    }
}
fn  evaluate_addition(add: &Expression, environment: &Environment) -> f64 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0.0, |total, next| total + evaluate(next, environment))

    } else {
        panic!("expected addition");
    }
}

fn evaluate_multiplication(multiply: &Expression, environment: &Environment )-> f64 {
    if let Expression::Multiply(expressions) = multiply {
        let iter = expressions.iter();
        iter.fold(1.0, |total, next| total * evaluate(next, environment))

    } else {
        panic!("expected multiplication")
    }
}

fn main() {}


//configure testing modules

#[cfg(test)]

mod tests {

    use crate::Expression;
    use crate::Environment;
    use crate::Predicate;

    fn num(value: f64)-> crate::Expression{
        Expression::Number(value)
    }

    #[test]
    fn test_bootstrap(){
        assert_eq!(0, 0);
    }
    #[test]
    fn test_evaluate_number(){
        //three phases: arrange, act, assert
        //arrange
        let number = num(42.0);
        //act
        let value = crate::evaluate(&number, &crate::Environment::root());
        //assert

        assert_eq!(value, 42.0);
    }

    #[test]
    fn test_Environment_contains_variable(){
        //arrange
        let mut environment = Environment::root();
        environment.insert("foo".to_string(), num(5.0));
        //act
        let key = "foo".to_string(); 
        let value = crate::evaluate(&Expression::Variable("foo".to_string()), &environment); 
        //assert
        assert_eq!(value, 5.0);
    }

    #[test]
    fn test_addition(){
        //arrange
        let addition = crate::Expression::Add(vec![num(2.0), num(2.0)]);

        //act
        let value = crate::evaluate(&addition, &crate::Environment::root());
        //assert
        assert_eq!(value, 4.0); 
    }

    #[test]

    fn test_multiplication(){
        //arrange
        let multiplication = crate::Expression::Multiply(vec![num(2.0), num(3.0)]);
        //act
        let value = crate::evaluate(&multiplication, &crate::Environment::root());
        //assert
        assert_eq!(value, 6.0);
    }


    #[test]
    fn test_variable_in_addition(){
        //define foo 5.0 (+2.0 foo)

        let variable = crate::Expression::Variable(String::from("foo"));
        let mut environment = Environment::root();
        environment.insert("foo".to_string(), num(5.0));
        let expression = Expression::Add(vec![num (2.0), variable]); 
        //act
        let value = crate::evaluate(&expression, &environment);

        //assert
        assert_eq!(value, 7.0);
    }

    #[test]
    fn test_multilevel_environment() {
        // arrange
        let mut root = Environment::root();
        root.insert("foo".to_string(), num(42.0));
        let mut sub = Environment::new(root);
        sub.insert("bar".to_string(), num(5.0));
        // act
        let should_be_42 = crate::evaluate(&Expression::Variable("foo".to_string()), &sub);
        // assert
        assert_eq!(should_be_42, 42.0);
    }

    #[test]
    fn test_conditional() {
        // arrange
        let mut environment = Environment::root();
        environment.insert("foo".to_string(), num(2.0));
        let expr = Expression::Conditional(
            Box::new(Predicate::Equals(Expression::Variable("foo".to_string()), num(2.0))),
            Box::new(num(23.0)),
            Box::new(num(42.0))
        );
        // act
        let value = crate::evaluate(&expr, &environment);
        // assert
        assert_eq!(value, 23.0)

    }

}
