pub mod algorithm;
pub mod leetcode;
pub mod structure;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        use rand::prelude::*;
        println!("{:?}", thread_rng().gen_range(0..=1));
    }
}
