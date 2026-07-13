pub fn clamp(x: u64, lo: u64, hi: u64) -> u64 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}
