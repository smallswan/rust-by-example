use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use std::thread;

struct Owner {
    name: String,
    // ...other fields
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
    // ...other fields
}

#[test]
fn arc_demo() {
    let five = Arc::new(5);
    for _ in 0..10 {
        let five = Arc::clone(&five);
        thread::spawn(move || {
            println!("{:?}", five);
        });
    }

    let val = Arc::new(AtomicUsize::new(5));
    for _ in 0..10 {
        let val = Arc::clone(&val);
        thread::spawn(move || {
            let v = val.fetch_add(1, Ordering::SeqCst);
            println!("{:?}", v);
        });
    }

    // Create a reference-counted `Owner`.
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string(),
        gadgets: RefCell::new(vec![]),
    });

    // Create `Gadget`s belonging to `gadget_owner`. Cloning the `Rc<Owner>`
    // gives us a new pointer to the same `Owner` allocation, incrementing
    // the reference count in the process.
    let gadget1 = Rc::new(Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    });
    let gadget2 = Rc::new(Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    });

    // Add the `Gadget`s to their `Owner`.
    {
        let mut gadgets = gadget_owner.gadgets.borrow_mut();
        gadgets.push(Rc::downgrade(&gadget1));
        gadgets.push(Rc::downgrade(&gadget2));

        // `RefCell` dynamic borrow ends here.
    }

    // Iterate over our `Gadget`s, printing their details out.
    for gadget_weak in gadget_owner.gadgets.borrow().iter() {
        // `gadget_weak` is a `Weak<Gadget>`. Since `Weak` pointers can't
        // guarantee the allocation still exists, we need to call
        // `upgrade`, which returns an `Option<Rc<Gadget>>`.
        //
        // In this case we know the allocation still exists, so we simply
        // `unwrap` the `Option`. In a more complicated program, you might
        // need graceful error handling for a `None` result.

        let gadget = gadget_weak.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }

    drop(gadget_owner);

    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    let mut my_number = dbg!(9);
    dbg!(my_number += 10);

    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
    dbg!(double_vec);

    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
        .iter()
        .inspect(|first_item| println!("The item is: {}", first_item))
        .map(|x| x * 2)
        .inspect(|next_item| println!("Then it is: {}", next_item))
        .collect::<Vec<i32>>();
}
