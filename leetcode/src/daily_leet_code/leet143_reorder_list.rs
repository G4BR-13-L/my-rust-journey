#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    fn get_list_middle(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        let mut fast = &head.clone();
        let mut slow = head;

        while fast.is_some() {
            fast = &(fast.as_ref().unwrap().next);
            if fast.is_some() {
                fast = &fast.as_ref().unwrap().next;
                slow = &mut(slow.as_mut().unwrap().next);
            }
        }
        slow.as_mut().unwrap().next.take()
    }
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        let mut previous = None;
        while let Some(mut current) = head {

            head = current.next;
            current.next = previous;
            previous = Some(current);
        }
        previous
    }

    #[inline(always)]
    fn merge_lists(mut head1: &mut Option<Box<ListNode>>, mut head2: Option<Box<ListNode>>) {
        let mut h1 = head1;
        let mut h2 = head2;
        while h1.is_some() && h2.is_some() {
            let mut h1next = h1.as_mut().unwrap().next.take();
            let mut h2next = h2.as_mut().unwrap().next.take();
            h1.as_mut().unwrap().next = h2;
            h1.as_mut().unwrap().next.as_mut().unwrap().next = h1next;
            h1 = &mut(h1.as_mut().unwrap().next.as_mut().unwrap().next);
            h2 = h2next;
        }
        if h2.is_some() {
            h1 = &mut(h2);
        }
    }

    let mut head2 = get_list_middle(head);
    head2 = reverse_list(head2);
    merge_lists(head, head2);
}

