use super::field::Field;
use std::iter;

#[derive(Debug)]
pub struct Grid {
    fields: Vec<Vec<u8>>,
}

impl std::clone::Clone for Grid {
    fn clone(&self) -> Grid {
        Grid {
            fields: self.fields.clone(),
        }
    }
}

impl Grid {
    pub fn new(fields: Vec<Vec<u8>>) -> Grid {
        Grid { fields }
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

    pub fn get_mutable_fields(&self) -> Vec<Field> {
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
}
