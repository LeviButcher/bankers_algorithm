pub fn add_vec(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    a.into_iter()
        .zip(b.into_iter())
        .map(|(c, d)| c + d)
        .collect()
}

pub fn subtract_vec(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    a.into_iter()
        .zip(b.into_iter())
        .map(|(c, d)| c - d)
        .collect()
}

pub fn is_less_than_or_equal(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    a.into_iter()
        .zip(b.into_iter())
        .map(|(c, d)| c <= d)
        .fold(true, |acc, curr| acc && curr)
}
