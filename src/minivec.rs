#[derive(Debug)]
pub enum MiniVec<T> {
    Empty,
    One(T),
    Two(T, T),
    Many(Vec<T>),
}

impl<T> MiniVec<T>
where
    T: Copy,
{
    pub fn push(&mut self, t: T) {
        match *self {
            MiniVec::Empty => *self = MiniVec::One(t),
            MiniVec::One(first) => *self = MiniVec::Two(first, t),
            MiniVec::Two(first, second) => *self = MiniVec::Many(vec![first, second, t]),
            MiniVec::Many(ref mut many) => many.push(t),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match *self {
            MiniVec::Empty => None,
            MiniVec::One(first) => {
                *self = MiniVec::Empty;
                Some(first)
            }
            MiniVec::Two(first, second) => {
                *self = MiniVec::One(first);
                Some(second)
            }
            MiniVec::Many(ref mut many) => many.pop(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            MiniVec::Empty => true,
            MiniVec::Many(many) => many.is_empty(),
            _ => false,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            MiniVec::Empty => 0,
            MiniVec::One(_) => 1,
            MiniVec::Two(_, _) => 2,
            MiniVec::Many(many) => many.len(),
        }
    }

    pub fn visit<V>(&self, visitor: V)
    where
        V: Fn(&T) -> (),
    {
        match self {
            MiniVec::Empty => (),
            MiniVec::One(first) => visitor(first),
            MiniVec::Two(first, second) => {
                visitor(first);
                visitor(second);
            }
            MiniVec::Many(many) => {
                for item in many {
                    visitor(item)
                }
            }
        }
    }
}

impl<T> Default for MiniVec<T> {
    fn default() -> Self {
        MiniVec::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_push() {
        let mut v: MiniVec<u32> = Default::default();

        assert_that(&v.len()).is_equal_to(0);
        for i in 1..10 {
            v.push(i);
            assert_that(&v.len()).is_equal_to(i as usize);
        }
    }

    #[test]
    fn test_pop() {
        let mut v: MiniVec<u32> = Default::default();

        v.push(10);
        v.push(20);
        assert_that(&v.pop()).contains_value(20);
        assert_that(&v.pop()).contains_value(10);
        assert_that(&v.pop()).is_none();
    }
}
