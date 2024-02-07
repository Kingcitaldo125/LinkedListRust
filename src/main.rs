struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn add(head:Option<Box<Node>>, dat:i32) -> Option<Box<Node>> {
    let mut hnode = Box::new(Node{
        data: dat,
        next: None,
    });

    let valid_head = match head {
        None => false,
        Some(_) => true,
    };

    if valid_head {
        hnode.next = head;
    }

    Some(hnode)
}

fn traverse(head:Option<Box<Node>>) {
    let mut h = head;

    if h.is_none() {
        return;
    }

    loop {
        match h {
            None => return,
            Some(ref headptr) => println!("{} ", headptr.data),
        }

        h = h.unwrap().next;
    }
}

fn main()
{
    let mut head:Option<Box<Node>> = None;

    head = add(add(add(head, 3), 2), 1);

    traverse(head);
}
