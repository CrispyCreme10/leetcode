pub mod solution {
    // Definition for singly-linked list.
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

    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {        
        let mut carry = false;
        let mut base_node = ListNode::new(0);
        let curr_node = &mut base_node;
        loop {
            // get l1.val OR 0
            let val1 = match &l1 {
                Some(node) => node.val,
                None => 0
            };

            // get l2.val OR 0
            let val2 = match &l2 {
                Some(node) => node.val,
                None => 0
            };

            // add these 2 values
            let mut total = val1 + val2;

            // add carryover
            if (carry) {
                total += 1;
            }

            // determine if carryover (use modulos)
            carry = total % 10 > 0;

            // create new ListNode with added value
            let new_node = ListNode::new(total);

            // set curr_node's next to new node
            curr_node.next = Some(Box::new(new_node));

            // set curr_node to new node
            curr_node = &mut new_node;

            // if l1 & l2 have no next node, break
            if l1 == None && l2 == None {
                break;
            }
        }

        return Some(Box::new(base_node));
    }

    pub fn tests() {
        
    }
}