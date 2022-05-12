// cons cell的なものをイメージ
// Optionは入れない
// 循環はしない
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct List<T> (Option<Rc<Cons<T>>>);

#[derive(Debug, PartialEq, Eq)]
struct Cons<T> (T, List<T>);
impl<T> Cons<T> {
    fn new(x:T) -> Self {
        Self(x, List(None))
    }
}
impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T> List<T> {
    pub fn new(x:T) -> Self {
        Self(Some(Rc::new(Cons::new(x))))
    }
    pub fn head(&self) -> Option<&T> {
        match &self.0 {
            Some(v) => Some(&v.0),
            None => None
        }
    }
    pub fn tail(self) -> Option<List<T>> {
        match self.0 {
            Some(v) => Some(v.1.clone()),
            None => None
        }
    }
    pub fn cons(self, x:T) -> Self {
        Self(Some(Rc::new(Cons(x, self.clone()))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let l = List::new(1);
        assert_eq!(l, List(Some(Rc::new(Cons::new(1)))))
    }
    #[test]
    fn test_head() {
        let l = List::new(1);
        let h = l.head().unwrap();
        assert_eq!(h, &1)
    }
    #[test]
    fn test_cons() {
        let l = List::new(0);
        let l = l.cons(1);
        assert_eq!(l, List(Some(Rc::new(Cons(1, List::new(0))))));
    }
    #[test]
    fn test_tail() {
        let l = List::new(0);
        let l = l.cons(1);
        let l = l.tail().unwrap();
        assert_eq!(l, List::new(0))
    }
}
