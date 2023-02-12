use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;
use syc_leetcode_solution_rs::ListNode;

fn main() {
    let nums = parse_util::read_i32_list();

    // Store nums into head
    let mut head: Option<Box<ListNode>> = None;
    for i in nums.iter().rev() {
        head = Some(Box::new(ListNode{val: *i, next: head}));
    }
    head = Solution::middle_node(head);

    // dump list from head to a vector to be printed
    let mut v: Vec<i32> = vec![];
    while let Some(node) = head {
        v.push(node.val);
        head = node.next;
    }
    println!("{:?}", v);
}
