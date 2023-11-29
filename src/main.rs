use better;

#[derive(Debug)]
struct Asdf{
    a: isize,
    b: isize,
}

#[better::new]
impl Asdf {
    fn new(a: isize, b: isize) -> Asdf{
        Asdf { a: a, b: b + 100 }
    }
}


fn main() {
    let a = Asdf(1, 3);
    println!("{:?}", a);
    
}

