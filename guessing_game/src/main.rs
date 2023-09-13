use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let x= 2.1; // this is f64
        // let x :f32 = 3.0; // this is f32
        // let z : chaar = 'ℤ' 
        // let heart_eyed_cat = '😻';
        // tuple
        // let tup: (i32, f64, u8) = (500, 6.4, 1);
        // let five_hundred = tup.0;
        // array
        // let a = [1, 2, 3, 4, 5]; // or let a: [i32; 5] = [1, 2, 3, 4, 5];
        // let first = a[0];
        


    
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break; }

    }
   
    }
}
