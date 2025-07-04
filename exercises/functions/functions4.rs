// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let price: i32 = 111;
    sale_price(price);
}

fn is_even(x: i32) -> bool {
    return x % 2 == 0
}

fn sale_price(price: i32) -> i32 {
    if (is_even(price)) {
        price - 9
    } else {
        price - 3
    }
}