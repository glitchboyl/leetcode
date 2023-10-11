/*
 * @lc app=leetcode id=225 lang=rust
 *
 * [225] Implement Stack using Queues
 */

// @lc code=start
struct MyStack {
    data: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        MyStack {
            data: Vec::<i32>::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.data.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        return self.data.pop().unwrap();
    }
    
    fn top(&self) -> i32 {
        return self.data[self.data.len() - 1];
    }
    
    fn empty(&self) -> bool {
        return self.data.is_empty();
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

