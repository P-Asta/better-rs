use better;

struct Asdf{
    a: isize,
    b: isize,
}
#[better::new]
impl Asdf {
    fn new(a: isize, b: isize) -> Self{
        Self { a: a, b }
    }
    fn bb(a: isize, b: isize) -> Self{
        Self { a: a, b }
    }
}



fn main() {
    // let a = Asdf{a: 1, };
    // println!("{:?}",a.a);
}