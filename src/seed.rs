use std::str::FromStr;

#[derive(Debug, Clone)]
pub(crate) struct SeedRange {
    start: u64,
    end: u64,
}

impl SeedRange {
    pub(crate) fn new(start: u64, end: u64) -> Self {
        SeedRange { start, end }
    }

    pub(crate) fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    #[cfg(test)]
    pub(crate) fn start(&self) -> u64 {
        self.start
    }

    #[cfg(test)]
    pub(crate) fn end(&self) -> u64 {
        self.end
    }
}

impl FromStr for SeedRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Range format error for {s}, expected format: start-end"
            ));
        }

        let start = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => return Err(format!("Range start value ({}) parse error", parts[0])),
        };
        let end = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => return Err(format!("Range end value ({}) parse error", parts[1])),
        };

        if start > end {
            return Err(format!(
                "Range start value {start} is greater than end value {end}"
            ));
        }

        Ok(SeedRange { start, end })
    }
}

pub(crate) struct SeedRangeIterator<'a> {
    range: &'a SeedRange,
    index: u64,
}

impl<'a> IntoIterator for &'a SeedRange {
    type Item = u64;
    type IntoIter = SeedRangeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let index = self.start;
        SeedRangeIterator { range: self, index }
    }
}

impl Iterator for SeedRangeIterator<'_> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.range.end {
            None
        } else {
            let seed = self.index;
            self.index += 1;
            Some(seed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_errors() {
        assert_eq!(
            SeedRange::from_str("1234").err(),
            Some("Range format error for 1234, expected format: start-end".to_owned())
        );

        assert_eq!(
            SeedRange::from_str("a-1234").err(),
            Some("Range start value (a) parse error".to_owned())
        );

        assert_eq!(
            SeedRange::from_str("1234-xyz").err(),
            Some("Range end value (xyz) parse error".to_owned())
        );

        assert_eq!(
            SeedRange::from_str("1234-1").err(),
            Some("Range start value 1234 is greater than end value 1".to_owned())
        );
    }

    #[test]
    fn test_seed_range_iterator() {
        let start = 0u64;
        let end = 10u64;
        let seed_range = SeedRange::new(start, end);
        let seeds: Vec<u64> = seed_range.into_iter().collect();
        let expected: Vec<u64> = (start..end + 1).collect();
        assert_eq!(seeds.len(), (end - start + 1) as usize);
        assert!(itertools::equal(seeds, expected));
    }
}
