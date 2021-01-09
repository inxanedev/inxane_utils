use std::collections::HashMap;

pub struct Cacher<Func, FuncArg, FuncRet> {
    func: Func,
    results: HashMap<FuncArg, FuncRet>
}

impl<Func, FuncArg, FuncRet> Cacher<Func, FuncArg, FuncRet>
    where Func: Fn(&FuncArg) -> FuncRet,
          FuncArg: Eq + std::hash::Hash,
          FuncRet: Copy + Clone
{
    pub fn new(f: Func) -> Self {
        Self {
            func: f,
            results: HashMap::new()
        }
    }
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
