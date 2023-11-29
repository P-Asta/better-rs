use better;
#[derive(Debug)]
struct Asdf{
    a: isize,
    b: isize,
}
#[better::new]
impl Asdf {
    fn new(a: isize, b: isize, mul: isize) -> Asdf{
        Asdf { a: a*mul, b: b*mul }
    }
}


fn main() {
    let a = (0..=10).into_iter().collect::<Vec<isize>>();
    println!("{:?}", a);

    let a = Asdf(1, 3, 10);
    println!("{:?}", a);
}