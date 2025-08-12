# Zero Knowledge Proof (ZKP) Implementation

A Rust implementation of Zero Knowledge Proof using the Chaum-Pedersen protocol. This library demonstrates how a prover can convince a verifier that they know a secret value without revealing the secret itself.

## 🔐 What is Zero Knowledge Proof?

Zero Knowledge Proof is a cryptographic method where one party (the prover) can prove to another party (the verifier) that they know a value x, without conveying any information apart from the fact that they know the value x.

This implementation uses the **Schnorr identification protocol**, which works as follows:

1. **Setup**: Public parameters include generators `g`, `h`, modulus `p`, and subgroup `q`
2. **Commitment**: Prover generates random `r` and computes commitments `t1 = g^r mod p` and `t2 = h^r mod p`
3. **Challenge**: Verifier sends a random challenge `c`
4. **Response**: Prover computes response `s = r - cx mod q` where `x` is the secret
5. **Verification**: Verifier checks if `t1 = g^s * y1^c mod p` and `t2 = h^s * y2^c mod p`

## 🚀 Features

- **Secure Implementation**: Uses cryptographically secure random number generation
- **Big Integer Support**: Handles large numbers using `num_bigint` crate
- **Comprehensive Testing**: Includes fixed value tests, dynamic tests, and randomized testing
- **Clean API**: Simple and intuitive interface for ZKP operations

## 📦 Dependencies

Add this to your `Cargo.toml`:

```toml
[dependencies]
num-bigint = "0.4"
rand = "0.8"
```

## 🛠️ Installation

1. Clone the repository:
```bash
git clone https://github.com/Vipul-sharma119/Zero-knowledge-Proof.git
cd Zero-knowledge-Proof
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## 💡 Usage

### Basic Example

```rust
use num_bigint::BigUint;
use your_crate_name::ZKP;

fn main() {
    // Setup public parameters
    let modulus_p = BigUint::from(23u32);
    let subgrp_q = BigUint::from(11u32);
    let gen_g = BigUint::from(2u32);
    let gen_h = BigUint::from(3u32);
    
    // Create ZKP instance
    let zkp = ZKP {
        gen_g: gen_g.clone(),
        gen_h: gen_h.clone(),
        modulus_p: modulus_p.clone(),
        subgrp_q: subgrp_q.clone(),
    };
    
    // Secret value (known only to prover)
    let secret_x = BigUint::from(4u32);
    
    // Generate public keys
    let y1 = ZKP::mod_exp(&gen_g, &secret_x, &modulus_p);
    let y2 = ZKP::mod_exp(&gen_h, &secret_x, &modulus_p);
    
    // Prover generates random commitment
    let r = ZKP::random(&subgrp_q);
    let t1 = ZKP::mod_exp(&gen_g, &r, &modulus_p);
    let t2 = ZKP::mod_exp(&gen_h, &r, &modulus_p);
    
    // Verifier sends challenge
    let challenge_c = BigUint::from(5u32);
    
    // Prover computes response
    let response_s = zkp.response(&r, &challenge_c, &secret_x);
    
    // Verification
    let is_valid = zkp.verify(&t1, &t2, &challenge_c, &response_s, &y1, &y2);
    println!("Proof is valid: {}", is_valid);
}
```

## 🏗️ API Reference

### `ZKP` Structure

```rust
pub struct ZKP {
    pub gen_g: BigUint,     // Generator g
    pub gen_h: BigUint,     // Generator h  
    pub modulus_p: BigUint, // Prime modulus
    pub subgrp_q: BigUint,  // Subgroup order
}
```

### Methods

#### `mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint`
Performs modular exponentiation: `base^exp mod modulus`

#### `random(bound: &BigUint) -> BigUint`
Generates a cryptographically secure random number below the given bound

#### `response(&self, r: &BigUint, c: &BigUint, x: &BigUint) -> BigUint`
Computes the ZKP response: `s = r - cx mod q`

#### `verify(&self, t1: &BigUint, t2: &BigUint, c: &BigUint, s: &BigUint, y1: &BigUint, y2: &BigUint) -> bool`
Verifies the zero knowledge proof

## 🧪 Testing

The project includes comprehensive tests:

- **Fixed Values Test**: Tests with predetermined values for debugging
- **Dynamic Values Test**: Tests with known random values
- **Random Values Test**: Runs 10 iterations with completely random parameters

Run tests with:
```bash
cargo test
```


WILL ADD CLIENT SIDE AND GRPC IN NEAR FUTURE
