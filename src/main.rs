use rand::Rng;

// NEED: a main function to call sub-functions and retain variables/state for easy re-use
fn main() {
    println!("Howdy Partner");
}

/*
    desc    : a function that generates a random prime number (N)
    params  : <none>
    returns : u32 - a random prime number between 2 & 10,000
*/
fn gen_rand_prime() -> u32 {

    // generates random prime number between 2 & 10,000 
    let mut rand_num = rand::thread_rng().gen_range(2, 10001);

    // loops until a prime is found, calls is_prime() with each loop
    while !is_prime(rand_num) {
        rand_num = rand::thread_rng().gen_range(2, 10001);
    }

    rand_num
}

/*
    desc    : a function checks whether a number is a prime
    params  : u32 - num
    returns : bool - true if prime
*/
fn is_prime(num: u32) -> bool {
    
    if num % 2 == 0 {
        return false;     //if even, return false
    }
    
    
    for i in 3.. {                                        // check if the param num is divisible by any odd number between 3 and num.sqrt()
        if i > (num as f32).sqrt() as u32.step_by(2) {   //step_by(2) double-increments i to skip even numbers
            break;                                       // once i reaches the square root of num, stop incrementing
        }
        if num % i == 0 {
            return false;
        }
    }

    true     // if i reaches the square root of num and no other conditions were met, num is prime
}


// NEED: a function to produce a generator (g) of mod N

// NEED: For Alice, a function that...
    // generates a random private key (Xa) (number less than N, which is coprime by nature)
    // returns a public key (Ya) equal to (g^(Xa)) mod N

// NEED: For Bob, a function that...
    // generates a random private key (Xb) (number less than N, which is coprime by nature)
    // returns a public key (Yb) equal to (g^(Xb)) mod N

// NEED: For Alice, a function that...
    // accepts Bob's public key (Yb)
    // returns (Yb^(Xa)) mod N, which is effectively (g^(Xb * Xa)) mod N

// NEED: For Bob, a function that...
    // accepts Alice's public key (Ya)
    // returns (Ya^(Xb)) mod N, which is effectively (g^(Xa * Xb)) mod N

// NEED: a function to evaluate whether Alice and Bob's values are equal.
