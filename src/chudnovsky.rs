use rug::{
    float::Round,
    Float,
    Integer,
    ops::Pow,
    Rational
};

pub fn compute_pi(prec: u32, iter: i32) -> String {
    let a = Float::with_val(prec, 10005.0).sqrt();
    let b = Float::with_val(prec, 426880.0);

    let mut k = Integer::from(6);
    let mut m = Integer::from(1);
    let mut l = Integer::from(13591409);
    let mut x = Integer::from(1);
    let mut s = Float::with_val(prec, 13591409.0);

    let inc_x = Integer::from(Integer::parse("-262537412640768000").unwrap());

    for i in 1..=iter {
        m = ((k.clone().pow(3) - 16*&k) * &m) / i.pow(3);
        l += 545140134;
        x *= &inc_x;

        s += Rational::from((&m*&l, &x));
        k += 12;
    }

    let pi = a * b / s;
    pi.to_string_radix_round(10, None, Round::Down)
}

#[cfg(test)]
mod tests {
    use super::compute_pi;
    
    #[test]
    fn check_against_pi_f64() {
        let pi = std::f64::consts::PI.to_string();
        let pi_c = compute_pi(50, 5);
        assert_eq!(pi[0..16], pi_c[0..16]);
    }
}
