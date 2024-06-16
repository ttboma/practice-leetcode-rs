use std::alloc::{self, Layout};
use std::mem;
use std::ptr::{self, NonNull};

/// # [155. Min Stack](https://leetcode.com/problems/min-stack/description/?envType=study-plan-v2&envId=top-interview-150)
///
/// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
///
/// Implement the `MinStack` class:
///
/// - `MinStack()` initializes the stack object.
/// - `void push(int val)` pushes the element `val` onto the stack.
/// - `void pop()` removes the element on the top of the stack.
/// - `int top()` gets the top element of the stack.
/// - `int getMin()` retrieves the minimum element in the stack.
///
/// You must implement a solution with `O(1)` time complexity for each function.
///
/// **Example 1:**
///
/// ```txt
/// Input
///
/// ["MinStack","push","push","push","getMin","pop","top","getMin"]
/// [[],[-2],[0],[-3],[],[],[],[]]
///
/// Output
///
/// [null,null,null,null,-3,null,0,-2]
///
/// Explanation
///
/// MinStack minStack = new MinStack();
/// minStack.push(-2);
/// minStack.push(0);
/// minStack.push(-3);
/// minStack.getMin(); // return -3
/// minStack.pop();
/// minStack.top();    // return 0
/// minStack.getMin(); // return -2
/// ```
///
/// **Constraints:**
///
/// - `-2^31 <= val <= 2^31 - 1`
/// - Methods `pop`, `top` and `getMin` operations will always be called on **non-empty**  stacks.
/// - At most `3 * 10^4` calls will be made to `push`, `pop`, `top`, and `getMin`.
#[derive(Debug)]
pub struct MinStack {
    ele: MyVec<i32>,
    min: MyVec<i32>,
}

#[derive(Debug)]
struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        MyVec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <= isize::MAX.
            let new_cap = 2 * self.cap;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }

        // Can't fail, we'll OOM first.
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    pub fn top(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { &*self.ptr.as_ptr().add(self.len - 1) })
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while self.pop().is_some() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl Default for MinStack {
    fn default() -> Self {
        Self::new()
    }
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            ele: MyVec::new(),
            min: MyVec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.ele.push(val);
        if let Some(min) = self.min.top() {
            if *min < val {
                return;
            }
        }
        self.min.push(val);
    }

    pub fn pop(&mut self) {
        // Methods `pop`, `top` and `getMin` operations will always be called on **non-empty**  stacks.
        let ele = self.ele.pop().unwrap();
        let min = *self.min.top().unwrap();

        if ele == min {
            self.min.pop();
        }
    }

    pub fn top(&self) -> i32 {
        // Methods `pop`, `top` and `getMin` operations will always be called on **non-empty**  stacks.
        *self.ele.top().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        // Methods `pop`, `top` and `getMin` operations will always be called on **non-empty**  stacks.
        *self.min.top().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
