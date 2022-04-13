fn main() {

}

enum Expression {
    Integer(i32),
    FixedPoint(i32,i32),
    Addition(Vec<Expression>)
}

// Evaluation modules

fn evaluate_integer(expression: &Expression) -> f64 {
    if let Expression::Integer(value) = expression {
        *value as f64
    } else {
        panic!("evaluate_integer did not receive expression type Integer")
    }
}

fn evaluate_add_integers(expressions: &Vec<Expression>) -> Expression {
    let mut total = 0;
    for each in expressions {
        if let Expression::Integer(value) = each {
            total = total + value
        } else {
            panic!("I only can add integers")
        }
    }
    Expression::Integer(total)
}

fn evaluate_addition(expression: &Expression) -> Expression {
    if let Expression::Addition(expressions) = expression {
        match expressions[0] {
            Expression::Integer(_) => evaluate_add_integers(&expressions),
            _ => panic!("I only know how to add integers")
        }

    } else {
        panic!("evalaute_addition did not receive expression type: addition")
    }
}

fn evaluate_fixed_point(expression: &Expression) -> f64 {
    if let Expression::FixedPoint(first, second) = expression {
        let mut decimal = *second as f64;
        while decimal > 1.0 {
            decimal = decimal / 10.0
        }
        return *first as f64 + decimal
    } else {
        panic!("Did not receive fixed point")

    }
}


fn evaluate(expression: &Expression) -> f64 {
    match expression {
        Expression::Integer(_) => evaluate_integer(&expression),
        Expression::Addition(_) => evaluate(&evaluate_addition(&expression)),
        Expression::FixedPoint(_,_) => evaluate_fixed_point(&expression),
        _ => panic!("Not Implemented!")
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn test_anything_works() {
        assert!(true);
    }

    #[test]
    fn test_integer_value() {
        // arrange
        let expr = crate::Expression::Integer(42);
        // act 
        let value = crate::evaluate(&expr);
        // assert
        assert_eq!(value, 42.0, "Should be equal 42 and 42.0")
    }

    #[test]
    fn test_simple_addition() {
        // arrange
        let expression = crate::Expression::Addition(vec![
            crate::Expression::Integer(2),
            crate::Expression::Integer(2)
        ]);
        // act
        let value = crate::evaluate(&expression);
        // assert
        assert_eq!(value, 4.0)
    }

    #[test]
    fn test_fixed_point() {
        // arrange
        let fixed_point = crate::Expression::FixedPoint(12, 45);
        // act 
        let value = crate::evaluate(&fixed_point);
        // assert
        assert_eq!(value, 12.45)
    }
}