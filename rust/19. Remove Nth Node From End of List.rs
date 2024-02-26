#[allow(invalid_reference_casting)]
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut fast = head.as_ref()?;

    for _ in 0..n {
        fast = fast.next.as_ref().unwrap();
    }

    let mut slow = head.as_ref()?;

    while fast.next.is_some() {
        fast = fast.next.as_ref().unwrap();
        slow = slow.next.as_ref().unwrap();
    }

    let mut slow = unsafe {  &mut*(slow as *const _ as *mut Box<ListNode>) };

    let next = slow.next.take().unwrap().next;
    slow.next = next;

    head?.next
}