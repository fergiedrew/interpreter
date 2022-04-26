use std::collections::HashMap;

fn main() {
    println!("{}", 03)

}

enum Expression {
    Integer(i32),
    FixedPoint(i32,i32),
    Addition(Vec<Expression>)
}

// Evaluation modules for operations with expressions

fn evaluate(expression: &Expression) -> f64 {
    match expression {
        Expression::Integer(_) => evaluate_integer(&expression),
        Expression::Addition(_) => evaluate(&evaluate_addition(&expression)),
        Expression::FixedPoint(_,_) => evaluate_fixed_point(&expression),
        _ => panic!("Not Implemented!")
    }
}

fn evaluate_integer(expression: &Expression) -> f64 {
    if let Expression::Integer(value) = expression {
        *value as f64
    } else {
        panic!("evaluate_integer did not receive expression type Integer")
    }
}


fn evaluate_fixed_point(expression: &Expression) -> f64 {
    if let Expression::FixedPoint(b,a) = expression {
        let mut before = *b as f64;
        let mut after = *a as f64;
        while after >= 100.0 {
            after -= 100.0;
            before += 1.0;
        }
        return before + (after / 100.0)
    } else {
        panic!("evaluate_fixed_point() not given type Expression::FixedPoint");
    }

}

fn evaluate_addition(expression: &Expression) -> Expression {
    if let Expression::Addition(expressions) = expression {
        match expressions[0] {
            Expression::Integer(_) => evaluate_add_integers(&expressions),
            Expression::FixedPoint(_,_) => evaluate_add_fixed_points(&expressions),
            _ => panic!("I only know how to add integers")
        }

    } else {
        panic!("evalaute_addition did not receive expression type: addition")
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


fn evaluate_add_fixed_points(expressions: &Vec<Expression>) -> Expression {
    let mut beforesum = 0;
    let mut aftersum = 0;
    for each in expressions {
        beforesum += before_decimal(&each);
        aftersum += after_decimal(&each);
        if aftersum >= 100 {
            aftersum -= 100;
            beforesum += 1;
        }
    }
    return Expression::FixedPoint(beforesum, aftersum)
}



// Getters for fields in Expressions

fn before_decimal(fixed_point: &Expression) -> i32 {
    if let Expression::FixedPoint(before, _) = fixed_point {
        return *before
    } else {
        panic!("Not given fixed point!");
    }
    
}

// Getter for after decimal point
fn after_decimal(fixed_point: &Expression) -> i32 {
    if let Expression::FixedPoint(_, after) = fixed_point {
        return *after
    } else {
        panic!("Not given fixed point!");
    }
    
}

fn get_length(integer: &i32) -> i32 {
    let mut len = 1;
    let mut modulus = 10;
    while *integer % modulus != *integer {
        modulus *= 10;
        len += 1;
    }
    return len
}







// ----------------------------------------------------------------------------------------------------------- //
// --------------------------------------------- Old Code ---------------------------------------------------- //
// ----------------------------------------------------------------------------------------------------------- //

// Does not work for any results or products that have a leading 0 after decimal point
// fn evaluate_add_fixed_points(expressions: &Vec<Expression>) -> Expression {
//     let mut sum = Expression::FixedPoint(0,0);
//     let mut beforesum = 0;
//     let mut aftersum = 0;

//     for fixed_point in expressions {
//         // Get the length of the number after decimal point
//         let mut previous_length = get_length(&after_decimal(&sum));
//         println!("{}", previous_length);

//         // Get number to add onto after decimal point as well as its length
//         let mut addon = after_decimal(&fixed_point);
//         let mut addon_length = get_length(&after_decimal(&fixed_point));


//         // If no fractional part, keep number after decimal and add before decimal
//         if addon  == 0 || after_decimal(&sum) == 0 {
//             beforesum += before_decimal(&fixed_point);
//             aftersum += after_decimal(&fixed_point);
//             sum = Expression::FixedPoint(beforesum, aftersum);
//             continue;
//         }
        

//         // Make the after decimal the same length, that is, same power of 10
//         if previous_length > addon_length {
//             addon = addon * 10i32.pow((previous_length - addon_length) as u32);
//         } else {
//             aftersum = aftersum * 10i32.pow((addon_length - previous_length) as u32);
//         }
        
//         // Add before decimal and after decimal
//         beforesum += before_decimal(&fixed_point);
//         aftersum += addon;

//         // Get length after adding fractional parts
//         let mut new_length = get_length(&aftersum);

//         // If length increased, carry the one to before sum
//         if new_length > previous_length {
//             beforesum += 1;
//             aftersum = aftersum - 10i32.pow(previous_length as u32);
//         }

//         // Put everything in a fixed point
//         sum = Expression::FixedPoint(beforesum, aftersum);

//     }

//     return sum;
// }

// fn evaluate_fixed_point(expression: &Expression) -> f64 {
//     if let Expression::FixedPoint(first, second) = expression {
//         let mut decimal = *second as f64;
//         while decimal > 1.0 {
//             decimal = decimal / 10.0
//         }
//         return *first as f64 + decimal
//     } else {
//         panic!("Did not receive fixed point")
//     }
// }






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


    #[test]
    fn test_fixed_point_addition() {
        // arrange 
        let num1 = crate::Expression::FixedPoint(0, 92);
        let num2 = crate::Expression::FixedPoint(0, 9);
        let addition  = crate::Expression::Addition(vec![num1,num2]);
        // act 
        let value = crate::evaluate(&addition);
        // assert
        assert_eq!(value, 1.01)
    }
}
