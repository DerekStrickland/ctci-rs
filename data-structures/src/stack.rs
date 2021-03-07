
pub struct Stack<T> {
    members: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            members:Vec::new(),
        }
    }

    pub fn len(self) -> usize {
        self.members.len()
    }

    pub fn push(&mut self, member:T) {
        self.members.push(member);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.members.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.members.last()
    }
}