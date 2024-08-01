/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
//定义链表的节点: 值val和指针next
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}
//new函数:创建节点
impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
//linklist结构体,长度,头尾指针
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
// new方法
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
// add方法
 //这个函数是一个公共的 (pub) 方法，接收一个可变引用 (&mut self) 和一个类型为 T 的对象 obj，其中 T 是泛型参数。它的目的是将 obj 添加到链表中。
    pub fn add(&mut self, obj: T) {
        //创建一个新的 Node 实例，将其包装在 Box 中以进行堆分配。Node::new(obj) 是一个构造函数，用于创建一个新的节点。此节点将包含传入的 obj 值。
        let mut node = Box::new(Node::new(obj));  
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        // let node_ptr 将 Box 转换为原始指针，并通过 NonNull 包装这个原始指针。NonNull 是一个非空的指针类型，它避免了 Option 的开销，
        //同时提供了一些额外的安全性保证。unsafe 代码块是因为 Box::into_raw 会丧失 Rust 的内存安全检查，
        //开发者需要确保这个指针在之后的操作中不会引发问题。= Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        //这个匹配表达式处理了链表的不同状态：
        //如果链表为空 (self.end 为 None)，即链表没有任何元素，那么新节点成为链表的开始节点 (self.start)。
        //如果链表不为空 (self.end 为 Some(end_ptr))，则链表的尾节点（end_ptr 指向的节点）的 next 指针被更新为新节点的指针 (node_ptr)。

        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> LinkedList<T>
	{
		//TODO
        let mut len = 0;
        let mut st = list_a.start;
        let mut en = list_a.end;
        if list_a.length==0 {
            len = list_b.length;
            st = list_b.start;
            en = list_b.end;
        }else if list_b.length==0 {
            len = list_a.length;
            st = list_a.start;
            en = list_a.end;
        }else{
            len = list_a.length + list_b.length;
            st = list_a.start;
            en = list_b.end;
            match list_a.end {
                None => println!("111"),
                Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = list_b.start },
            }
        }
        LinkedList {
            length: len,
            start: st,
            end: en,
        }
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}