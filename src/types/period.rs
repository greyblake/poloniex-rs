use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Period {
    P5m,
    P15m,
    P30m,
    P2h,
    P4h,
    P1d
}

impl<'a> Into<i64> for &'a Period {
    fn into(self) -> i64 {
        match *self {
            Period::P5m => 300,
            Period::P15m => 900,
            Period::P30m => 1800,
            Period::P2h => 7200,
            Period::P4h => 14400,
            Period::P1d => 86400
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
        assert_eq!(format!("{}", P5m), "300");
        assert_eq!(format!("{}", P15m), "900");
        assert_eq!(format!("{}", P30m), "1800");
        assert_eq!(format!("{}", P2h), "7200");
        assert_eq!(format!("{}", P4h), "14400");
        assert_eq!(format!("{}", P1d), "86400");
    }
}
