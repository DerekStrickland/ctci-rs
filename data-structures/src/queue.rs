use std;
use std::io::Empty;
use std::error::Error;

pub struct Queue {
    members: Vec<i8>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            members: Vec::new(),
        }
    }

    pub fn len(self) -> usize {
        self.members.len()
    }

    pub fn enqueue(&mut self, member: i8) {
        self.members.push(member)
    }

    pub fn dequeue(&mut self) -> Option<i8> {
        match self.members.len() {
            0 => None,
            _ => Some(self.members.remove(0))
        }
    }

    pub fn peek(self) -> Option<i8> {
       Some(self.members[0])
    }
}