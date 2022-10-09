use std::ops::Rem;
use std::fmt;

pub struct Matcher<T> {
    condition: Box<dyn Fn(T) -> bool>,
    substitute: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S: ToString>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
    {
        Matcher {
            condition: Box::new(matcher),
            substitute: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Copy + Clone + ToString,
{
    pub fn new() -> Self {
        Fizzy { matchers: vec![] }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);

        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |v| {
            let word = self
                .matchers
                .iter()
                .filter(|matcher| (matcher.condition)(v))
                .map(|matcher| matcher.substitute.clone())
                .collect::<String>();

            if word.is_empty() {
                v.to_string()
            } else {
                word
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Rem<Output = T> + From<u8> + PartialEq + Copy + fmt::Display,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
