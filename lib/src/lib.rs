use alloy_sol_types::sol;
use tiny_keccak::Hasher;
use tiny_keccak::Sha3;
sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        //uint32 n;
        //uint32 a;
       // uint32 b;

        bytes32 output;

    }
}

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn fibonacci(n: u32) -> (u32, u32) {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    (a, b)
}

pub fn keccak_test() -> [u8; 32] {
    let mut sha3 = Sha3::v256();
    let input = b"hello world";
    let mut output = [0u8; 32];

    sha3.update(input);
    sha3.finalize(&mut output);

    output
}
