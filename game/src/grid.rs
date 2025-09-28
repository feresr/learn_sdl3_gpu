use std::ops::Index;

pub struct Grid<T, const C: usize, const TILE_SIZE: usize, const WIDTH: usize, const HEIGHT: usize>
{
    pub inner: [T; C],
}

impl<T, const C: usize, const TILE_SIZE: usize, const WIDTH: usize, const HEIGHT: usize>
    Grid<T, C, TILE_SIZE, WIDTH, HEIGHT>
{
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.inner[x + (WIDTH * y)]
    }

    pub fn get_tile(&self, index_x: usize, index_y: usize) -> &T {
        &self.inner[index_x + (HEIGHT * index_y)]
    }
}

impl<'a, T, const C: usize, const TILE_SIZE: usize, const WIDTH: usize, const HEIGHT: usize>
    IntoIterator for &'a Grid<T, C, TILE_SIZE, WIDTH, HEIGHT>
{
    // into iterator is so that for loops work
    type Item = (usize, usize, &'a T);
    type IntoIter = GridIterator<'a, T, C, WIDTH>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            cells: &self.inner,
            index: 0,
        }
    }
}

pub struct GridIterator<'a, T, const C: usize, const W: usize> {
    cells: &'a [T; C],
    index: usize,
}

impl<'a, T, const C: usize, const WIDTH: usize> Iterator for GridIterator<'a, T, C, WIDTH> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= C {
            return None;
        }

        let current_index = self.index;
        self.index += 1;

        let x = current_index % WIDTH as usize;
        let y = current_index / WIDTH as usize;
        let cell = &self.cells[current_index];

        Some((x, y, cell))
    }
}

impl<T, const C: usize, const TILE_SIZE: usize, const WIDTH: usize, const HEIGHT: usize> 
    Index<usize> for Grid<T, C, TILE_SIZE, WIDTH, HEIGHT> 
{
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}