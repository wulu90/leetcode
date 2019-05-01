pub struct MinStack {
    min: i32,
    vec: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            min: i32::max_value(),
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        if x <= self.min {
            self.vec.push(self.min);
            self.min = x;
        }
        self.vec.push(x)
    }

    pub fn pop(&mut self) {
        if self.vec.pop().unwrap() == self.min {
            self.min = self.vec.pop().unwrap();
        }
    }

    pub fn top(&self) -> i32 {
        *self.vec.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod test {
    use super::MinStack;

    #[test]
    fn minstack_test() {
        let mut minstack = MinStack::new();
        minstack.push(-2);
        minstack.push(0);
        minstack.push(-3);
        assert_eq!(-3, minstack.get_min());
        minstack.pop();
        assert_eq!(0, minstack.top());
        assert_eq!(-2, minstack.get_min());
    }

}
