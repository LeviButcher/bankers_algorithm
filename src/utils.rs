pub fn add_tuple(a: (u32, u32, u32, u32), b: (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

pub fn subtract_tuple(a: (u32, u32, u32, u32), b: (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
}

pub fn sum_tuple(a: (u32, u32, u32, u32)) -> u32 {
    a.0 + a.1 + a.2 + a.3
}
