use std::slice::Iter as SliceIter;

#[derive(Debug)]
pub struct Iter<'a, TIntersect>(pub(super) SliceIter<'a, TIntersect>);

impl<'a, TIntersect> ExactSizeIterator for Iter<'a, TIntersect> where TIntersect: 'a {}

impl<'a, TIntersect> Iterator for Iter<'a, TIntersect>
where
    TIntersect: 'a,
{
    type Item = &'a TIntersect;

    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
