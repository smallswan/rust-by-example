//! 跳跃表
//! 代码来源： https://zhuanlan.zhihu.com/p/264629355

use std::cell::RefCell;
use std::rc::Rc;

type RealNode = Rc<RefCell<Node>>;
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct Node {
    data: String,
    next: Vec<Link>,
    offset: u64,
}

impl Node {
    fn new(next: Vec<Link>, offset: u64, data: String) -> RealNode {
        Rc::new(RefCell::new(Node { next, offset, data }))
    }
}

#[derive(Debug, Clone)]
struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    length: u64,
}

impl SkipList {
    fn new(level: usize) -> Self {
        SkipList {
            head: None,
            tails: vec![None; level],
            max_level: level - 1,
            length: 0,
        }
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }

    fn append(&mut self, offset: u64, data: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };
        let node = Node::new(vec![None; level], offset, data);
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(node.clone());
            }
            self.tails[i] = Some(node.clone());
        }

        if self.head.is_none() {
            self.head = Some(node.clone());
        }
        self.length += 1;
    }

    fn level_path(&self) {
        match self.head {
            Some(ref head) => {
                let node = head.clone();

                for level in (0..=self.max_level).rev() {
                    let mut n = node.clone();
                    print!("level={:?} ", level);
                    loop {
                        let next = n.clone();
                        print!(
                            "offset={:?}, data={:?}\t",
                            next.borrow().offset,
                            next.borrow().data
                        );
                        match next.borrow().next[level] {
                            Some(ref next) => {
                                n = next.clone();
                            }
                            _ => break,
                        };
                    }
                    println!("");
                }
            }
            None => {}
        }
    }

    fn find(&self, offset: u64) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level - 1;
                let node = head.clone();
                let mut result = None;

                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref tmp) => {
                                if tmp.borrow().offset <= offset {
                                    n = tmp.clone();
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.data.clone());
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_list() {
        let mut skip_list = SkipList::new(6);
        skip_list.append(1, "hello".to_owned());
        skip_list.append(2, "world".to_owned());
        skip_list.append(3, "it".to_owned());
        skip_list.append(4, "rust".to_owned());
        skip_list.append(5, "java".to_owned());
        skip_list.append(6, "golang".to_owned());
        skip_list.append(7, "C#".to_owned());
        skip_list.level_path();
        println!("offset={}, data={:?}", 5, skip_list.find(5));
    }
}
