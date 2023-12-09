fn hello() {
    let mut my_name = "Robiel";
    my_name = "Robiel Tesfazghi";
    println!("Hello, world! My name is {}!", my_name);
}

fn add () {
    let x = 5;
    let y = 10;
    let sum = x + y;
    println!("The sum of {} and {} is {}.", x, y, sum);
}

fn main() {
   hello();
   add()
}
