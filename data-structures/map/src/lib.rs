
#[allow(dead_code)]
pub struct Map<T, U> {
    keys: Vec<T>,
    values: Vec<U>,
    len: usize,
}
#[derive(Debug, PartialEq)]
pub enum MapError {
    RepeatedKey,
    NoKey,
    DifferentLenght,
}
type _MyError = Result<(), MapError>;

use std::cmp::PartialEq;
impl<T: PartialEq + Clone, U: Clone> Map<T, U> {
    
    pub fn new(key: T, val: U) -> Map<T, U> {
        Map { keys: vec![key], 
            values: vec![val], 
            len: 1 }
    }

    pub fn insert(&mut self, key: T, val: U) -> _MyError {
        
        let exists = self.keys.iter()
            .find(|&x| x == &key);
        
        if exists.is_some() {
            return Err(MapError::RepeatedKey);
        }

        self.keys.push(key);
        self.values.push(val);
        self.len += 1;
        Ok(())
    }
    // Function to create a Map with many vaalues at once.
    // if keys and values vectors have different lenghts
    // this function will return an error.
    pub fn from(mut keys: Vec<T>, mut values: Vec<U>) ->  Result<Map<T, U>, MapError> {
        
        if keys.len() != values.len() {
            return Err(MapError::DifferentLenght);
        }

        let k = keys.swap_remove(0);
        let v = values.swap_remove(0);

        
        let mut map = Map::new(k, v);

        for i in 0..keys.len() {
            map.insert(keys.swap_remove(i), values.swap_remove(i));
        }
        Ok(map) 
    }

    fn _find_value(&self, key: &T) -> Option<usize> {
        return self.keys.iter()
            .position(|x| x == key);
    } 

    // delete pair by looking at key.
    pub fn delete(&mut self, key: T) -> _MyError {
        
       let element = self._find_value(&key);

       match element {
           None => return Err(MapError::NoKey),
           Some(x) => {
                self.values.remove(x);
                self.keys.remove(x);
                self.len -= 1;
                return Ok(());
           },
       }
    }

    pub fn keys(&self) -> &Vec<T> {
        &self.keys
    }
    pub fn values(&self) -> &Vec<U> {
        &self.values
    }

    // uses key to find the associated value.
    pub fn get_value(&self, key: T) -> Option<&U> {
        
        let index = self._find_value(&key);

        return match index {
            None => None,
            Some(i) => Some(&self.values[i]),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MapError::{RepeatedKey, NoKey};

    #[test]
    fn basics() {
        let mut map = Map::new("zero", 0);

        map.insert("one", 1).unwrap();
        map.insert("foo", 100).unwrap();
        map.delete("foo").unwrap();

        assert_eq!(None, map.get_value("foo"));
        assert_eq!(Some(&1), map.get_value("one"));

        assert_eq!(Err(RepeatedKey), map.insert("one", 40));
        assert_eq!(Err(NoKey), map.delete("xyz"));
        
    }
}
