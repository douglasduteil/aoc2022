//

use std::{
    convert::Infallible,
    iter::StepBy,
    ops::{Index, Range},
    slice::Iter,
    str::FromStr,
};

//

#[derive(Debug, Clone, PartialEq)]
pub struct UGrid {
    /// The number of rows.
    pub rows: usize,
    /// The number of columns.
    pub columns: usize,
    /// The values stored in the column-major order.
    pub values: Vec<usize>,
}

impl FromStr for UGrid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split_terminator(&['\n', ' '][..])
            .map(String::from)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let len_row = data.len();
        let len_col = data[0].len();

        let data: Vec<usize> = data
            .iter()
            .flat_map(|c| {
                c.chars()
                    .flat_map(|c| c.to_digit(10))
                    .map(|value| value as usize)
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(UGrid {
            values: data,
            rows: len_row,
            columns: len_col,
        })
    }
}

impl UGrid {
    /// NOTE(douglas): I made it for fun (:
    ///
    #[allow(dead_code)]
    pub fn sub_grid_iter(
        &self,
        row_range: Range<usize>,
        col_range: Range<usize>,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let is_in_range = move |i: usize| {
            row_range.contains(&(i % self.rows)) && col_range.contains(&(i / self.columns))
        };

        Box::new(
            self.values
                .iter()
                .enumerate()
                .filter_map(move |(index, value)| match index {
                    i if is_in_range(i) => Some(*value),
                    _ => None,
                }),
        )
    }

    pub fn index_to_coord(&self, index: usize) -> (usize, usize) {
        let max = self.values.len() - 1;
        let UGrid { columns, rows, .. } = self;
        if index > max {
            panic!("\n /!\\ : index {index} out of grid range {rows}x{columns} =index= {max}max\n");
        }
        (index % self.columns, index / self.columns)
    }

    /// NOTE(douglas): I made it for fun (:
    ///
    #[allow(dead_code)]
    pub fn coord(&self) -> Vec<((usize, usize), &usize)> {
        self.values
            .iter()
            .enumerate()
            .map(|(index, value)| (self.index_to_coord(index), value))
            .collect::<Vec<_>>()
    }

    pub fn iter_row(&self, index: usize) -> Iter<usize> {
        let start = index * self.columns;
        let end = start + self.columns;

        self.values[start..end].iter()
    }

    /// NOTE(douglas): I made it for fun (:
    ///
    #[allow(dead_code)]
    pub fn iter_rows(&self) -> RowIter<'_> {
        RowIter {
            grid: self,
            index: 0,
        }
    }

    pub fn iter_col(&self, index: usize) -> StepBy<Iter<usize>> {
        self.values[index..].iter().step_by(self.columns)
    }

    /// NOTE(douglas): I made it for fun (:
    ///
    #[allow(dead_code)]
    pub fn iter_columns(&self) -> ColumnIter<'_> {
        ColumnIter {
            grid: self,
            index: 0,
        }
    }

    /// NOTE(douglas): I made it for fun (:
    ///
    #[allow(dead_code)]
    pub fn edge(&self) -> Vec<&usize> {
        let last_column_index = self.columns - 1;

        let first_row = self.iter_row(0);
        let last_row = self.iter_row(self.rows - 1);

        let middle = |(index, val)| match index {
            0 => None,
            i if i == last_column_index => None,
            _ => Some(val),
        };

        let first_col = self.iter_col(0).enumerate().filter_map(middle);

        let last_col = self
            .iter_col(last_column_index)
            .enumerate()
            .filter_map(middle);

        let fist_last_col = first_col
            .zip(last_col)
            .flat_map(|(first, last)| [first, last]);

        let mut edge = Vec::with_capacity(self.rows * 2 + self.columns * 2 - 4);

        edge.extend(first_row);
        edge.extend(fist_last_col);
        edge.extend(last_row);

        edge
    }

    /// Retrieve the index from a given (x,y) positio
    pub fn coord_to_index(&self, x: usize, y: usize) -> usize {
        y * self.columns + x
    }
}

/// Performs the indexing (`grid[(x, y)]`) operation.
///
impl Index<(usize, usize)> for UGrid {
    type Output = usize;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        unsafe { self.values.get_unchecked(self.coord_to_index(x, y)) }
    }
}

//

