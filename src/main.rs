// started from : https://stackoverflow.com/questions/66947835/how-to-create-a-static-array-of-objects-that-implement-a-common-trait


trait T { 
    fn get_name(&self) -> &str;
}

#[derive(Clone)]
struct S<'a> { 
    name: &'a str,
}

impl<'a> T for S<'a> {
    fn get_name(&self) -> &'a str {
        self.name.clone()
    }
}


fn main() {
    let s0 = S {name: "xxx"};

    let mut a: Vec<&dyn T> = Vec::new();
    a.push(&s0);   // borrow
    for s in a.iter() {
        println!("{}", s.get_name())
    }
    // borrow of s0 ends here because a is no longer used (and implicitly goes out of scope?).
    // moving the println to the end of the function will cause compile error.

    let mut ab: Vec<Box<dyn T>> = Vec::new();
    ab.push(Box::new(s0));   // move
    for s in ab.iter() {
        println!("{}", s.get_name())
    }

}
