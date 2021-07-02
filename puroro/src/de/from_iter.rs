struct ScopedIter<I> {
    iter: I,
    pos: usize,
    end_stack: Vec<usize>,
}
impl<I> ScopedIter<I> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            pos: 0,
            end_stack: Vec::new(),
        }
    }
    fn new_with_len(iter: I, len: usize) -> Self {
        Self {
            iter,
            pos: 0,
            end_stack: vec![len],
        }
    }
    fn push_scope(&mut self, new_len: usize) {
        if let Some(cur_end) = self.end_stack.last() {
            assert!(self.pos + new_len <= *cur_end);
        }
        self.end_stack.push(self.pos + new_len);
    }
    fn pop_scope(&mut self) {
        if let Some(cur_end) = self.end_stack.last() {
            assert_eq!(self.pos, *cur_end);
        }
        self.end_stack.pop().unwrap();
    }
}
impl<I> Iterator for ScopedIter<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let retval = match self.end_stack.last() {
            Some(end) => {
                if self.pos < *end {
                    self.iter.next()
                } else {
                    None
                }
            }
            None => self.iter.next(),
        };
        self.pos += 1;
        retval
    }
}

#[test]
fn test_scoped_iter() {}
