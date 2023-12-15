pub struct FlatMap<O, F, U>
where
    U: IntoIterator,
{
    outer: O,
    mapped: Option<U::IntoIter>,
    mapper: F,
}

impl<O, F, U> FlatMap<O, F, U>
where
    U: IntoIterator,
{
    pub fn new(iterator: O, f: F) -> Self {
        Self {
            outer: iterator,
            mapped: None,
            mapper: f,
        }
    }
}

pub fn flat_map<O, F, U>(i: O, f: F) -> FlatMap<O, F, U>
where
    O: Iterator,
    U: IntoIterator,
    F: FnMut(O::Item) -> U,
{
    FlatMap::new(i, f)
}

impl<O, F, U> Iterator for FlatMap<O, F, U>
where
    O: Iterator,
    U: IntoIterator,
    F: FnMut(O::Item) -> U,
{
    type Item = U::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut unmapped_inner) = self.mapped {
                if let Some(unmapped) = unmapped_inner.next() {
                    return Some(unmapped);
                }
                self.mapped = None;
            }

            let next_mapped = self.outer.next()?;
            self.mapped = Some((self.mapper)(next_mapped).into_iter());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_lib_test() {
        let merged: String = flat_map(vec!["123", "GGG", "AAAA"].iter(), |s| s.chars()).collect();

        assert_eq!(merged, "123GGGAAAA");
    }

    #[test]
    fn ignore_whitespaces() {
        let words = ["alpha", "", "beta", "", "", "gamma"];
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect();

        assert_eq!(merged, "alphabetagamma");
    }
}
