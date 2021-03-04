use std;

struct Stack {
    members: [i8]
}

impl Stack {
    fn len(self) -> i32 {
        self.members.len()
    }

    fn push(&mut self, member:i8) {
        self.members = [[member], &self.members].concat()
    }

    fn pop(&mut self) -> Option<i8> {
        if let Some(first, members) = self.members.split_first() {
            self.members = members;
            first
        }

        None
    }

    fn peek(self) -> i8 {
        self.members.first()
    }
}