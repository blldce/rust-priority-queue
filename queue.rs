use std::{
    alloc::{alloc, Layout},
    fmt::Debug,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Priority {
    LOW = 0,
    MEDIUM,
    HIGH,
}

pub struct Node<'priority_lifetime, T: Debug + PartialEq> {
    data: T,
    priority: &'priority_lifetime Priority,
    previous_node: *mut Node<'priority_lifetime, T>,
}

pub struct Queue<'priority_lifetime, T: Debug + PartialEq> {
    size: u32,
    head_node: *mut Node<'priority_lifetime, T>,
}

impl<'priority_lifetime, T: Debug + PartialEq> Queue<'priority_lifetime, T> {
    pub fn new() -> Queue<'priority_lifetime, T> {
        Queue {
            size: 0,
            head_node: 0 as *mut Node<'priority_lifetime, T>,
        }
    }
    pub unsafe fn enqueue(&mut self, data: T, priority: &'priority_lifetime Priority) -> bool {
        let mut new_node: *mut Node<'priority_lifetime, T>;
        new_node = alloc(Layout::new::<Node<T>>()) as *mut Node<'priority_lifetime, T>;
        (*new_node).data = data;
        (*new_node).priority = priority;

        // first time?
        if self.head_node == (0 as *mut Node<T>) {
            self.head_node = new_node;
        } else {
            let mut lower_priority_node = self.head_node as *mut Node<T>;
            let mut pre_lower_priority_node = lower_priority_node as *mut Node<T>;
            let mut is_head: bool = true; // starting from head
                                          // iterate through greater priorties
            while lower_priority_node != (0 as *mut Node<T>) {
                if priority >= (*lower_priority_node).priority {
                    break;
                }
                is_head = false;
                pre_lower_priority_node = lower_priority_node;
                lower_priority_node = (*lower_priority_node).previous_node;
            }
            if lower_priority_node != (0 as *mut Node<T>) {
                (*new_node).previous_node = lower_priority_node;
                if is_head {
                    self.head_node = new_node;
                } else {
                    (*pre_lower_priority_node).previous_node = new_node;
                }
            } else {
                // there is no lower priority than new node! new node is lowest priority!
                (*pre_lower_priority_node).previous_node = new_node;
            }
        }
        self.size += 1;
        true
    }

    pub unsafe fn dequeue(&mut self) -> bool {
        if self.size == 0 {
            return false;
        }
        if self.size == 1
        // we're in head
        {
            self.head_node = 0 as *mut Node<T>;
            self.size -= 1;
            return true;
        }

        let mut iter = self.head_node as *mut Node<T>;
        let mut prev_node_ptr = iter as *mut Node<T>;

        while (*iter).previous_node != (0 as *mut Node<T>) {
            prev_node_ptr = iter;
            iter = (*iter).previous_node;
        }

        (*prev_node_ptr).previous_node = 0 as *mut Node<T>;
        self.size -= 1;
        return true;
    }

    pub unsafe fn peek(&self, data: T) -> bool {
        if self.size == 0 {
            return false;
        }
        let mut iter = self.head_node as *mut Node<T>;

        while iter != (0 as *mut Node<T>) {
            if (*iter).data == data {
                return true; // founded!
            }

            iter = (*iter).previous_node;
        }
        return false; // not found!
    }

    pub unsafe fn print_items(&self) -> () {
        if self.size == 0 {
            return;
        }
        println!("****************");
        let mut iter = self.head_node as *mut Node<T>;
        while iter != (0 as *mut Node<T>) {
            println!(
                "Address : {:p} -- Priority : {:?} -- Value : {:?} -- Prev : {:p} ",
                iter,
                (*iter).priority,
                (*iter).data,
                (*iter).previous_node
            );
            iter = (*iter).previous_node;
        }
    }
}
