pub mod library {
    pub mod writers {
        use crate::library::books::Book;

        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }

    pub mod books {
        pub struct Book {
            pub title: String,
            pub year: u64,
        }
    }
}

use library::writers::Writer;
use library::books::Book;

pub fn order_books(writer: &mut Writer) {
    writer.books.sort_by(|a, b| a.title.cmp(&b.title));
}

fn main() {
    let mut writer_a = Writer {
        first_name: "William".to_string(),
        last_name: "Shakespeare".to_string(),
        books: vec![
            Book {
                title: "Hamlet".to_string(),
                year: 1600,
            },
            Book {
                title: "Othelo".to_string(),
                year: 1603,
            },
            Book {
                title: "Romeo and Juliet".to_string(),
                year: 1593,
            },
            Book {
                title: "MacBeth".to_string(),
                year: 1605,
            },
        ],
    };

    println!("Before ordering");
    for b in &writer_a.books {
        println!("{:?}", b.title);
    }

    order_books(&mut writer_a);

    println!("\nAfter ordering");
    for b in writer_a.books {
        println!("{:?}", b.title);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w_shakespeare() {
        let mut writer = Writer {
            first_name: "William".to_string(),
            last_name: "Shakespeare".to_string(),
            books: vec![
                Book {
                    title: "Hamlet".to_string(),
                    year: 1600,
                },
                Book {
                    title: "Othelo".to_string(),
                    year: 1603,
                },
                Book {
                    title: "Romeo and Juliet".to_string(),
                    year: 1593,
                },
                Book {
                    title: "MacBeth".to_string(),
                    year: 1605,
                },
            ],
        };

        order_books(&mut writer);

        assert_eq!("Hamlet", writer.books[0].title);
        assert_eq!("MacBeth", writer.books[1].title);
        assert_eq!("Othelo", writer.books[2].title);
        assert_eq!("Romeo and Juliet", writer.books[3].title);
    }
    #[test]
    fn j_k_rowling() {
        let mut writer = Writer {
            first_name: "William".to_string(),
            last_name: "Shakespeare".to_string(),
            books: vec![
                Book {
                    title: "Harry Potter and the Philosopher's Stone".to_string(),
                    year: 1997,
                },
                Book {
                    title: "Harry Potter and the Prisoner of Azkaban".to_string(),
                    year: 1999,
                },
                Book {
                    title: "Harry Potter and the Order of the Phoenix".to_string(),
                    year: 2003,
                },
                Book {
                    title: "Harry Potter and the Chamber of Secrets".to_string(),
                    year: 1998,
                },
                Book {
                    title: "Harry Potter and the Deathly Hallows".to_string(),
                    year: 2007,
                },
            ],
        };

        order_books(&mut writer);

        assert_eq!(
            "Harry Potter and the Chamber of Secrets",
            writer.books[0].title
        );
        assert_eq!(
            "Harry Potter and the Deathly Hallows",
            writer.books[1].title
        );
        assert_eq!(
            "Harry Potter and the Order of the Phoenix",
            writer.books[2].title
        );
        assert_eq!(
            "Harry Potter and the Philosopher's Stone",
            writer.books[3].title
        );
        assert_eq!(
            "Harry Potter and the Prisoner of Azkaban",
            writer.books[4].title
        );
    }
}