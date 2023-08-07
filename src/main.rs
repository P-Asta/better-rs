use better;

#[derive(Debug)]
struct Asdf{
    a: isize,
    b: isize,
}

#[better::new]
impl Asdf {
    fn new() -> Asdf{
        Asdf { a: 10, b: 20 }
    }
    fn a(&self) -> isize{
        self.a
    }
}


fn main() {
    let a = Asdf();
    println!("{:?}", a);
    
}

