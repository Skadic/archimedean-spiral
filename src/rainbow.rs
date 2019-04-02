pub struct RainbowIterator {
    x: f32
}

impl RainbowIterator {
    pub fn new() -> RainbowIterator {
        RainbowIterator {
            x: 0.0,
        }
    }
}

impl Iterator for RainbowIterator {
    type Item = [f32; 4];


    fn next(&mut self) -> Option<Self::Item> {
        use std::f32::consts;
        let green = |x: f32| {
            let n = x % (1.5 * consts::PI);
            if n > consts::PI {
                0.0
            } else {
                n.sin().powi(2)
            }
        };

        let red = |x: f32| green(x - consts::PI / 2.0);
        let blue = |x: f32| green(x - consts::PI);

        self.x += 0.1;
        Some([red(self.x), green(self.x), blue(self.x), 1.0])
    }
}

#[cfg(test)]
mod tests {
    use crate::rainbow::RainbowIterator;

    #[test]
    fn test_rainbow_iterator() {
        let mut rainbow = RainbowIterator::new();

        for _ in 0..62 {
            println!("{:?}", rainbow.next().unwrap());
        }
    }
}