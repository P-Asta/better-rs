# better-rs

## new constructor

```rs
use better;

#[derive(Debug)]
struct Asdf{
    a: isize,
    b: isize,
}

#[better::new]
impl Asdf {
    fn new(a: isize, b: isize) -> Asdf{
        Asdf { a: a + 1, b: b + 1 }
    }
    fn a(&self) -> isize{
        self.a
    }
}


fn main() {
    let a = Asdf();
    println!("{:?}", a);
    
}
```
