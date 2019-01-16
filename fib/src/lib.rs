//! Popuplation counts

use std::ops::Deref;

// Represents a FIFO queue
#[derive(Debug)]
struct Queue<T: Copy> {
    queue: Vec<T>,
    size: usize,
    initialize_to: T,
}

impl<T: Copy> Queue<T> {
    fn new(size: usize, initialize_to: T) -> Queue<T> {
        let mut queue = Vec::with_capacity(size);

        for _ in 0..size {
            queue.push(initialize_to);
        }

        Queue {
            queue,
            size,
            initialize_to,
        }
    }

    fn from_vec(vec: Vec<T>, initialize_to: T) -> Queue<T> {
        let size = vec.len();
        Queue {
            queue: vec.to_vec(),
            size,
            initialize_to,
        }
    }

    // Add the specified element to the 'back' of the queue (front of the
    // vector).
    // Since the queue size is fixed, this will also remove the element at
    // the 'front' of the queue (last element in the vector)
    fn push(&mut self, val: T) -> () {
        self.queue.insert(0usize, val);
        self.queue.pop();
    }
}

impl<T: Copy> Deref for Queue<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.queue
    }
}

#[derive(Debug)]
pub struct Population {
    // Recent population counts used in the recurrence relation
    counts: Queue<usize>,
    // Number of children pairs per parent pair
    litter: usize,
    // Expected life of a pair
    life_expectancy: Option<usize>,
    index: usize,
}

// Implement `Iterator` for `Population`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Population {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_next: usize;

        if let Some(life_expectancy) = self.life_expectancy {
            if self.index < life_expectancy - 1 {
                new_next = (*self.counts)[0] + (*self.counts)[1] * self.litter;
            } else if self.index < life_expectancy {
                new_next = (*self.counts)[0] + (*self.counts)[1] * self.litter - 1;
            } else {
                new_next = (*self.counts)[0] + (*self.counts)[1] * self.litter
                    - (*self.counts).last().unwrap();
            }
        } else {
            new_next = (*self.counts)[0] + (*self.counts)[1] * self.litter;
        }
        self.counts.push(new_next);
        self.index = self.index + 1;
        Some(self.counts[1])
    }
}

/// Creates a population iterator
pub fn population(litter: usize) -> Population {
    Population {
        counts: Queue::from_vec(vec![1usize, 0], 0usize),
        life_expectancy: None,
        litter,
        index: 0usize,
    }
}

/// Creates a population iterator for a population whose members have a
/// specified life expectancy
pub fn population_with_moratilty(litter: usize, life_expectancy: usize) -> Population {
    let mut counts = Queue::new(life_expectancy + 1, 0usize);
    counts.push(1usize);
    Population {
        counts: counts,
        life_expectancy: Some(life_expectancy),
        litter,
        index: 0usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_a_queue() {
        let q = Queue::new(5usize, 0isize);
        assert_eq!(*q, vec![0isize, 0, 0, 0, 0]);
    }

    #[test]
    fn it_can_add_to_a_queue() {
        let mut q = Queue::new(3usize, 0isize);
        q.push(7isize);
        assert_eq!(*q, vec![7isize, 0, 0]);
    }
}
