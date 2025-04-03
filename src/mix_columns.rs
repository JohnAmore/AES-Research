// MixColumns
//Galois Field multiplication with the P-Box.
//
//P-Box (Global constant)
//[2 3 1 1]
//[1 2 3 1]
//[1 1 2 3]
//[3 1 1 2]
//
//Each number 1-3 is a different polynomial.
//1 = 1 * byte
//2 = x * byte
//3= (x+1) * byte
//
//Take the binary of the byte, "convert" to hexadecmial format, and multiply it by the set
//ploynomial.
//The byte and multiplication is done under the constraint of a Galois Field.
//GF(2^8)
//
//If the byte becomes more than GF(2^8), we need to convert it back into the field, which is where
//hi_bit_set comes in.
//
//a => State box byte value
//b => P-box polynomial value
//hi_bit_set => Boolean value to check for any overloading bits (*1* 0000 0000), or x^8.

use crate::state;

fn hex_multiplication(a: u8, b: u8) -> u8 {
    let mut p = 0;
    let mut hi_bit_set: bool;
    let mut a = a;
    let mut b = b;
    //Handle the multiplication through XOR
    for _ in 0..8 {
        if (b & 1) != 0 {
            p ^= a;
        }
        //Boolean value to check if x^8 is reached. If so, we need to
        //"convert it" to (x^4 + x^3 + x + 1).
        hi_bit_set = (a & 0x80) != 0;
        //"Shift" P-Box left
        a <<= 1;
        if hi_bit_set {
            // XOR with 0x1B to remove the extra bit and convert it.
            a ^= 0x1B;
        }
        //"Shift" P-Box right
        b >>= 1;
    }
    p
}

fn mix_columns(state: &mut state::State) {
    let mut temp = vec![0u8; 16];
    //For each column in state_box (4), compute matrix multiplication method and add it to the temp
    //vector.
    for c in 0..4 {
        let i = c;
        //s0-s4 => Column elements
        let s0 = state.state_box[i];
        let s1 = state.state_box[i + 4];
        let s2 = state.state_box[i + 8];
        let s3 = state.state_box[i + 12];

        temp[i] = hex_multiplication(s0, 2)
            ^ hex_multiplication(s1, 3)
            ^ hex_multiplication(s2, 1)
            ^ hex_multiplication(s3, 1);
        temp[i + 4] = hex_multiplication(s0, 1)
            ^ hex_multiplication(s1, 2)
            ^ hex_multiplication(s2, 3)
            ^ hex_multiplication(s3, 1);
        temp[i + 8] = hex_multiplication(s0, 1)
            ^ hex_multiplication(s1, 1)
            ^ hex_multiplication(s2, 2)
            ^ hex_multiplication(s3, 3);
        temp[i + 12] = hex_multiplication(s0, 3)
            ^ hex_multiplication(s1, 1)
            ^ hex_multiplication(s2, 1)
            ^ hex_multiplication(s3, 2);
    }
    //Set the new temp vector as the current state. MixColumns complete.
    for i in 0..state.state_box.len() {
        state.state_box[i] = temp[i];
    }
}
