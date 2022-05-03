use std::collections::HashMap;

pub enum Expression{
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Variable(String),
    Number(f64),
}

pub struct Enviroment {
    hash_map: HashMap<String, Expression>,
    parent: Box<Option<Enviroment>>,
}

impl Enviroment{
    fn new(parent: Enviroment) -> Enviroment {
        Enviroment{hash_map: HashMap::new(), parent: Box::new(Option::Some(parent))}
    }

    fn root() -> Enviroment {
        Enviroment{hash_map: HashMap::new(), parent: Box::new(Option::None)}
    }

    fn insert(self: &mut Enviroment, key: String, expression: Expression) {
        self.hash_map.insert(key, expression);
    }

    fn value_for_key(self: &Enviroment, key: &String) -> &Expression {
        match self.hash_map.get(key) {
            Some(expression) => expression,
            None => match &*self.parent {
                Some(environment) => environment.value_for_key(key),
                None => panic!("not found in any environment")
            }
        }
    }
}

fn evaluate(expression: &Expression, enviroment: &Enviroment) -> f64{
    match expression{
        Expression::Number(value)=> *value, 
        Expression::Add(_)=> evaluate_addition(expression, enviroment), 
        Expression::Multiply(_)=> evaluate_multiplication(expression, enviroment),
        Expression::Subtract(_) => evaluate_subtraction(expression, enviroment),
        Expression::Variable(name)=> evaluate(&enviroment.value_for_key(&name), enviroment),
        _=> -1.0
    }
}
fn  evaluate_subtraction(add: &Expression, enviroment: &Enviroment) -> f64 {
    if let Expression::Add(expressions) = add {
        let mut iter = expressions.iter();
        let start = iter.next();
        if let Some(Expression::Number(value)) = start{
            iter.fold(*value, |total, next| total - evaluate(next, enviroment))
        } else {
            0.0
        }
        
    } else {
        panic!("expected subtraction");
    }
}
fn  evaluate_addition(add: &Expression, enviroment: &Enviroment) -> f64 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0.0, |total, next| total + evaluate(next, enviroment))

    } else {
        panic!("expected addition");
    }
}

fn evaluate_multiplication(multiply: &Expression, enviroment: &Enviroment )-> f64 {
    if let Expression::Multiply(expressions) = multiply {
        let iter = expressions.iter();
        iter.fold(1.0, |total, next| total * evaluate(next, enviroment))

    } else {
        panic!("expected multiplication")
    }
}

fn main() {}


//configure testing modules

#[cfg(test)]

mod tests {

    use crate::Expression;
    use crate::Enviroment;

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
        let value = crate::evaluate(&number, &crate::Enviroment::root());
        //assert

        assert_eq!(value, 42.0);
    }

    #[test]
    fn test_enviroment_contains_variable(){
        //arrange
        let mut enviroment = Enviroment::root();
        enviroment.insert("foo".to_string(), num(5.0));
        //act
        let key = "foo".to_string(); 
        let value = crate::evaluate(&Expression::Variable("foo".to_string()), &enviroment); 
        //assert
        assert_eq!(value, 5.0);
    }

    #[test]
    fn test_addition(){
        //arrange
        let addition = crate::Expression::Add(vec![num(2.0), num(2.0)]);

        //act
        let value = crate::evaluate(&addition, &crate::Enviroment::root());
        //assert
        assert_eq!(value, 4.0); 
    }

    #[test]

    fn test_multiplication(){
        //arrange
        let multiplication = crate::Expression::Multiply(vec![num(2.0), num(3.0)]);
        //act
        let value = crate::evaluate(&multiplication, &crate::Enviroment::root());
        //assert
        assert_eq!(value, 6.0);
    }


    #[test]
    fn test_variable_in_addition(){
        //define foo 5.0 (+2.0 foo)

        let variable = crate::Expression::Variable(String::from("foo"));
        let mut environment = Enviroment::root();
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
        let mut root = Enviroment::root();
        root.insert("foo".to_string(), num(42.0));
        let mut sub = Enviroment::new(root);
        sub.insert("bar".to_string(), num(5.0));
        // act
        let should_be_42 = crate::evaluate(&Expression::Variable("foo".to_string()), &sub);
        // assert
        assert_eq!(should_be_42, 42.0);
    }

}
