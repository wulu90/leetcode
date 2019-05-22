/**
 * Implement the following operations of a stack using queues.
 *
 * push(x) -- Push element x onto stack.
 * pop() -- Removes the element on top of the stack.
 * top() -- Get the top element.
 * empty() -- Return whether the stack is empty.
 * Example:
 *
 * MyStack stack = new MyStack();
 *
 * stack.push(1);
 * stack.push(2);  
 * stack.top();   // returns 2
 * stack.pop();   // returns 2
 * stack.empty(); // returns false
 * Notes:
 *
 * You must use only standard operations of a queue -- which means only push to back, peek/pop from front, size, and is empty operations are valid.
 * Depending on your language, queue may not be supported natively. You may simulate a queue by using a list or deque (double-ended queue), as long as you use only standard operations of a queue.
 * You may assume that all operations are valid (for example, no pop or top operations will be called on an empty stack).
 *
 */
use std::collections::VecDeque;

pub struct MyStack {
    stack: VecDeque<i32>,
    temp: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack {
            stack: VecDeque::new(),
            temp: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stack.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        let mut v = self.stack.front().unwrap().clone();
        for _ in 0..self.stack.len() {
            v = self.stack.pop_front().unwrap();
            self.temp.push_back(v);
        }
        for _ in 0..self.temp.len() - 1 {
            self.stack.push_back(self.temp.pop_front().unwrap());
        }
        //empty temp
        self.temp.pop_front();
        v
    }

    pub fn top(&mut self) -> i32 {
        let mut v = self.stack.front().unwrap().clone();
        for _ in 0..self.stack.len() {
            v = self.stack.pop_front().unwrap();
            self.temp.push_back(v);
        }
        for _ in 0..self.temp.len() {
            self.stack.push_back(self.temp.pop_front().unwrap());
        }
        v
    }

    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::MyStack;

    #[test]
    fn mystack_test() {
        let mut obj = MyStack::new();

        obj.push(1);
        obj.push(2);
        let ret_2: i32 = obj.pop();
        println!("{:?}", obj.stack);
        assert_eq!(2, ret_2);
        let ret_3: i32 = obj.top();
        println!("{:?}", obj.stack);
        assert_eq!(1, ret_3);
        let ret_4: bool = obj.empty();
        println!("{:?}", obj.stack);
        assert_eq!(false, ret_4);
        let ret_5 = obj.pop();
        println!("{:?}", obj.stack);
        assert_eq!(1, ret_5);
        println!("{:?}", obj.stack);
        assert_eq!(true, obj.empty());
    }

}
