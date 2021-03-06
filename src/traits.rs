pub trait List<'a, T> {
    fn new() -> Self;
    fn length(&self) -> usize;
    fn get(&'a self, ind: usize) -> &'a T;
    fn get_mut(&'a mut self, ind: usize) -> &'a mut T;
    fn insert_at(&mut self, index: usize, t: T) -> Result<(), T>;
    fn put_first(&mut self, t: T) -> &mut Self;
    fn put_last(&mut self, t: T) -> &mut Self;
    fn remove_first(&mut self) -> Option<T>;
    fn remove_last(&mut self) -> Option<T>;
    fn remove_nth(&mut self, index: usize) -> Option<T>;
    fn remove_first_matching<F: Fn(&T) -> bool>(&mut self, f: F) -> Option<T>;
    fn replace_nth(&mut self, index: usize, t: T) -> Result<T, T>;
}
