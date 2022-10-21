// use std::marker::PhantomData;

// pub struct CircularBuffer<T> {
//     // This field is here to make the template compile and not to
//     // complain about unused type parameter 'T'. Once you start
//     // solving the exercise, delete this field and the 'std::marker::PhantomData'
//     // import.
//     field: PhantomData<T>,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum Error {
//     EmptyBuffer,
//     FullBuffer,
// }

// impl<T> CircularBuffer<T> {
//     pub fn new(capacity: usize) -> Self {
//         unimplemented!(
//             "Construct a new CircularBuffer with the capacity to hold {}.",
//             match capacity {
//                 1 => "1 element".to_string(),
//                 _ => format!("{} elements", capacity),
//             }
//         );
//     }

//     pub fn write(&mut self, _element: T) -> Result<(), Error> {
//         unimplemented!("Write the passed element to the CircularBuffer or return FullBuffer error if CircularBuffer is full.");
//     }

//     pub fn read(&mut self) -> Result<T, Error> {
//         unimplemented!("Read the oldest element from the CircularBuffer or return EmptyBuffer error if CircularBuffer is empty.");
//     }

//     pub fn clear(&mut self) {
//         unimplemented!("Clear the CircularBuffer.");
//     }

//     pub fn overwrite(&mut self, _element: T) {
//         unimplemented!("Write the passed element to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.");
//     }
// }

pub struct CircularBuffer<T> {
    vals: Vec<T>,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
where
    T: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            vals: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn is_full(&self) -> bool {
        self.vals.len() == self.capacity
    }

    fn write_unchecked(&mut self, element: T) {
        self.vals.push(element);
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        (!self.is_full())
            .then(|| self.write_unchecked(element))
            .ok_or(Error::FullBuffer)
    }

    pub fn read(&mut self) -> Result<T, Error> {
        (!self.vals.is_empty())
            .then(|| self.vals.remove(0))
            .ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.vals.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            self.vals.remove(0);
        }

        self.write_unchecked(element);
    }
}
