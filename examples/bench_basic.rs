#[liteyear::bench(n = 0..8)]
fn fac(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        n => n * fac(n - 1),
    }
}

fn main() {}
