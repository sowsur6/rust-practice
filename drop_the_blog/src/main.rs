use std::cell::{RefCell, Cell};
  use std::rc::Rc; 
//   remove above line to submit in exam or use it to test
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Blog {
    pub fn new() -> Blog {
		Blog {
			drops: Cell::new(0), 
            states: RefCell::new(Vec::new()), 
		}
	}
    pub fn new_article(&self, body: String) -> (usize, Article) {
		self.states.borrow_mut().push(false);
        let id = self.new_id() - 1;
        return (id, Article::new(id, body, self));
	}
    pub fn new_id(&self) -> usize {
		 return self.states.borrow().len();
	}
	
    pub fn is_dropped(&self, id: usize) -> bool {
		return self.states.borrow()[id];
	}
	
    pub fn add_drop(&self, id: usize) {
		if self.states.borrow_mut()[id]{
            panic!("{} is already dropped", id)
        }else {
            self.states.borrow_mut()[id] = true;
            self.drops.set(self.drops.get() + 1)
        }
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
    pub id: usize,
	pub body: String,
	pub parent: &'a Blog,
}

impl<'a> Article<'a> {
    pub fn new(id: usize, body: String, blog: &'a Blog) -> Article {
		Article {
			id: id,
			body:body,
			parent: blog,
			
		}
	}
    pub fn discard(self) {
		self.parent.add_drop(self.id);
	}
}

fn main() {
    let blog = Blog::new();
    let (id, article) = blog.new_article(String::from("Winter is coming"));
    let (id1, article1) = blog.new_article(String::from("The story of the universe"));

    article.discard();

    println!("{:?}", (blog.is_dropped(id), id, &blog.drops));

    article1.discard();
    println!("{:?}", (blog.is_dropped(id1), id1, &blog.drops));

    let (id2, article2) = blog.new_article(String::from("How to cook 101"));
    let article2 = Rc::new(article2);
    let article2_clone = article2.clone();

    drop(article2_clone);

    println!("{:?}",
      (blog.is_dropped(id2), id2, &blog.drops, Rc::strong_count(&article2)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dropped_and_drops() {
        let blog = Blog::new();
        let (pid, article) = blog.new_article(String::from("gnome-shell"));
        let (pid0, article0) = blog.new_article(String::from("i3"));
        let (pid1, article1) = blog.new_article(String::from("shell"));
        let (pid2, article2) = blog.new_article(String::from("spotify"));

        article.discard();
        assert_eq!(blog.drops.get(), 1_usize);
        article0.discard();
        assert_eq!(blog.drops.get(), 2_usize);

        assert!(blog.is_dropped(pid), "{} should have been dropped", pid);
        assert!(blog.is_dropped(pid0), "{} should have been dropped", pid0);
        assert!(
            !blog.is_dropped(pid1),
            "{} should not have been dropped",
            pid1
        );
        assert!(
            !blog.is_dropped(pid2),
            "{} should not have been dropped",
            pid2
        );

        article1.discard();
        article2.discard();
        assert_eq!(blog.drops.get(), 4_usize);
    }

    #[test]
    fn test_using_rc() {
        // will create a new reference to the article
        // this will test the following
        // if we drop the cloned value the RC will decrease
        // but the article will not be dropped!
        let blog = Blog::new();
        let (_, article) = blog.new_article(String::from("Xorg"));
        let article = Rc::new(article);
        let article_clone = article.clone();

        assert_eq!(Rc::strong_count(&article), 2);

        drop(article_clone);

        assert_eq!(Rc::strong_count(&article), 1);
    }

    #[test]
    #[should_panic(expected = "0 is already dropped")]
    fn test_drop_same_article() {
        // test if we drop the same article after it was already been dropped
        let blog = Blog::new();
        let (_pid, article) = blog.new_article(String::from("gsd-rfkill"));
        let article_clone = article.clone();
        article.discard();
        article_clone.discard();
    }
}