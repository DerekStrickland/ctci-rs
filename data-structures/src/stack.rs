use std;

pub struct Stack {
    members: Vec<i8>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            members:Vec::new(),
        }
    }

    pub fn len(self) -> usize {
        self.members.len()
    }

    pub fn push(&mut self, member:i8) {
        self.members.push(member);
    }

    pub fn pop(&mut self) -> Option<i8> {
        self.members.pop()
    }

    pub fn peek(self) -> Option<i8> {
        Some(self.members[self.members.len() -1])
    }
}