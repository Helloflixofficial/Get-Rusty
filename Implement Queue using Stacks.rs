#[derive(Default)]
pub struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self {
            stack1: vec![],
            stack2: vec![],
        }
    }

    pub fn push(&mut self, x: i32) {
        while let Some(val) = self.stack1.pop() {
            self.stack2.push(val);
        }
        self.stack1.push(x);
        while let Some(val) = self.stack2.pop() {
            self.stack1.push(val);
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.stack1.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        *self.stack1.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}

//// I need to revise this again. I'm still lacking [something]. but still i will do this again with meine and dif methods??//sadf
