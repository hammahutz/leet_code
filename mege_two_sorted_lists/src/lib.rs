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
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let meged_list: Option<Box<ListNode>>;
    if let Some(num1) = list1 {
        if let Some(num2) = list2 {
            if num1 < num2 {

            }
        } else {
            // finns inget kvar i list2
        }
    } else {
        // finns inget kvar i list1
        if let Some(num2) = list2 {
        } else {
            // finns inget kvar i list2
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = Some(Box::new(ListNode {val: 1, next: ListNode{val: 2, next: ListNode {val: 3 next: None}}}));
        let list2 = Some(Box::new(ListNode{1, Some(Box::new(ListNode{2, Some(Box::new(ListNode{4,None })) })) })) ;
        let result = merge_two_lists(list1, list2);
        assert_eq!(result, list2);
    }
}
