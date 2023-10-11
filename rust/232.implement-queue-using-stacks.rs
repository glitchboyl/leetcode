/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
struct MyQueue {
    data: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue {
            data: Vec::<i32>::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.data.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        return self.data.remove(0);
    }
    
    fn peek(&self) -> i32 {
        return self.data[0];
    }
    
    fn empty(&self) -> bool {
        return self.data.is_empty();
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

