<<<<<<< HEAD
#[derive(Debug)]
=======
#[derive(Copy, Clone)]
>>>>>>> 7845c772d0b1c04d93514b7ffca1e4a822fa7dce
enum Primitive {
    Add,
    Multiply,
    Number(i32)
}

fn evaluate(array: Vec<Primitive>) -> i32 {
    let element = &array[0];
    let mut iter = array.iter();
    iter.next();
    match element {
        Primitive::Add => { iter.fold(0, |total, next| total + evaluate(vec![*next])) }
        Primitive::Multiply => { iter.fold(1, |total, next| total * evaluate(vec![*next])) }
        Primitive::Number(val) => *val
    }
}

fn main() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Add);
    primitives.push(Primitive::Number(17));
    primitives.push(Primitive::Number(-754));
    let result = evaluate(primitives);
    println!("{}", result);
}
