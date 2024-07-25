fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn euler_phi(n: u128) -> u128 {
    let mut count = 0;
    for i in 1..=n {
        if gcd(n, i) == 1 {
            count += 1;
        }
    }
    count
}

fn main() {
    let n = 4900747; // You can change this to any number you want to test
    println!("Euler's Totient Function phi({}) = {}", n, euler_phi(n));
}
