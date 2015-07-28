fn main() {
    let number = 10;
    // ok, pretty straightforward. `number` means the number 10.
    let mut a_mutable_number = 10;
    // `mut` is short for `mutable`; this means it can change.
    // `a_mutable_number` is 10 now, but might not be 10 later.
    let typed_number: i32 = 10;
    // a bit weird. i32 is a number type (32 bit number).
    // `typed_number` is a specific kind of number.
    // They wear a jersey with a signed 32 on it and play for the integer team.
    let reference_number = &mut a_mutable_number;
    // weirder. `mut` swapped locations and married an ampersand. Why?
    // Maybe it's time I start playing around with them.
}
