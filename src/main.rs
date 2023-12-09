fn add (num_one: i32, num_two:i32) -> i32 {
    let sum = num_one + num_two;
    return sum;
}

fn main() {
   let mut total = add(10,15);
   let mut free_shipping = false;

   if total > 50 {
         println!("You qualify for free shipping!");
        free_shipping = true;
    } 
    else if total > 20 {
        println!("If you add more items, you can qualify for free shipping!");
    }
    else {
        println!("No free shipping for you!"); 
    }

    match total {
        0 => println!("Your cart is empty!"),
        1..=50 => println!("Total: {:?}", total),
        _ => println!("No match found"),
        
    }

    total = match free_shipping {
        true =>  add(total, 0),
        false => add(total, 5),
        
    };



    println!("Your total is: {:?}", total);
}
