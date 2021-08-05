pub fn fizzbuzz_kronke(x: u16) -> &'static str {
    if x % 3 == 0 {
        if x % 5 == 0 {
            return "Fizz Buzz"
        }
        return "Fizz"
    } else {
        if x % 5 == 0 {
            return "Buzz"
        } 
        return ""
    }
}

pub fn fizzbuzz_kronke_inverted(x: u16) -> &'static str {
    if x % 3 != 0 {
        if x % 5 != 0 {
            return ""
        }
        return "Buzz"
    } else {
        if x % 5 != 0 {
            return "Fizz"
        }
        return "Fizz Buzz"
    }
}

pub fn fizzbuzz_lsitarski(x: u16) -> &'static str {
    if x % 3 == 0 && x % 5 == 0 {
        return "Fizz Buzz"
    }
    if x % 3 == 0 {
        return "Fizz"
    }
    if x % 5 == 0 {
        return "Buzz"
    }
    return ""
}

const MASK15: [usize; 15] = [3,0,0,1,0,2,1,0,0,1,2,0,1,0,0];
const STR: [&str; 4] = [ "", "Fizz","Buzz", "Fizz Buzz" ];

pub fn fizzbuzz_kronke_branchless(x: usize) -> &'static str {
    return STR[MASK15[x%15]]
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::{fizzbuzz_kronke_branchless, fizzbuzz_kronke};

    #[test]
    fn fizzbuzz_kronke_branchless_test() {
        let mut rng = rand::thread_rng();
        assert_eq!(fizzbuzz_kronke_branchless(1), "");
        assert_eq!(fizzbuzz_kronke_branchless(3), "Fizz");
        assert_eq!(fizzbuzz_kronke_branchless(5), "Buzz");
        assert_eq!(fizzbuzz_kronke_branchless(15), "Fizz Buzz");
        for _ in 0..500 {
            let val: usize = rng.gen_range(0..(u16::MAX as usize));
            println!("{}", val);
            assert_eq!(fizzbuzz_kronke_branchless(val), fizzbuzz_kronke(val as u16));
        }
    }
}