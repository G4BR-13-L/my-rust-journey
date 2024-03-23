#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
  let mut current = head;
  let mut versed = Vec::new();

  while let Some(mut node) = current {
    versed.push(node.val);
    let next_node = node.next.take();
    current = next_node;
  }

  let len = versed.len();
  let mut i = 0;
  let mut j = len - 1;

  while i < j {
    if versed[i] != versed[j] {
      return false;
    }
    i += 1;
    j -= 1;
  }

  true
}
