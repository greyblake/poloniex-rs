use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Period {
    M5,
    M15,
    M30,
    H2,
    H4,
    D1
}

impl<'a> Into<i64> for &'a Period {
    fn into(self) -> i64 {
        match *self {
            Period::M5 => 300,
            Period::M15 => 900,
            Period::M30 => 1800,
            Period::H2 => 7200,
            Period::H4 => 14400,
            Period::D1 => 86400
        }
    }
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num: i64 = self.into();
        write!(f, "{}", num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::Period::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", M5), "300");
        assert_eq!(format!("{}", M15), "900");
        assert_eq!(format!("{}", M30), "1800");
        assert_eq!(format!("{}", H2), "7200");
        assert_eq!(format!("{}", H4), "14400");
        assert_eq!(format!("{}", D1), "86400");
    }
}
