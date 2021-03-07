mod queue;
mod stack;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_enqueue() {
        let mut queue = queue::Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_queue_dequeue() {
        let mut queue = queue::Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue().unwrap(), 1);
        assert_eq!(queue.dequeue().unwrap(), 2);
        assert_eq!(queue.dequeue().unwrap(), 3);
    }

    #[test]
    fn test_queue_peek() {
        let mut queue = queue::Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(*queue.peek().unwrap(), 1);
    }

    #[test]
    fn test_stack_push() {
        let mut stack = stack::Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = stack::Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = stack::Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(*stack.peek().unwrap(), 3);
    }
}
