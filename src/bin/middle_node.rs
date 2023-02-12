use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;
use syc_leetcode_solution_rs::ListNode;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let (input, list) = parse_util::parse_list(&buffer).unwrap();
    assert!(
        input.trim().len() == 0,
        "Please enter one square-bracket-enclosed-list in one line.",
    );
    let nums: Vec<i32> = list.iter().map(|s| s.parse().unwrap()).collect();

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
