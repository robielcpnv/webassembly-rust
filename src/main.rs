fn hello() {
    let mut my_name = "Robiel";
    my_name = "Robiel Tesfazghi";
    println!("Hello, world! My name is {}!", my_name);
}

fn add (num_one: i32, num_two:i32) -> i32 {
    let sum = num_one + num_two;
    println!("The sum of {} and {} is {}.", num_one, num_two, sum);
    return sum;
}

fn main() {
   hello();
   add(10,5);
}
