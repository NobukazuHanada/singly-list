use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

pub struct SinglyList<T> {
    head: Link<T>,
    length: usize,
}

struct Node<T> {
    element: T,
    next: Link<T>,
}

pub struct IntoIter<T> {
    next: SinglyList<T>,
}
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<T> SinglyList<T> {
    pub fn new() -> Self {
        SinglyList {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head_node) = self.head.take() {
            self.head = head_node.next;
            self.length -= 1;
            Some(head_node.element)
        } else {
            None
        }
    }

    pub fn insert(&mut self, index: usize, element: T) {
        let mut counter = 0;
        let mut node = self.head.as_mut();

        if index == 0 {
            return self.push(element);
        }

        while let Some(current_node) = node {
            if counter == index - 1 {
                let new_node = Box::new(Node {
                    element,
                    next: current_node.next.take(),
                });
                current_node.next = Some(new_node);
                self.length += 1;
                return;
            }
            node = current_node.next.as_mut();
            counter += 1;
        }
    }

    pub fn delete(&mut self, index: usize) -> Option<T> {
        let mut counter = 0;
        let mut node = self.head.as_mut();

        if index == 0 {
            return self.pop();
        }

        while let Some(current_node) = node {
            if counter == index - 1 {
                let delete_node = current_node.as_mut().next.take();
                if let Some(mut delete_node) = delete_node {
                    let new_node = delete_node.next.take();
                    current_node.next = new_node;
                    self.length -= 0;
                    return Some(delete_node.element);
                }
            }
            node = current_node.next.as_mut();
            counter += 1;
        }
        None
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| node.as_ref()),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| node.as_mut()),
        }
    }
}

impl<T> Default for SinglyList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for SinglyList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { next: self }
    }
}

impl<T: Debug> Debug for SinglyList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut peekable = self.iter().peekable();
        while let Some(element) = peekable.next() {
            write!(f, "{:?}", element)?;
            if peekable.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.next.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| node.as_ref());
            &node.element
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| node.as_mut());
            &mut node.element
        })
    }
}

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
prop_compose! {
    fn vec_and_index()
    (v in prop::collection::vec(".*", 1..100))
    (i in 0..v.len(), v in Just(v) )
    -> (Vec<String>, usize) {
        (v, i)
    }
}

#[cfg(test)]
proptest! {
    #[test]
    fn singly_list_push_test(v in prop::collection::vec(0..500, 0..500)){
        let mut list = SinglyList::new();
        for elem in v.iter().rev() {
            list.push(elem)
        }

        prop_assert_eq!(list.length, v.len());
        prop_assert_eq!(format!("{:?}", list), format!("{:?}",v));
    }

    #[test]
    fn singly_list_pop_test((v, i) in vec_and_index()){
        let mut list = SinglyList::new();
        let mut pop_result : Vec<&String> = vec![];
        for elem in v.iter().rev() {
            list.push(elem)
        }
        for _ in 0..i {
            if let Some(result) = list.pop() {
                pop_result.push(result);
            }
        }
        prop_assert_eq!(list.length, v.len() - i);
        prop_assert_eq!(format!("{:?}", pop_result), format!("{:?}", &v[0..i]));
        prop_assert_eq!(format!("{:?}", list), format!("{:?}", &v[i..]));

        for elem in pop_result.iter().rev() {
            list.push(elem);
        }

        prop_assert_eq!(list.length, v.len());
        prop_assert_eq!(format!("{:?}", list), format!("{:?}",v));
    }

    #[test]
    fn singly_list_insert_test((v, i) in vec_and_index(), v2 in proptest::collection::vec(".*", 0..100)) {
        let mut list = SinglyList::new();
        let mut result_vec = vec![];
        for elem in v.iter().rev() {
            list.push(elem);
        }

        for elem in v.iter() {
            result_vec.push(elem);
        }


        for elem in v2.iter() {
            list.insert(i, elem);
            result_vec.insert(i, elem);
        }

        prop_assert_eq!(list.length, result_vec.len());
        prop_assert_eq!(format!("{:?}", list), format!("{:?}",result_vec ));
    }

    #[test]
    fn singly_list_delete_test(
        head in prop::collection::vec(".*", 0..100),
        delete_body in prop::collection::vec(".*", 0..100),
        tail in prop::collection::vec(".*", 0..100)
    ) {
        let mut clone_head = head.clone();
        let mut clone_delete_body = delete_body.clone();
        let mut clone_tail_body = tail.clone();
        clone_head.append(&mut clone_delete_body);
        clone_head.append(&mut clone_tail_body);

        let mut singly_list = SinglyList::new();
        let mut deleted_elements = vec![];

        for element in clone_head.iter().rev() {
            singly_list.push(element);
        }

        for _ in delete_body.iter() {
            if let Some(elem) = singly_list.delete(head.len()) {
                deleted_elements.push(elem);
            }
        }

        let mut expected_result = head.clone();
        expected_result.append(&mut tail.clone());

        prop_assert_eq!(
            format!("{:?}", singly_list),
            format!("{:?}", expected_result)
        );

        prop_assert_eq!(
            format!("{:?}", deleted_elements),
            format!("{:?}", delete_body)
        );
    }
}
