use std;

struct Queue {
    members: [i8]
}

impl Queue {
    fn len(self) -> i32 {
        self.members.len()
    }

    fn enqueue(&mut self, member: i8) {
        self.members.append(member)
    }

    fn dequeue(&mut self) -> Option<i8> {
        if let Some(last, members) = self.members.split_last_mut() {
            self.members = members;
            last
        }

        None
    }

    fn peek(self) -> Option<i8> {
        self.members.last()
    }
}