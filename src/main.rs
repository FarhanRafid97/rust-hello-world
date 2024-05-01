fn main() {
    greeting("World")
}

fn greeting(arg: &str) {
    println!("Hello, {}", &arg);
}
