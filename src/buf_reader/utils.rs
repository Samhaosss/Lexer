#[derive(Debug, PartialEq)]
pub struct CirNum<T> {
    max: T,
    min: T,
    value: T,
}

impl CirNum<usize> {
    pub fn new(min: usize, max: usize, value: usize) -> CirNum<usize> {
        if min >= max {
            panic!("min >= max?")
        }
        CirNum { max, min, value }
    }
    pub fn add(&mut self, step: usize) -> usize {
        if step > self.max - self.min {
            panic!("step > max - min")
        }
        if self.value + step >= self.max {
            self.value = self.min + ((self.value + step) - self.max);
        } else {
            self.value += step;
        };
        self.value
    }
    pub fn sub(&mut self, step: usize) -> usize {
        if step > self.max - self.min {
            panic!("step > max - min")
        }
        if step > (self.value - self.min) {
            self.value = self.max - (step - (self.value - self.min));
        } else {
            self.value -= step;
        }
        self.value
    }
    pub fn add_1(&mut self) -> usize {
        self.add(1usize)
    }
    pub fn sub_1(&mut self) -> usize {
        self.sub(1)
    }
    pub fn get_value(&self) -> usize {
        self.value
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn read_word() {
        //let file_name = env::args().skip(1).next().unwrap();
        let mut a = CirNum::new(0, 100, 0);
        for i in &[99, 2, 100] {
            a.add(*i);
            println!("{}", a.get_value());
        }
        for i in &[100, 2, 99] {
            a.sub(*i);
            println!("{}", a.get_value());
        }
        assert_eq!(0, a.get_value());
    }
}
