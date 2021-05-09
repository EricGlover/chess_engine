use super::Square;

pub struct FileIterator<'a> {
    x: usize,
    y: usize,
    squares: &'a [[Square; 8]; 8],
}
impl<'a> FileIterator<'a> {
    pub fn new(x: usize, y: usize, squares: &'a [[Square; 8]; 8]) -> FileIterator {
        FileIterator { x, y, squares }
    }
}
impl<'a> Iterator for FileIterator<'a> {
    type Item = &'a Square;
    fn next(&mut self) -> Option<Self::Item> {
        // y is past top of board then return
        if self.y > self.squares.len() - 1 {
            return None;
        }
        let square = self.squares[self.y].get(self.x);
        self.y = self.y + 1;
        return square;
    }
}

// iterates a1 -> a8, b1 -> b8, etc..
pub struct FilesIterator<'a> {
    x: usize,
    y: usize,
    squares: &'a [[Square; 8]; 8],
}
impl<'a> FilesIterator<'a> {
    pub fn new(squares: &'a [[Square; 8]; 8]) -> FilesIterator {
        FilesIterator {
            x: 0,
            y: 0,
            squares,
        }
    }
}
impl<'a> Iterator for FilesIterator<'a> {
    type Item = FileIterator<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        // if past the right of the board then return
        if self.x > self.squares[0].len() - 1 {
            return None;
        }
        let file = Some(FileIterator::new(self.x, self.y, self.squares));
        self.x = self.x + 1;
        return file;
    }
}

pub struct RankIterator<'a> {
    x: usize,
    y: usize,
    squares: &'a [[Square; 8]; 8],
}
impl<'a> RankIterator<'a> {
    pub fn new(rank: usize, squares: &'a [[Square; 8]; 8]) -> RankIterator {
        if rank < 0 || rank > 7 {
            panic!("can not iterate off the board");
        }
        RankIterator {
            x: 0,
            y: rank,
            squares,
        }
    }
}

impl<'a> Iterator for RankIterator<'a> {
    type Item = &'a Square;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.squares[self.y].len() - 1 {
            return None;
        }
        let square = self.squares[self.y].get(self.x);
        self.x = self.x + 1usize;
        return square;
    }
}

pub struct SquareIterator<'a> {
    i: usize,
    j: usize,
    squares: &'a [[Square; 8]; 8],
}
impl<'a> SquareIterator<'a> {
    pub fn new(squares: &'a [[Square; 8]; 8]) -> SquareIterator {
        SquareIterator {
            i: 0,
            j: 0,
            squares,
        }
    }
}

impl<'a> Iterator for SquareIterator<'a> {
    type Item = &'a Square;
    fn next(&mut self) -> Option<Self::Item> {
        if self.j > self.squares[self.i].len() - 1 {
            self.i = self.i + 1;
            self.j = 0;
        }
        if self.i > self.squares.len() - 1 {
            return None;
        }
        let square = self.squares[self.i].get(self.j);
        self.j = self.j + 1usize;
        return square;
    }
}
