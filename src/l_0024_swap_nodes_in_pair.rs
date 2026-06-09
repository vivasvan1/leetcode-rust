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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // match head {
        //     Some(mut first) => match first.next.take() {
        //         Some(_) => {
        //             todo!()
        //         }
        //         None => Some(first),
        //     },
        //     None => None,
        // }
        let mut head = head;

        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut current = &mut head;

        while current.is_some() && current.as_ref().unwrap().next.is_some() {

            let mut first  = current.take().unwrap();
            let mut second = first.next.take().unwrap();
            let third = second.next.take();

            first.next = third;
            second.next = Some(first);

            *current = Some(second);

            current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        return head;
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

    #[test]
    fn test_empty_list() {
        // Input: []
        // Output: []
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
    }

    #[test]
    fn test_single_element() {
        // Input: [1]
        // Output: [1]
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
    }

    #[test]
    fn test_even_number_of_elements() {
        // Input: [1, 2, 3, 4]
        // Output: [2, 1, 4, 3]
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
    }

    #[test]
    fn test_odd_number_of_elements() {
        // Input: [1, 2, 3, 4, 5]
        // Output: [2, 1, 4, 3, 5]
        // The last element '5' should remain in place.
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4, 5])),
            to_list(vec![2, 1, 4, 3, 5])
        );
    }
}
