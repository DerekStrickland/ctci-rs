
pub struct SinglyLinkedNode<T> {
    pub data: T,
    pub next: Option<SinglyLinkedNode<T>>
}

impl<T> SinglyLinkedNode<T> {
    pub fn new(data: T) -> Self {
        SinglyLinkedNode{
            data,
            next: None
        }
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<SinglyLinkedNode<T>>
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList{
            head: None
        }
    }

    pub fn add(&mut self, item: T) {
        let mut next = &self.head.next;

        while next != None {
            next = next.next
        }

        next.next = Some(SinglyLinkedNode::new(item));
    }

    pub fn indexOf(&self, item: T) -> i32 {
        if self.head == None { return -1; }

        let head = self.head.unwrap();
        if head.data == item { return 0; }

        let mut index = 0;
        let mut next = head.next;

        while next != None {
            index = index + 1;
            let nextNode = next.unwrap();
            if nextNode.data == item {
                return index;
            }
            next = nextNode.next;
        }

        -1
    }

    pub fn len(&self) -> i32 {
        if &self.head == None {
            0
        }

        let mut len = 1;
        let mut next = self.head.unwrap().next;

        while next != None {
            len = len + 1;
            next = next.next;
        }

        len
    }
}