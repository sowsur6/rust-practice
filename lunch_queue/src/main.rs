#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;
#[derive(Debug)]
pub struct Person {
    
    pub name: String,
    pub discount: i32,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let new_node = Box::new(Person {
            name,
            discount,
            next_person: self.node.take(),
        });

        self.node = Some(new_node);
    }

    pub fn invert_queue(&mut self) {
        let mut current_node = self.node.take();
        let mut prev_node = None;

        while let Some(mut boxed_node) = current_node {
            let next_node = boxed_node.next_person.take();
            boxed_node.next_person = prev_node.take();
            prev_node = Some(boxed_node);
            current_node = next_node;
        }

        self.node = prev_node;
    }

    // pub fn rm(&mut self) -> Option<(String, i32)> {
    //     // let mut current_node = self.node.take();
    //     // let mut prev_node = None;

    //     // while let Some(mut boxed_node) = current_node {
    //     //     let next_node = boxed_node.next_person.take();
    //     //     boxed_node.next_person = prev_node.take();
    //     //     prev_node = Some(boxed_node);
    //     //     current_node = next_node;
    //     // }

    //     // self.node = prev_node;

    //     self.node.take().map(|person| {
    //         self.node = person.next_person;
    //         (person.name, person.discount)
    //     })

    // }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut current_node = self.node.take();
        let mut prev_node = None;
    
        while let Some(mut boxed_node) = current_node {
            let next_node = boxed_node.next_person.take();
            boxed_node.next_person = prev_node.take();
            prev_node = Some(boxed_node);
            current_node = next_node;
        }
    
        self.node = prev_node;
    
        let result = self.node.take().map(|person| {
            self.node = person.next_person;
            (person.name, person.discount)
        });
    
        // Reverse the linked list again
        let mut current_node = self.node.take();
        let mut prev_node = None;
    
        while let Some(mut boxed_node) = current_node {
            let next_node = boxed_node.next_person.take();
            boxed_node.next_person = prev_node.take();
            prev_node = Some(boxed_node);
            current_node = next_node;
        }
    
        self.node = prev_node;
    
        result
    }
    

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current_node = &self.node;
        while let Some(node) = current_node {
            if node.name == name {
                return Some((node.name.clone(), node.discount));
            }
            current_node = &node.next_person;
        }
        None
    }
}





fn main() {
    let mut list = Queue::new();
    list.add(String::from("Marie"), 20);
    list.add(String::from("Monica"), 15);
    list.add(String::from("Ana"), 5);
    list.add(String::from("Alice"), 35);
    println!("{:?}", list);

    println!("{:?}", list.search("Marie"));
    println!("{:?}", list.search("Alice"));
    println!("{:?}", list.search("someone"));

    println!("removed {:?}", list.rm());
    println!("list {:?}", list);
    list.invert_queue();
    println!("invert {:?}", list);
}



// Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Marie", discount: 20, next_person: None }) }) }) }) }
// Some(("Marie", 20))
// Some(("Alice", 35))
// None
// removed Some(("Marie", 20))
// list Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: None }) }) }) }
// invert Queue { node: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Alice", discount: 35, next_person: None }) }) }) }


// $ cargo run
// Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Marie", discount: 20, next_person: None }) }) }) }) }
// Some(("Marie", 20))
// Some(("Alice", 35))
// None
// removed Some(("Marie", 20))
// list Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: None }) }) }) }
// invert Queue { node: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Alice", discount: 35, next_person: None }) }) }) }
// $

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = Queue::new();
        assert!(list.node.is_none());
    }

    #[test]
    fn test_one_person() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 14);
        list.rm();
        assert!(list.node.is_none());
    }

    #[test]
    fn test_two_person() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 13);
        list.add(String::from("Monica"), 54);
        list.rm();

        assert_eq!(list.node.as_ref().unwrap().name, "Monica");
        assert_eq!(list.node.as_ref().unwrap().discount, 54);
    }

    #[test]
    fn test_more_person() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);
        list.rm();

        assert_eq!(list.node.as_ref().unwrap().name, "Alice");
        assert_eq!(list.node.as_ref().unwrap().discount, 35);

        list.rm();
        list.rm();
        assert_eq!(list.node.as_ref().unwrap().name, "Alice");
        assert_eq!(list.node.as_ref().unwrap().discount, 35);
    }

    #[test]
    fn test_search() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);

        assert_eq!(list.search("Ana"), Some((String::from("Ana"), 5)));

        assert_eq!(list.search("Monica"), Some((String::from("Monica"), 15)));

        assert_eq!(list.search("Alice"), Some((String::from("Alice"), 35)));

        assert_eq!(list.search("someone_that_does_not_exist"), None);
    }

    #[test]
    fn test_invert() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);

        list.invert_queue();
        assert_eq!(list.node.as_ref().unwrap().name, "Marie");
        assert_eq!(list.node.as_ref().unwrap().discount, 20);

        list.rm();
        list.invert_queue();
        assert_eq!(list.node.as_ref().unwrap().name, "Ana");
        assert_eq!(list.node.as_ref().unwrap().discount, 5);
    }
}
