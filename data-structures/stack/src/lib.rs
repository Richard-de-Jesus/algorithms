pub struct Stack<T> {
    top: i32,
    capacity: usize,
    array: Vec<T>,
}
#[derive(Debug)]
pub enum Fail {
    Full,
    Empty,
}

use std::cmp::PartialOrd;
impl<T: PartialOrd> Stack<T> {
    pub fn new(capacity: usize) -> Stack<T> {
        let stack = Stack {
            top: -1,
            capacity,
            array: Vec::with_capacity(capacity),
        };
        stack
    }

    fn is_full(&self) -> bool {
        self.top == self.capacity as i32 - 1
    }

    fn is_empty(&self) -> bool {
        self.top == -1
    }

    pub fn push(&mut self, item: T) -> Result<(), Fail> {
        if self.is_full() {
            return Err(Fail::Full);
        }
        self.top += 1;
        // have to 'cheat' here with push, to prevent panic with
        // new stack, wich vector have 0 lenght.
        self.array.push(item);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<(), Fail> {
        if self.is_empty() {
            return Err(Fail::Empty);
        }

        self.top -= 1;
        self.array.pop();
        Ok(())
    }
    // look at top of stacck.
    pub fn top(&self) -> Result<&T, Fail> {
        if self.is_empty() {
            return Err(Fail::Empty);
        }
        Ok(&self.array[self.top as usize])
    }

    pub fn from(items: Vec<T>, capacity: usize) -> Stack<T> {
        
        let mut st = Stack::new(capacity);

        for e in items {
            st.push(e).unwrap();
        }
        st
    }

}

// now we have the functions to convert  infix
// to postfix notation.
pub fn order(ch: char) -> i32 {
    return match ch {
        '^' => 3,
        '/' | '*' => 2,
        '+' | '-' => 1,
        _ => -1,
    };
}
// push char from stack to string then pop the stack.
fn _st_to_str(st: &mut Stack<char>, res: &mut String) -> Result<(), Fail> {
    res.push(*st.top()?);
    st.pop()?;
    Ok(())
}

fn _in_to_post(arg: String) -> Result<String, Fail> {
    let mut stack = Stack::new(arg.len());
    let mut result = String::from("");

    for ch in arg.chars() {
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' => result.push(ch),
            '(' => stack.push('(')?,
            ')' => {
                while *stack.top()? != '(' {
                    _st_to_str(&mut stack, &mut result)?;
                }
                stack.pop()?;
            }
            ' ' => (),
            _ => {
                let f = order(ch);
                while !stack.is_empty() && f <= order(*stack.top()?) {
                    _st_to_str(&mut stack, &mut result)?;
                }
                stack.push(ch)?;
            }
        }
    };

    while !stack.is_empty() {
        _st_to_str(&mut stack, &mut result)?;
    }
    return Ok(result);
}
// wrapper, so that there is no need for 
// .unwrap() in the API.
pub fn in_to_post(arg: String) -> String {
    _in_to_post(arg).unwrap()
}

fn _post_to_in(arg: String) -> Result<String, Fail> {
    let mut stack = Stack::new(arg.len() * 2);

    for ch in arg.chars() {
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                stack.push(ch.to_string())?;
            }
            _ => {
                let a = stack.top()?.clone();
                stack.pop()?;
                let b = stack.top()?.clone();
                stack.pop()?;
                
                let exp = format!("({b}{ch}{a})");
                stack.push(exp)?;            
            } 
        }
    }
    
    Ok(stack.top()?.to_string())
}
// another wrapper to remove .unwrap() in API
pub fn post_to_in(arg: String) -> String {
    _post_to_in(arg).unwrap()
}

#[cfg(test)]
mod test {
    use crate::{in_to_post, Stack, post_to_in};
    use Stack as St;
    #[test]
    fn basic() {
        let mut foo = St::from(vec![5,7,11], 42);

        assert_eq!(11, *foo.top().unwrap());

        foo.pop().unwrap();
        assert_eq!(7, *foo.top().unwrap());

        let foo = St::from(vec!['a','w'], 10);

        assert_eq!('w', *foo.top().unwrap());
    }
    #[test]
    fn full() {
        let mut foo = St::new(1);

        foo.push(4.2).unwrap();
        let err = match foo.push(5.1) {
            Ok(_x) => 0,
            Err(_e) => 1,
        };
        assert_eq!(1, err);

        foo.pop().unwrap();
        let err = match foo.pop() {
            Ok(_x) => 0,
            Err(_e) => 1,
        };
        assert_eq!(1, err);
    }
    #[test]
    fn expression_conversion() {
        let mut infix = "a+b*(c^d-e)^(f+g*h)-i".to_string();
        let postfix = String::from("abcd^e-fgh*+^*+i-");

        assert_eq!(postfix, in_to_post(infix.clone()));

        infix = "((a+(b*(((c^d)-e)^(f+(g*h)))))-i)".to_string();
        assert_eq!(infix, post_to_in(postfix.clone())); 
    }
}
