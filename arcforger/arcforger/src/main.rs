struct AProperties {
    desc: String
}

trait AFunction {
    fn A(&mut self) -> i32;
}

struct B {
    a: AProperties
}

trait BFunction {
    fn open(&mut self) -> String;
    fn close();
}

impl BFunction for B {
    fn open(&mut self) -> String {
        // something about heap allocation and maybe "sss" = stack allocation which rustc detects
        // well the type is different and you cant coerce stack allocated "sss" to heap allocated String
        String::from("Open Seasame")
    }
    fn close() {

    }
}

impl AFunction for B {
    fn A(&mut self) -> i32 {
        let x = 32;
        x
    }
}

fn main() {
    println!("Hello, world!");

    let mut b = B{a: AProperties{desc: String::from("a description")}};
    let bopen = b.open();
    let bA = b.A();
    let bclose = B::close();
    if bclose == () {
        println!("b closed!");
    }

    println!("b open() = {}, b A() = {}", bopen, bA);
}
