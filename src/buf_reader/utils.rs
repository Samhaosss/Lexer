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
    pub fn add_1(&mut self) -> usize {
        self.add(1usize)
    }
    pub fn get_value(&self) -> usize {
        self.value
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }
}


