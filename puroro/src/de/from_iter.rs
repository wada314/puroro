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
        match self.end_stack.last() {
            Some(end) => {
                if self.pos < *end {
                    self.pos += 1;
                    self.iter.next()
                } else {
                    None
                }
            }
            None => {
                self.pos += 1;
                self.iter.next()
            }
        }
    }
}

#[test]
fn test_scoped_iter() {
    let s1 = ScopedIter::new("abcdefg".chars());
    assert_eq!("abcdefg", s1.collect::<String>());

    let s2 = ScopedIter::new_with_len("abcdefg".chars(), 5);
    assert_eq!("abcde", s2.collect::<String>());

    let mut s3 = ScopedIter::new("abcdefg".chars());
    s3.push_scope(5);
    assert_eq!("abcde", s3.collect::<String>());

    let mut s4 = ScopedIter::new("abcdefg".chars());
    assert_eq!(Some('a'), s4.next());
    s4.push_scope(5);
    assert_eq!("bcdef", s4.by_ref().collect::<String>());
    s4.pop_scope();
    assert_eq!("g", s4.by_ref().collect::<String>());
}
