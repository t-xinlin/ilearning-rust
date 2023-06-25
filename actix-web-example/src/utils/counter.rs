pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}


