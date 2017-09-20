use std::vec;

pub trait StreamMap<T, U> {
    fn produce(&mut self, T) -> Vec<U>;
}

pub struct StreamAdapter<M, I, T> {
    map: M,
    source: I,
    buffer: vec::IntoIter<T>,
}

impl<M, I, T> StreamAdapter<M, I, T> {
    pub fn new(map: M, iter: I) -> Self {
        StreamAdapter {
            map: map,
            source: iter,
            buffer: Vec::new().into_iter(),
        }
    }
}

impl<M, I, T, U> Iterator for StreamAdapter<M, I, U>
    where I: Iterator<Item=T>, M: StreamMap<T, U> {
    type Item = U;

    fn next(&mut self) -> Option<U> {
        match self.buffer.next() {
            s@Some(_) => s,
            None => match self.source.next() {
                Some(x) => {
                    self.buffer = self.map.produce(x).into_iter();
                    self.next()
                }
                None => None
            }
        }
    }
}
