
pub struct Queue<T> {
    members: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            members: Vec::new(),
        }
    }

    pub fn len(self) -> usize {
        self.members.len()
    }

    pub fn enqueue(&mut self, member: T) {
        self.members.push(member)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        match self.members.len() {
            0 => None,
            _ => Some(self.members.remove(0))
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.members.first()
    }
}