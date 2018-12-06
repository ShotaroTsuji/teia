#[derive(Debug)]
pub struct Heap<T> {
    data: Vec<T>,
}

#[derive(Debug,Clone,Copy)]
struct Position(usize);

impl Position {
    fn root() -> Position {
        Position(1)
    }

    fn parent(&self) -> Position {
        Position(self.0 / 2)
    }

    fn left(&self) -> Position {
        Position(self.0 * 2)
    }

    fn right(&self) -> Position {
        Position(self.0 * 2 + 1)
    }

    fn to_index(&self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0-1)
        }
    }
}

impl<T: Ord> Heap<T> where
T: ::std::fmt::Debug
{
    pub fn new() -> Heap<T> {
        Heap {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);

        let mut pos = Position(self.data.len());

        while !self.is_valid(pos) {
            unsafe {
                let p1: *mut T = &mut self.data[pos.to_index().unwrap()];
                let p2: *mut T = &mut self.data[pos.parent().to_index().unwrap()];
                p1.swap(p2);
            }
            pos = pos.parent();
        }
    }

    fn get(&self, pos: Position) -> Option<&T> {
        pos.to_index().and_then(|index| self.data.get(index))
    }

    fn is_valid(&self, pos: Position) -> bool {
        let value = self.get(pos);
        let parent = self.get(pos.parent());
        match (parent, value) {
            (None, _) => true,
            (Some(p), Some(v)) if p > v => true,
            (Some(p), Some(v)) if p < v => false,
            _ => panic!("Unknown case"),
        }
    }
}
