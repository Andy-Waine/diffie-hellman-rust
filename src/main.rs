
// NEED: a main function to call sub-functions and retain variables/state for easy re-use
fn main() {
    println!("Howdy Partner");
}

// NEED: a function to generate a random prime number (N)

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
