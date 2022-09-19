mod invokes;
mod iter;
mod ref_arg;

pub use invokes::*;
pub use iter::*;
pub use ref_arg::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct WordFrequency {
        inner: HashMap<String, usize>,
    }

    impl WordFrequency {
        fn new() -> Self {
            Self {
                inner: HashMap::new(),
            }
        }

        fn add(&mut self, word: &str) {
            self.inner
                .entry(String::from(word))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    impl<'a> IntoIterator for &'a WordFrequency {
        type Item = (&'a str, usize);

        type IntoIter = MapInvoke<
            <&'a HashMap<String, usize> as IntoIterator>::IntoIter,
            fn((&'a String, &'a usize)) -> (&'a str, usize),
        >;

        fn into_iter(self) -> Self::IntoIter {
            self.inner.iter().map_invoke(|(s, c)| (s.as_str(), *c))
        }
    }

    #[test]
    fn should_implement_hashmap_wrapper_with_iterators() {
        let mut sut = WordFrequency::new();
        sut.add("red");
        sut.add("green");
        sut.add("red");

        let mut got = sut.into_iter().collect::<Vec<_>>();
        got.sort();

        assert_eq!(vec![("green", 1), ("red", 2)], got);
    }

    #[test]
    fn map_iter_should_be_iterable() {
        let src = ["red", "green", "blue"];
        let iter: MapInvoke<std::slice::Iter<&str>, RefArg<fn(&'static str) -> String>> =
            src.iter().map_invoke(RefArg::new(String::from));

        assert_eq!(
            src.into_iter().map(String::from).collect::<Vec<_>>(),
            iter.collect::<Vec<_>>()
        );
    }

    #[test]
    fn fn_should_be_invokable() {
        let maplen: fn(&str) -> usize = str::len;
        let len = maplen.invoke(("foobar",));
        assert_eq!(6, len);
    }

    #[test]
    fn flat_map_iter_should_be_iterable() {
        let src: [&'static str; 3] = ["red", "green", "blue"];
        let iter: FlatMapInvoke<std::slice::Iter<&str>, RefArg<fn(&str) -> std::str::Chars>> =
            src.iter().flat_map_invoke(RefArg::new(str::chars));

        let expected = String::from("redgreenblue");
        let v = iter.collect::<String>();
        assert_eq!(expected, v);
    }
}
