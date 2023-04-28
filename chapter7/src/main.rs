#[derive(Debug, Copy, Clone)]
struct Child(usize);

#[derive(Debug, Copy, Clone)]
struct Parent(usize, Child, Child);
    

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("p2: {:?}", p2);

    println!("p1: {:?}", p1);

    //p1 = Parent(2, Child(21), Child(22));
    //println!("p1: {:?}", p1);
}
