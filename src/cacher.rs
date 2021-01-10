use std::collections::HashMap;

/// A Cacher is a feature that accepts a function to calculate a result, and allows the user to ask
/// the Cacher for the result of a specific value. The Cacher will automatically store the results
/// inside of a HashMap, so when you ask for the same value twice, the result will only be
/// calculated once.
///
/// Basic usage:
/// ```
/// use inxane_utils::Cacher;
///
/// // Construct a new Cacher, with a function that simply returns it's input * 2.
/// let mut cache = Cacher::new(|num| {
///     num * 2
/// });
///
/// println!("{}", cache.value(15)); // First calculation occurs here
/// println!("{}", cache.value(15)); // No calculation here! It'll just take the result out of the cache.
///
/// assert_eq!(cache.value(15), 30);
/// ```
pub struct Cacher<Func, FuncArg, FuncRet> {
    func: Func,
    results: HashMap<FuncArg, FuncRet>
}

impl<Func, FuncArg, FuncRet> Cacher<Func, FuncArg, FuncRet>
    where Func: Fn(&FuncArg) -> FuncRet,
          FuncArg: Eq + std::hash::Hash,
          FuncRet: Copy + Clone
{
    /// Constructs a new Cacher with the specified closure for the calculation function.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure, used to calculate each result
    pub fn new(f: Func) -> Self {
        Self {
            func: f,
            results: HashMap::new()
        }
    }
    /// Get the calculation result for the specified argument.
    ///
    /// Arguments:
    /// * `v` - The value for which we want to get the calculation result.
    pub fn value(&mut self, v: FuncArg) -> FuncRet {
        match self.results.get(&v) {
            Some(result) => *result,
            None => {
                let result = (self.func)(&v);
                self.results.insert(v, result);
                result
            }
        }
    }
}

#[cfg(test)]
mod cacher_tests {
    use super::*;
    #[test]
    fn basic_usage() {
        let mut cache = Cacher::new(|num| {
            num * 2
        });
        cache.value(2);
        cache.value(2);
        cache.value(3);
        cache.value(3);
        assert!(cache.results.len() == 2 && *cache.results.get(&2).unwrap() == 4);
    }
}
