pub mod algorithm;
pub mod leetcode;
pub mod structure;

use std::ops::Drop;

#[derive(Debug, Default, Clone)]
pub struct People;

impl Drop for People {
    fn drop(&mut self) {
        println!("I have been dropped!");
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::People;

    #[test]
    fn test_hello() {
        {
            let a = Box::new(People::default());
            let b = a.clone();

            println!("{:?}", a);
            println!("{:?}", b);
        }

        for _ in 0..5 {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
