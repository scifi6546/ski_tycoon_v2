#[allow(dead_code)]
pub struct CircularBuffer<T> {
    pub data: Vec<T>,
    current_index: usize,
}
#[allow(dead_code)]
impl<T> CircularBuffer<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            current_index: 0,
        }
    }
    pub fn get_mut(&mut self) -> &mut T {
        let len = self.data.len();
        &mut self.data[self.current_index % len]
    }
    pub fn next(&mut self) {
        self.current_index += 1;
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
    pub fn drain(self) -> Vec<T> {
        self.data
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn CircularBuffer() {
        let mut buff = CircularBuffer::new(vec![0usize, 1usize, 2usize]);
        for i in 0..3 {
            assert_eq!(buff.get_mut(), &i);
            buff.next();
        }
    }
}
