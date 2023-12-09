use std::vec;

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

    let items:[i32;5] = [1,2,3,4,5];
    println!("Array Items: {:?}", items);

    let vector_items = vec![1,2,3,4,5];
    let mut vector_items_two = Vec::new();
    vector_items_two.push(1);
    vector_items_two.push(2);
    vector_items_two.push(3);
    vector_items_two.push(4);
    vector_items_two.push(5);

    println!("Vector Items: {:?}", vector_items);
    println!("Vector Items Two: {:?}", vector_items_two);
}
