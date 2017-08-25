use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Period {
    /// 5 minutes (300 seconds)
    M5,

    /// 15 minutes (900 seconds)
    M15,

    /// 30 minutes (1800 seconds)
    M30,

    /// 2 hours (7200 seconds)
    H2,

    /// 4 hours (14400 seconds)
    H4,

    /// 1 day(24 hours, 14400 seconds)
    D1
}

impl<'a> Into<i64> for &'a Period {
    fn into(self) -> i64 {
        match *self {
            Period::M5 => 300,
            Period::M15 => 900,
            Period::M30 => 1_800,
            Period::H2 => 7_200,
            Period::H4 => 14_400,
            Period::D1 => 86_400
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
