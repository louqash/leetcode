// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut node = &mut l1;
        let mut sum = 0;
        while l2.is_some() || sum != 0 {
            if let Some(list_node2) = l2 {
                sum += list_node2.val;
                l2 = list_node2.next;
            }

            if node.is_none() {
                *node = Some(Box::new(ListNode::new(0)));
            }
            let list_node1 = node.as_mut().unwrap();
            sum += list_node1.val;
            list_node1.val = sum % 10;
            sum = sum / 10;

            node = &mut list_node1.next;
        }
        l1
    }
}
