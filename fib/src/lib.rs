//! Popuplation counts

#[derive(Debug)]
pub struct Population {
    curr: u64,
    next: u64,
    litter: u64,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Population {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr * self.litter + self.next;

        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

pub fn population(litter: u64) -> Population {
    Population {
        curr: 0,
        next: 1,
        litter,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
