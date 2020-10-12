use std::fmt;

#[derive(Debug)]
pub struct Field {
    pub row: u8,
    pub column: u8,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row, self.column)
    }
}

impl Field {
    pub fn new(row: u8, column: u8) -> Field {
        Field { row, column }
    }
}

impl std::cmp::PartialEq for Field {
    fn eq(&self, other: &Field) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl std::clone::Clone for Field {
    fn clone(&self) -> Field {
        Field::new(self.row, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_format() {
        assert_eq!(format!("{}", Field::new(1, 2)), "(1,2)");
    }
}