/// Row iterator
pub struct RowIter<'a> {
    grid: &'a UGrid,
    index: usize,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Iter<'a, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let RowIter { grid, index } = self;

        if !(0..grid.rows).contains(index) {
            return None;
        }

        let iter = grid.iter_row(*index);
        self.index += 1;
        Some(iter)
    }
}

//

/// Column iterator
pub struct ColumnIter<'a> {
    grid: &'a UGrid,
    index: usize,
}

impl<'a> Iterator for ColumnIter<'a> {
    type Item = StepBy<Iter<'a, usize>>;

    fn next(&mut self) -> Option<Self::Item> {
        let ColumnIter { grid, index } = self;

        if !(0..grid.columns).contains(index) {
            return None;
        }

        let iter = grid.iter_col(*index);
        self.index += 1;
        Some(iter)
    }
}

//

#[cfg(test)]
mod test_u_grid {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_index_to_coord() {
        let grid: UGrid = "01\n23\n45".parse().unwrap();
        assert_eq!(grid.index_to_coord(0), (0, 0));
        assert_eq!(grid.index_to_coord(1), (1, 0));
        assert_eq!(grid.index_to_coord(2), (0, 1));
        assert_eq!(grid.index_to_coord(3), (1, 1));
        assert_eq!(grid.index_to_coord(4), (0, 2));
        assert_eq!(grid.index_to_coord(5), (1, 2));
    }

    #[test]
    fn test_coord_to_index() {
        let grid: UGrid = "01\n23\n45".parse().unwrap();
        assert_eq!(grid.coord_to_index(0, 0), 0);
        assert_eq!(grid.coord_to_index(1, 0), 1);
        assert_eq!(grid.coord_to_index(0, 1), 2);
        assert_eq!(grid.coord_to_index(1, 1), 3);
        assert_eq!(grid.coord_to_index(0, 2), 4);
        assert_eq!(grid.coord_to_index(1, 2), 5);
    }

    #[test]
    fn test_iter_row() {
        let grid: UGrid = "12\n34\n56".parse().unwrap();

        let mut iter = grid.iter_row(0);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);

        let mut iter = grid.iter_row(1);
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);

        let mut iter = grid.iter_row(2);
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_rows() {
        let grid: UGrid = "12\n34\n56".parse().unwrap();

        let max_by_row: Vec<usize> = grid
            .iter_rows()
            .flat_map(|row| row.max())
            .map(|item| *item)
            .collect();
        assert_eq!(max_by_row, vec![2, 4, 6]);

        let sum_by_row: Vec<usize> = grid.iter_rows().map(|row| row.sum()).collect();
        assert_eq!(sum_by_row, vec![1 + 2, 3 + 4, 5 + 6])
    }

    #[test]
    fn test_iter_col() {
        let grid: UGrid = "12\n34\n56".parse().unwrap();

        let mut iter = grid.iter_col(0);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);

        let mut iter = grid.iter_col(1);
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_cols() {
        let grid: UGrid = "12\n34\n56".parse().unwrap();

        let max_by_row: Vec<usize> = grid
            .iter_columns()
            .flat_map(|row| row.max())
            .map(|item| *item)
            .collect();
        assert_eq!(max_by_row, vec![5, 6]);

        let sum_by_row: Vec<usize> = grid.iter_columns().map(|row| row.sum()).collect();
        assert_eq!(sum_by_row, vec![1 + 3 + 5, 2 + 4 + 6])
    }

    #[test]
    fn test_edge() {
        let grid: UGrid = "123\n456\n789".parse().unwrap();

        assert_eq!(
            grid.edge(),
            vec![
                &1usize, &2, &3, //
                &4, /* 5, */ &6, //
                &7, &8, &9,
            ]
        );
    }

    #[test]
    fn test_subgrid_center_1x1() {
        let grid: UGrid = "123\n456\n789".parse().unwrap();
        assert_eq!(grid.sub_grid_iter(1..2, 1..2).collect_vec(), vec![5usize]);
    }

    #[test]
    fn test_subgrid_center_2x2() {
        let grid: UGrid = "1111\n1221\n1221\n1111".parse().unwrap();
        assert_eq!(
            grid.sub_grid_iter(1..3, 1..3).collect_vec(),
            vec![2usize, 2, 2, 2]
        );
    }
}
