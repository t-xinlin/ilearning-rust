/*
给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。

如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。

您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例：

输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出：7 -> 0 -> 8
原因：342 + 465 = 807
*/

use std::cmp::Ordering;
use std::collections::LinkedList;

// enum L {
//     Val(i32, Box<L>),
//     Nil,
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(v: i32) -> ListNode {
        ListNode { val: v, next: None }
    }

    fn append(self, e: i32) -> Self {
        ListNode {
            val: e,
            next: Some(Box::new(self)),
        }
    }

    fn len(&self) -> i32 {
        match &(*self).next {
            Some(t) => 1 + t.len(),
            None => 1,
        }
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &ListNode) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut tmp1 = l1;
    let mut tmp2 = l2;
    let mut carry = 0;
    let mut c = ListNode::new(0);

    while tmp1.is_some() || tmp2.is_some() {
        let mut sum = carry;
        if let Some(t) = tmp1 {
            println!("===1 {}", t.val);
            sum += t.val;
            tmp1 = t.next;
        }
        if let Some(t) = tmp2 {
            println!("===2 {}", t.val);
            sum += t.val;
            tmp2 = t.next;
        }
        carry = sum / 10;
        c = c.append(sum % 10);
        println!("======carry {}", carry);
    }
    if carry > 0 {
        println!("======{}", carry);
        c = c.append(carry);
    }

    let out = Some(Box::new(c));
    out.unwrap().next
}
pub fn add_two_numbers_by_linklist(l1: LinkedList<i32>, l2: LinkedList<i32>) -> LinkedList<i32> {
    let mut tmp1 = l1;
    let mut tmp2 = l2;
    let mut carry = 0;
    let mut c = LinkedList::new();

    while !tmp1.is_empty() && !tmp2.is_empty() {
        let mut sum = carry;
        sum += tmp1.pop_back().unwrap();
        sum += tmp2.pop_back().unwrap();

        carry = sum / 10;
        c.push_front(sum % 10);
        println!("======carry {}", carry);
    }
    if carry > 0 {
        println!("======{}", carry);
        c.push_front(carry);
    }
    c
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        use std::collections::*;
        let mut link_list1 = LinkedList::new();
        link_list1.push_front(1);
        link_list1.push_front(6);
        link_list1.push_front(8);

        let mut link_list2 = LinkedList::new();
        link_list2.push_front(1);
        link_list2.push_front(6);
        link_list2.push_front(8);
        println!("=========link1 sum: {:?}", link_list1);
        println!("=========link2 sum: {:?}", link_list2);
        let sum = add_two_numbers_by_linklist(link_list1, link_list2);

        println!("=========link sum: {:?}", sum);

        let mut list1 = ListNode::new(1);
        list1 = list1.append(2);
        list1 = list1.append(8);
        let length = list1.len();
        // assert_eq!(3, length);
        println!("======length: {}", length);

        println!("===list1: {:?}", list1);

        let mut list2 = ListNode::new(1);
        // list1.append(1);
        list2 = list2.append(2);
        list2 = list2.append(8);

        let retrun_list = add_two_numbers(Some(Box::new(list2)), Some(Box::new(list1)));
        println!("===return list: {:?}", retrun_list);

        let mut cp_list = ListNode::new(2);
        cp_list = cp_list.append(4);
        cp_list = cp_list.append(2);

        assert_eq!(Some(Box::new(cp_list)), retrun_list);
    }
}
