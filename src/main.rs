fn main() {
    let _x = ciao();

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    //ownership

    let s = "ciao"; //string literal

    print(s);

    println!("{}", s);

    let mut s2 = String::from("ciao2");

    print2(&mut s2); //i need to pass a reference in order to use the string after this line

    println!("{}", s2);
}

fn ciao() -> i32 {
    return 1;
}

fn print(s: &str) {
    println!("{}", s)
}
fn print2(s: &mut String) {
    println!("{}", s);
    s.push_str(" adding some strings");
}
