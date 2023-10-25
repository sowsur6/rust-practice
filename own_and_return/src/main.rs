pub struct Film {
    pub name: String,
}

pub fn read_film_name(film: &Film) -> String {
    film.name.clone()
}

pub fn take_film_name(film: Film) -> String {
    film.name
}

fn main() {
    let my_film = Film { name: "Terminator".to_string() };
    // println!("{}", take_film_name(my_film));
    // the order of the print statements is intentional, if your implementation is correct,
    // you should have a compile error because my_film was consumed
    println!("{}", read_film_name(&my_film));
    println!("{}", take_film_name(Film { name: "Terminator".to_string() }))
    // you can test this function by commenting out the first print statement,
    // you should see the expected output without errors in this case
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume() {
        assert_eq!(
            take_film_name(Film {
                name: "Matrix".to_string()
            }),
            "Matrix".to_string()
        );
    }
    #[test]
    fn test_only_print() {
        assert_eq!(
            read_film_name(&Film {
                name: "Matrix".to_string()
            }),
            "Matrix".to_string()
        )
    }
}