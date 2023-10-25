// use std::ops::Add;

// #[derive(Debug)]
// struct Garage<T: Add<Output = T> + Copy> {
//     left: Option<T>,
//     right: Option<T>,
// }

// impl<T: Add<Output = T> + Copy> Garage<T> {
//     fn move_to_right(&mut self) {
//         if let Some(left_val) = self.left {
//             if let Some(right_val) = self.right {
//                 self.right = Some(left_val + right_val);
//             } else {
//                 self.right = Some(left_val);
//             }
//             self.left = None;
//         }
//     }

//     fn move_to_left(&mut self) {
//         if let Some(right_val) = self.right {
//             if let Some(left_val) = self.left {
//                 self.left = Some(right_val + left_val);
//             } else {
//                 self.left = Some(right_val);
//             }
//             self.right = None;
//         }
//     }
// }

fn main() {
    let mut garage = Garage {
        left: Some(5),
        right: Some(2),
    };

    println!("{:?}", garage);
    garage.move_to_right();
    println!("{:?}", garage);
    garage.move_to_left();
    println!("{:?}", garage);
}



use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Garage<T: Add<Output = T> + Copy + std::default::Default> {
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T: Add<Output = T> + Copy + std::default::Default> Garage<T> {
    pub fn move_to_right(&mut self) {
        if let Some(value) = self.left {
            self.left = None;
            self.right = Some(self.right.unwrap_or_default() + value);
        }
    }

    pub fn move_to_left(&mut self) {
        if let Some(value) = self.right {
            self.right = None;
            self.left = Some(self.left.unwrap_or_default() + value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_to_right() {
        let mut garage_int = Garage {
            left: Some(5),
            right: Some(2),
        };

        garage_int.move_to_right();
        assert_eq!(
            garage_int,
            Garage {
                left: None,
                right: Some(7)
            }
        );
        garage_int.move_to_right();
        assert_eq!(
            garage_int,
            Garage {
                left: None,
                right: Some(7)
            }
        );

        let mut garage_float = Garage {
            left: Some(4.25),
            right: Some(1.11),
        };

        garage_float.move_to_right();
        assert_eq!(
            garage_float,
            Garage {
                left: None,
                right: Some(5.36)
            }
        );
    }

    #[test]
    fn test_move_to_left() {
        let mut garage_int = Garage {
            left: Some(10),
            right: Some(2),
        };

        garage_int.move_to_left();
        assert_eq!(
            garage_int,
            Garage {
                left: Some(12),
                right: None
            }
        );
        garage_int.move_to_left();
        assert_eq!(
            garage_int,
            Garage {
                left: Some(12),
                right: None
            }
        );

        let mut garage_float = Garage {
            left: Some(4.25),
            right: Some(1.11),
        };

        garage_float.move_to_left();
        assert_eq!(
            garage_float,
            Garage {
                left: Some(5.36),
                right: None
            }
        );
    }
}