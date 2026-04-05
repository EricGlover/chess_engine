use super::Square;



// pub trait SquareIterator {
//     fn get_next(&mut self) ->Option<&Square>;
// }

// impl Iterator for SquareIterator {
//     type Item = &Square;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.get_next()
//     }
// }



// pub struct BoardFileIterator<'a> {
//     x: usize,
//     y: usize,
//     squares: &'a [[Square; 8]; 8],
// }
// impl<'a> BoardFileIterator<'a> {
//     pub fn new(x: usize, y: usize, squares: &'a [[Square; 8]; 8]) -> BoardFileIterator {
//         BoardFileIterator { x, y, squares }
//     }
// }
// impl<'a> SquareIterator for BoardFileIterator<'a> {
//     fn get_next(&mut self) -> Option<&Square> {
//         // y is past top of board then return
//         if self.y > self.squares.len() - 1 {
//             return None;
//         }
//         let square = self.squares[self.y].get(self.x);
//         self.y = self.y + 1;
//         return square;
//     }
// }

// // iterates a1 -> a8, b1 -> b8, etc..
// pub struct BoardFilesIterator<'a> {
//     x: usize,
//     y: usize,
//     squares: &'a [[Square; 8]; 8],
// }
// impl<'a> BoardFilesIterator<'a> {
//     pub fn new(squares: &'a [[Square; 8]; 8]) -> BoardFilesIterator {
//         BoardFilesIterator {
//             x: 0,
//             y: 0,
//             squares,
//         }
//     }
// }
// impl Iterator for BoardFilesIterator {
//     type Item = Box<dyn SquareIterator>;
//     fn next(&mut self) -> Option<Self::Item> {
//         // if past the right of the board then return
//         if self.x > self.squares[0].len() - 1 {
//             return None;
//         }
//         let file = Some(Box::new(BoardFileIterator::new(self.x, self.y, self.squares)));
//         self.x = self.x + 1;
//         return file;
//     }
// }

// pub struct BoardRankIterator<'a> {
//     x: usize,
//     y: usize,
//     squares: &'a [[Square; 8]; 8],
// }
// impl<'a> BoardRankIterator<'a> {
//     pub fn new(rank: usize, squares: &'a [[Square; 8]; 8]) -> BoardRankIterator {
//         if rank < 0 || rank > 7 {
//             panic!("can not iterate off the board");
//         }
//         BoardRankIterator {
//             x: 0,
//             y: rank,
//             squares,
//         }
//     }
// }

// impl<'a> Iterator for BoardRankIterator<'a> {
//     type Item = &'a Square;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.x > self.squares[self.y].len() - 1 {
//             return None;
//         }
//         let square = self.squares[self.y].get(self.x);
//         self.x = self.x + 1usize;
//         return square;
//     }
// }

// pub struct BoardSquareIterator<'a> {
//     i: usize,
//     j: usize,
//     squares: &'a [[Square; 8]; 8],
// }
// impl<'a> BoardSquareIterator<'a> {
//     pub fn new(squares: &'a [[Square; 8]; 8]) -> BoardSquareIterator {
//         BoardSquareIterator {
//             i: 0,
//             j: 0,
//             squares,
//         }
//     }
// }

// impl<'a> Iterator for BoardSquareIterator<'a> {
//     type Item = &'a Square;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.j > self.squares[self.i].len() - 1 {
//             self.i = self.i + 1;
//             self.j = 0;
//         }
//         if self.i > self.squares.len() - 1 {
//             return None;
//         }
//         let square = self.squares[self.i].get(self.j);
//         self.j = self.j + 1usize;
//         return square;
//     }
// }
