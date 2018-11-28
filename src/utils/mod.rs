/// This module implements a sample circal number, supporting
/// add,sub and a sample contructing function
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CirNum<T> {
    max: T,
    min: T,
    value: T,
}

impl CirNum<usize> {
    /// constuctor fir circal number, user must make sure max bigger then
    /// min, and init value between [min,max]
    pub fn new(min: usize, max: usize, value: usize) -> CirNum<usize> {
        if max - min < 1 || value < min || value > max {
            panic!("min >= max?")
        }
        CirNum { max, min, value }
    }
    pub fn add(&mut self, step: usize) -> usize {
        // it's not reasonable, if step is bigger than the diff between
        // max and min, add function should handle this situation
        // if step > self.max - self.min {
        //     panic!("step > max - min")
        // }
        let step = step % (self.max - self.min);
        if self.value + step >= self.max {
            self.value = self.min + ((self.value + step) - self.max);
        } else {
            self.value += step;
        };
        self.value
    }

    pub fn sub(&mut self, step: usize) -> usize {
        let step = step % (self.max - self.min);
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
    fn circal_num_add_sub_correctness() {
        //let file_name = env::args().skip(1).next().unwrap();
        let mut a = CirNum::new(0, 10, 0);
        // fufill basic function
        assert_eq!(2, a.add(2));
        assert_eq!(0, a.add(8));
        // error first time
        assert_eq!(0, a.add(20));
        assert_eq!(2, a.add(42));
        a.sub(2);
        assert_eq!(9, a.sub(1));
        assert_eq!(0, a.sub(9));
        assert_eq!(0, a.sub(20));
    }

    // #[test]
    // #[should_panic]
    // fn constuctor_correctness() {
    //     let tmp = CirNum::new(0, 1, 0);
    //     let tmp2 = CirNum::new(15, 2, 10);
    //     let tmp3 = CirNum::new(10, 20, 30);
    // }
}
