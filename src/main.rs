use std::io; // use standard input & output
use rand::Rng; // import rand
use std::cmp::Ordering; // order component

fn main() { 
    println!("**press Ctrl + C to quit!");
    println!("\n----- Guess the number ----");

    // Loop is an INFINITE loop
    // lines inside LOOP are executing infinite time
    // possible to break the loop with 'break' keyword
    loop { 
        println!("\nPlease input your Guess: "    );

            // what is mut? Mutable. means, we can change the value of variable.
            // ususally Rust variables are immutable. (we can not change value once assigned) 
        let mut guess = String::new();
                                
                                    // & --> Reference
                                    // References are immutable. (means the value is accessible multiple times without copying into multiple variables)
                                    // So, 'guess' is accessible in anywhere as it is! 
        io::stdin().read_line(&mut guess)
                .expect("Faild to read the line");
        

        println!("Generating secret number...");           
                // generate a random number using rand
                                        // just generate random number also, 
                                        // ensure the generating number is LOCAL only to this Thread / Main thread
                                                    // specify a range
        let secret_number = rand::thread_rng().gen_range(1..100);

        println!("The secret number is {}", secret_number);

        // type conversion guess string to guess integer/number
                                            // parse() is the method which convert string to prefered type (u32 -- 32 bit integer)
        let guess:  u32 = match guess.trim().parse() {
            // checks the  parsed value
            // Ok holds the true result..
            // Err(_) holds the false result...
            Ok(num) => num,
                      // continue the LOOP
            Err(_) => {
                println!("Please enter a number!!");
                continue;
            }
        };
                                // use variables in Strings            
        println!("Your guess is {guess}");

        // compare two values (any 2 that can be compared)
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("So your guess is Small. Let's try one more time!");
            }, 
            Ordering::Greater => {
                println!("So your guess is Big. Let's try one more time!");
            },
            Ordering::Equal => {
                println!("You guessed right!!!!");
                // break the LOOP after correct guess
                break; 
            }
        };
    };

}
