use std::ops::{Index, IndexMut};

// TODO: Once generic_const_exprs is in stable, we can use that to compute C from COLUMNS * ROWS
pub struct Grid<
    T,
    const C: usize,
    const CELL_WIDTH: usize,
    const CELL_HEIGHT: usize,
    const COLUMNS: usize,
    const ROWS: usize,
> {
    pub inner: [T; C],
}

impl<
    T,
    const C: usize,
    const CELL_WIDTH: usize,
    const CELL_HEIGHT: usize,
    const COLUMNS: usize,
    const ROWS: usize,
> Grid<T, C, CELL_WIDTH, CELL_HEIGHT, COLUMNS, ROWS>
{
    pub fn get_cell_at_index_mut(&mut self, index_x: usize, index_y: usize) -> &mut T {
        &mut self.inner[index_x + (COLUMNS * index_y)]
    }

    pub fn get_cell_at_index(&self, index_x: usize, index_y: usize) -> &T {
        &self.inner[index_x + (COLUMNS * index_y)]
    }

    pub fn get_cell_at_position_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index_x = x / CELL_WIDTH;
        let index_y = y / CELL_HEIGHT;
        &mut self.inner[index_x + (COLUMNS * index_y)]
    }

    pub fn get_cell_at_position(&self, x: usize, y: usize) -> &T {
        let index_x = x / CELL_WIDTH;
        let index_y = y / CELL_HEIGHT;
        &self.inner[index_x + (COLUMNS * index_y)]
    }
}

impl<
    'a,
    T,
    const C: usize,
    const CELL_WIDTH: usize,
    const CELL_HEIGHT: usize,
    const WIDTH: usize,
    const HEIGHT: usize,
> IntoIterator for &'a Grid<T, C, CELL_WIDTH, CELL_HEIGHT, WIDTH, HEIGHT>
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

impl<
    T,
    const C: usize,
    const CELL_WIDTH: usize,
    const CELL_HEIGHT: usize,
    const WIDTH: usize,
    const HEIGHT: usize,
> Index<usize> for Grid<T, C, CELL_WIDTH, CELL_HEIGHT, WIDTH, HEIGHT>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<
    T,
    const C: usize,
    const CELL_WIDTH: usize,
    const CELL_HEIGHT: usize,
    const WIDTH: usize,
    const HEIGHT: usize,
> IndexMut<usize> for Grid<T, C, CELL_WIDTH, CELL_HEIGHT, WIDTH, HEIGHT>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
