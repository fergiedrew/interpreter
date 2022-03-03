#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
enum Primitive {
    Add,
    Multiply,
    Subtract,
    Number(i32)
}

fn evaluate(array: Vec<Primitive>) -> i32 {
    let element = &array[0];
    let mut iter = array.iter();
    iter.next();
    match element {
        Primitive::Add => { iter.fold(0, |total, next| total + evaluate(vec![*next])) }
        Primitive::Multiply => { iter.fold(1, |total, next| total * evaluate(vec![*next])) }
        Primitive::Subtract => {
            let start = iter.next();
            if let Some(Primitive::Number(value)) = start {
                iter.fold(*value, |total,next| total - evaluate(vec!(*next)))
            } else {
                0
            } 
        }
        Primitive::Number(val) => *val
    }
}

fn main() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Subtract);
    primitives.push(Primitive::Number(-20));
    primitives.push(Primitive::Number(20));
    primitives.push(Primitive::Number(-20));
    let result = evaluate(primitives);
    println!("Should be -20: {}", result);
}
