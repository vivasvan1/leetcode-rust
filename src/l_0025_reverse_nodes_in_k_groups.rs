pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        // 1. Set up a dummy node to act as our stable anchor anchor
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        fn is_k_depth_available(current: &Option<Box<ListNode>>, k: i32) -> bool {
            let mut ptr = current.as_ref();
            let mut depth = 0;
            while let Some(node) = ptr {
                depth += 1;
                if depth >= k {
                    return true;
                }
                ptr = node.next.as_ref();
            }
            false
        }

        fn split_segment(segment: &mut Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
            let mut curr = segment;

            for _ in 0..n {
                if let Some(node) = curr {
                    // println!("node : {:?}", &node);
                    curr = &mut node.next;
                }
            }
            curr.take()
        }

        // k==1 means no need to reverse
        if k <= 1 {
            return dummy.next;
        }

        // < 2 nodes means nothing to do
        if dummy.next.is_none() {
            return dummy.next;
        }

        let mut prev_group_head = &mut dummy;
        // println!("whole list with dummy:  {:?}", prev_group_tail);

        while is_k_depth_available(&prev_group_head.next, k) {
            let mut next_group_head = split_segment(&mut prev_group_head.next, k);
            // println!(" right split : {:?}", next_group_head);
            // println!(" left split : {:?}", prev_group_head.next);
            // let first_minus_1 = prev_group_head;
            let mut current_segment = prev_group_head.next.take();
            let mut prev: Option<Box<ListNode>> = None;
            for i in 0..k {
                // println!("current segment : {:?}", current_segment);

                if let Some(mut node) = current_segment {
                    let next_node = node.next.take();
                    if i == 0 {
                        node.next = next_group_head.take();
                        prev = Some(node);
                    }else if i == k-1 {
                        node.next = prev.take();
                        prev_group_head.next = Some(node);
                    }else{
                        node.next = prev.take();
                        prev= Some(node);
                    }

                    current_segment = next_node;
                }
            }
            
            // println!("prev_group_head : {:?}", prev_group_head);
            let mut ptr = prev_group_head.next.as_mut();
            for _ in 0..k-1 {
                ptr = ptr.unwrap().next.as_mut();
            }
            match ptr {
                Some(node) => {
                    prev_group_head = node;
                }
                None => {
                    break;
                }
            }
            // println!("prev_group_head : {:?}", &prev_group_head);
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to build a linked list from a vector
    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
    //
    // #[test]
    // fn test_empty_list() {
    //     // Input: []
    //     // Output: []
    //     assert_eq!(
    //         Solution::reverse_k_group(to_list(vec![]), 2),
    //         to_list(vec![])
    //     );
    // }
    //
    // #[test]
    // fn test_single_element() {
    //     // Input: [1]
    //     // Output: [1]
    //     assert_eq!(
    //         Solution::reverse_k_group(to_list(vec![1]), 2),
    //         to_list(vec![1])
    //     );
    // }
    //
    // #[test]
    // fn test_even_number_of_elements() {
    //     // Input: [1, 2, 3, 4]
    //     // Output: [2, 1, 4, 3]
    //     assert_eq!(
    //         Solution::reverse_k_group(to_list(vec![1, 2, 3, 4]), 2),
    //         to_list(vec![2, 1, 4, 3])
    //     );
    // }

    #[test]
    fn test_odd_number_of_elements() {
        // Input: [1, 2, 3, 4, 5]
        // Output: [2, 1, 4, 3, 5]
        // The last element '5' should remain in place.
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![2, 1, 4, 3, 5])
        );
    }

    // #[test]
    // fn test_reverse_k_group_1() {
    //     // Input: [1, 2, 3, 4, 5, 6, 7]
    //     // Output: [3, 2, 1, 6, 5, 4, 7]
    //     assert_eq!(
    //         Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5, 6, 7]), 3),
    //         to_list(vec![3, 2, 1, 6, 5, 4, 7])
    //     );
    // }
    //
    //
    // #[test]
    // fn test_reverse_k_group_2() {
    //     // Input: [1, 2, 3, 4, 5, 6, 7]
    //     // Output: [3, 2, 1, 6, 5, 4, 7]
    //     assert_eq!(
    //         Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5, 6, 7]), 8),
    //         to_list(vec![1, 2, 3, 4, 5, 6, 7])
    //     );
    // }
}
