// solution to run locally
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>,
}

impl ListNode {
	#[inline]
	pub fn new(val: i32) -> Self {
		ListNode {
			next: None,
			val,
		}
	}
}

pub struct Solution;

impl Solution {
	pub fn add_two_numbers(
		l1: Option<Box<ListNode>>,
		l2: Option<Box<ListNode>>,
	) -> Option<Box<ListNode>> {
		let mut p = l1;
		let mut q = l2;
		let mut carry = 0;
		let mut current = &mut head;

		while p.is_some() || q.is_some() {
			let sum = carry
				+ p.as_ref().map_or(0, | node | node.val)
				+ q.as_ref().map_or(0, | node | node.val);

			carry = sum / 10;

			if let Some(node) = current {
				node.next = Some(Box::new(ListNode::new(sum %  10)));
				current = &mut node.next;
			}

			p = if let Some(node) = p { node.next } else { None };
			q = if let Some(node) = q { node.next } else { None };
		}

		if carry > 0 {
			if let Some(node) = current {
				node.next = Some(Box::new(ListNode::new(carry)));
			}
		}

		head.unwrap().next
	}
}

// helper function to convert vector to linked list
pub fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
	let mut head = None;
	for &val in vec.iter().rev() {
		let mut new_node = Box::new(ListNode::new(val));
		new_node.next = head;
		head = Some(new_node);
	}
	head
}

// helper function to convert linked list back to vector
pub fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
	let mut vec = Vec::new();
	let mut current = list;
	while let Some(node) = current {
		vec.push(node.val);
		current = node.next;
	}
	vec
}

// solution to submit

impl Solution {
	pub fn add_two_numbers(
		l1: Option<Box<ListNode>>,
		l2: Option<Box<ListNode>>,
	) -> Option<Box<ListNode>> {
		let mut p = l1;
		let mut q = l2;
		let mut carry = 0;
		let mut head = Some(Box::new(ListNode::new(0)));
		let mut current = &mut head;

		while p.is_some() || q.is_some() {
			let sum = carry
				+ p.as_ref().map_or(0, |node| node.val)
				+ q.as_ref().map_or(0, |node| node.val);

			carry = sum / 10;

			if let Some(node) = current {
				node.next = Some(Box::new(ListNode::new(sum % 10)));
				current = &mut node.next;
			}

			p = if let Some(node) = p { node.next } else { None };
			q = if let Some(node) = q { node.next } else { None };
		}

		if carry > 0 {
			if let Some(node) = current {
				node.next = Some(Box::new(ListNode::new(carry)));
			}
		}

		head.unwrap().next
	}
}