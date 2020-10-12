use super::field::Field;
use std::iter;

#[derive(Debug)]
pub struct Grid {
    fields: Vec<Vec<u8>>,
    pub mutable_fields: Vec<Field>,
}

impl std::clone::Clone for Grid {
    fn clone(&self) -> Grid {
        Grid {
            fields: self.fields.clone(),
            mutable_fields: self.mutable_fields.clone(),
        }
    }
}

impl Grid {
    pub fn new(fields: Vec<Vec<u8>>) -> Grid {
        let mut grid = Grid {
            fields,
            mutable_fields: vec![],
        };

        // Calculate mutable fields once and cache fields.
        let mutable_fields = grid.get_mutable_fields();
        grid.mutable_fields = mutable_fields;

        grid
    }

    /// Returns all field indices (row, column) in a parcel.
    pub fn get_parcel_fields(parcel_index: u8) -> Vec<Field> {
        let col_start = (parcel_index % 3) * 3;
        let row_start = (parcel_index / 3) * 3;
        let mut fields: Vec<Field> = vec![];
        for r in 0..3 {
            for c in 0..3 {
                fields.push(Field::new(row_start + r, col_start + c));
            }
        }
        fields
    }

    fn get_mutable_fields(&self) -> Vec<Field> {
        let mut mutable_fields: Vec<Field> = vec![];
        for r in 0..9 {
            for c in 0..9 {
                if self.fields[r as usize][c as usize] == 0 {
                    mutable_fields.push(Field::new(r as u8, c as u8));
                }
            }
        }
        mutable_fields
    }

    pub fn get_parcel_index(field: &Field) -> u8 {
        let x = field.row / 3;
        let y = field.column / 3;
        x * 3 + y
    }

    pub fn get(&self, field: &Field) -> u8 {
        *self
            .fields
            .get(field.row as usize)
            .unwrap()
            .get(field.column as usize)
            .unwrap()
    }

    pub fn set(&mut self, field: &Field, value: u8) {
        self.fields[field.row as usize][field.column as usize] = value;
    }

    pub fn fmt(&self) -> String {
        let mut out = String::new();
        for (i, row) in self.fields.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                out += &iter::repeat("-").take(11).collect::<String>()[..];
                out += "\n";
            }
            for (j, v) in row.iter().enumerate() {
                if j > 0 && j % 3 == 0 {
                    out += "|";
                }
                if v == &0 {
                    out += "x";
                } else {
                    let val = format!("{}", v);
                    out += &val[..];
                }
            }
            if i < self.fields.len() - 1 {
                out += "\n";
            }
        }
        out
    }

    pub fn get_row(&self, row_index: u8) -> Vec<u8> {
        self.fields.get(row_index as usize).unwrap().clone()
    }

    pub fn get_col(&self, col_index: u8) -> Vec<u8> {
        self.fields
            .clone()
            .into_iter()
            .map(|r| r[col_index as usize])
            .collect()
    }

    pub fn get_parcel(&self, index: u8) -> Vec<Vec<u8>> {
        let start_row = (index / 3) * 3;
        let start_col = (index % 3) * 3;
        let mut parcel = vec![vec![0; 3]; 3];
        for ci in 0..3 {
            for ri in 0..3 {
                let row = start_row + ri;
                let col = start_col + ci;
                parcel[ri as usize][ci as usize] = self.get(&Field::new(row, col))
            }
        }
        parcel
    }

    /// Resets the sudoku to its original values by setting all mutable fields to
    /// zero.
    pub fn reset(&mut self) {
        let mutable_fields = self.mutable_fields.clone();
        for field in mutable_fields.iter() {
            self.set(field, 0);
        }
    }

    /// Returns all field indicies (row, column) of a mutable fields in a parcel.
    pub fn get_mutable_fields_of_parcel(&self, parcel_index: u8) -> Vec<Field> {
        let parcel_fields = Grid::get_parcel_fields(parcel_index);
        parcel_fields
            .into_iter()
            .filter(|f| self.mutable_fields.contains(&f))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_field_value() {
        let grid = Grid::new(vec![vec![0; 9]; 9]);
        assert_eq!(grid.get(&Field::new(0, 0)), 0);
        assert_eq!(grid.get(&Field::new(8, 8)), 0);
    }

    #[test]
    fn it_should_list_all_parcel_fields() {
        assert_eq!(
            Grid::get_parcel_fields(0),
            vec![
                Field::new(0, 0),
                Field::new(0, 1),
                Field::new(0, 2),
                Field::new(1, 0),
                Field::new(1, 1),
                Field::new(1, 2),
                Field::new(2, 0),
                Field::new(2, 1),
                Field::new(2, 2)
            ]
        );
        assert_eq!(
            Grid::get_parcel_fields(7),
            vec![
                Field::new(6, 3),
                Field::new(6, 4),
                Field::new(6, 5),
                Field::new(7, 3),
                Field::new(7, 4),
                Field::new(7, 5),
                Field::new(8, 3),
                Field::new(8, 4),
                Field::new(8, 5)
            ]
        );
    }
}
