
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

// impl BloodType {
//     pub fn can_receive_from(&self, other: &Self) -> bool {
//         match self.rh_factor {
//             RhFactor::Positive => {
//                 match other.rh_factor {
//                     RhFactor::Positive => {
//                         (self.antigen == Antigen::O && other.antigen == Antigen::O) ||
//                         (self.antigen == Antigen::A && (other.antigen == Antigen::O || other.antigen == Antigen::A)) ||
//                         (self.antigen == Antigen::B && (other.antigen == Antigen::O || other.antigen == Antigen::B)) ||
//                         (self.antigen == Antigen::AB && (other.antigen == Antigen::O || other.antigen == Antigen::A || other.antigen == Antigen::B || other.antigen == Antigen::AB))
//                     },
//                     RhFactor::Negative => {
//                         (self.antigen == Antigen::O && (other.antigen == Antigen::O || other.antigen == Antigen::A || other.antigen == Antigen::B || other.antigen == Antigen::AB)) ||
//                         (self.antigen == Antigen::A && (other.antigen == Antigen::O || other.antigen == Antigen::A)) ||
//                         (self.antigen == Antigen::B && (other.antigen == Antigen::O || other.antigen == Antigen::B)) ||
//                         (self.antigen == Antigen::AB && (other.antigen == Antigen::O || other.antigen == Antigen::A || other.antigen == Antigen::B))
//                     }
//                 }
//             },
//             RhFactor::Negative => {
//                 match other.rh_factor {
//                     RhFactor::Positive => {
//                         false
//                     },
//                     RhFactor::Negative => {
//                         self.antigen == other.antigen && (self.antigen == Antigen::O || self.antigen == Antigen::A || self.antigen == Antigen::B || self.antigen == Antigen::AB)
//                     }
//                 }
//             }
//         }
//     }
//     pub fn donors(&self) -> Vec<Self> {
//         match self.rh_factor {
//             RhFactor::Positive => {
//                 match self.antigen {
//                     Antigen::O => vec![BloodType{ antigen: Antigen::O, rh_factor: RhFactor::Positive }, BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative }],
//                     Antigen::A => vec![BloodType{ antigen: Antigen::A, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }],
//                     Antigen::B => vec![BloodType{ antigen: Antigen::B, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }],
//                     Antigen::AB => vec![BloodType{ antigen: Antigen::A, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::B, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }],
//                 }
//             },
//             RhFactor::Negative => {
//                 match self.antigen {
//                     Antigen::O => vec![BloodType{ antigen: Antigen::O, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::O, rh_factor: RhFactor::Negative }],
//                     Antigen::A => vec![BloodType{ antigen: Antigen::A, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::A, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Negative }],
//                     Antigen::B => vec![BloodType{ antigen: Antigen::B, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::B, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Negative }],
//                     Antigen::AB => vec![BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Negative }],
//                 }
//             }
//         }
//     }

//     pub fn recipients(&self) -> Vec<Self> {
//         match self.rh_factor {
//             RhFactor::Positive => {
//                 match self.antigen {
//                     Antigen::O => vec![BloodType{ antigen: Antigen::O, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::A, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::B, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }],
//                     Antigen::A => vec![BloodType{ antigen: Antigen::A, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }],
//                     Antigen::B => vec![BloodType{ antigen: Antigen::B, rh_factor: RhFactor::Positive }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }],
//                     Antigen::AB => vec![BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Positive }],
//                 }
//             },
//             RhFactor::Negative => {
//                 match self.antigen {
//                     Antigen::O => vec![BloodType{ antigen: Antigen::O, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::A, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::B, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Negative }],
//                     Antigen::A => vec![BloodType{ antigen: Antigen::O, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::A, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Negative }],
//                     Antigen::B => vec![BloodType{ antigen: Antigen::O, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::B, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Negative }],
//                     Antigen::AB => vec![BloodType{ antigen: Antigen::O, rh_factor: RhFactor::Negative }, BloodType{ antigen: Antigen::AB, rh_factor: RhFactor::Negative }],
//                 }
//             },
//         }
//     }
    
// }
impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        match self.antigen.clone() {
            Antigen::AB => match self.rh_factor.clone() {
                RhFactor::Positive => true,
                RhFactor::Negative => match (&other.antigen, &other.rh_factor) {
                    (Antigen::O, RhFactor::Negative) => true,
                    (Antigen::B, RhFactor::Negative) => true,
                    (Antigen::A, RhFactor::Negative) => true,
                    (Antigen::AB, RhFactor::Negative) => true,
                    _ => false,
                },
            },
            Antigen::A => match self.rh_factor.clone() {
                RhFactor::Positive => match (other.antigen.clone(), other.rh_factor.clone()) {
                    (Antigen::O, RhFactor::Negative) => true,
                    (Antigen::O, RhFactor::Positive) => true,
                    (Antigen::A, RhFactor::Negative) => true,
                    (Antigen::A, RhFactor::Positive) => true,
                    _ => false,
                },
                RhFactor::Negative => match (other.antigen.clone(), other.rh_factor.clone()) {
                    (Antigen::O, RhFactor::Negative) => true,
                    (Antigen::A, RhFactor::Negative) => true,
                    _ => false,
                },
            },
            Antigen::B => match self.rh_factor.clone() {
                RhFactor::Positive => match (other.antigen.clone(), other.rh_factor.clone()) {
                    (Antigen::O, RhFactor::Negative) => true,
                    (Antigen::O, RhFactor::Positive) => true,
                    (Antigen::B, RhFactor::Negative) => true,
                    (Antigen::B, RhFactor::Positive) => true,
                    _ => false,
                },
                RhFactor::Negative => match (other.antigen.clone(), other.rh_factor.clone()) {
                    (Antigen::O, RhFactor::Negative) => true,
                    (Antigen::B, RhFactor::Negative) => true,
                    _ => false,
                },
            },
            Antigen::O => match self.rh_factor.clone() {
                RhFactor::Positive => match (other.antigen.clone(), other.rh_factor.clone()) {
                    (Antigen::O, RhFactor::Negative) => true,
                    (Antigen::O, RhFactor::Positive) => true,
                    _ => false,
                },
                RhFactor::Negative => match (other.antigen.clone(), other.rh_factor.clone()) {
                    (Antigen::O, RhFactor::Negative) => true,
                    _ => false,
                },
            },
        }
    }
    pub fn recipients(&self) -> Vec<Self> {
        let mut new_vec:Vec<BloodType>=Vec::new();
        match (self.antigen.clone(),self.rh_factor.clone()){
            (Antigen::AB,RhFactor::Positive)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Positive});
                // return new_vec;
            },
            (Antigen::AB, RhFactor::Negative)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Negative});
                // return new_vec;
            },
            (Antigen::A,RhFactor::Positive)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Positive});
                // return new_vec;
            },
            (Antigen::A,RhFactor::Negative)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Negative});
                // return new_vec;
            },
            (Antigen::B,RhFactor::Positive)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Positive});
                // return new_vec;
            },
            (Antigen::B,RhFactor::Negative)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Negative});
                // return new_vec;
            },
            (Antigen::O,RhFactor::Positive)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Positive});
                // return new_vec;
            },
            (Antigen::O,RhFactor::Negative)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Negative});
                // return new_vec;
            },

        };
        new_vec
    
    }
    pub fn donors(&self) -> Vec<BloodType> {
        let mut new_vec:Vec<BloodType>=Vec::new();
        match (self.antigen.clone(),self.rh_factor.clone()){
            (Antigen::O,RhFactor::Negative)=>
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Negative})
                // return new_vec;
            ,
            (Antigen::O,RhFactor::Positive)=>{
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Negative});
                // return new_vec;
                
            },
            (Antigen::A,RhFactor::Negative)=>{
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Negative});
                // return new_vec;
            },
            (Antigen::A,RhFactor::Positive)=>{
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Negative});
                // return new_vec;
            },
            (Antigen::B,RhFactor::Negative)=>{
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Negative});
                // return new_vec;
            },
            (Antigen::B,RhFactor::Positive)=>{
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Negative});
                // return new_vec;
            },
            (Antigen::AB,RhFactor::Negative)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Negative});
                // return new_vec;
            },
            (Antigen::AB,RhFactor::Positive)=>{
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Positive});
                new_vec.push(BloodType{antigen:Antigen::AB, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::A, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::B, rh_factor:RhFactor::Negative});
                new_vec.push(BloodType{antigen:Antigen::O, rh_factor:RhFactor::Negative});
                // return new_vec;
            },
        };
        new_vec
    }
}



fn main() {
	let blood_type = BloodType {
		antigen: Antigen::O,
		rh_factor: RhFactor::Positive,
	};
	println!("recipients of O+ {:?}", blood_type.recipients());
	println!("donors of O+ {:?}", blood_type.donors());
	let another_blood_type = BloodType {
		antigen: Antigen::O,
		rh_factor: RhFactor::Positive,
	};
	println!(
		"donors of O+ can receive from {:?} {:?}",
		&another_blood_type,
		blood_type.can_receive_from(&another_blood_type)
	);
}


// recipients of O+ [BloodType { antigen: AB, rh_factor: Positive }, BloodType { antigen: O, rh_factor: Positive }, BloodType { antigen: A, rh_factor: Positive }, BloodType { antigen: B, rh_factor: Positive }]
// donors of O+ [BloodType { antigen: O, rh_factor: Positive }, BloodType { antigen: O, rh_factor: Negative }]
// donors of O+ can receive from BloodType { antigen: O, rh_factor: Positive } true
#[cfg(test)]
mod tests {
  //  use blood_types_s::*;
//use crate::BloodType;
use super::*;
    #[test]
    fn compatible_ab_neg_with_a_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_a_pos() {
        let blood_type = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_ab_neg() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_neg_with_o_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_pos_with_o_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_compatible_ab_neg_with_o_neg() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_antigen_ab_from_str() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        assert_eq!(blood_type.antigen, Antigen::AB);
        assert_eq!(blood_type.rh_factor, RhFactor::Positive);
    }

    #[test]
    fn test_antigen_a_from_str() {
        let blood_type = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        assert_eq!(blood_type.antigen, Antigen::A);
        assert_eq!(blood_type.rh_factor, RhFactor::Negative);
    }

    #[test]
    fn test_donors() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let mut givers = blood_type.donors();
        // println!("Before sorting {:?}", &givers);
        givers.sort();
        // println!("{:?}", &givers);
        let mut expected = vec![
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
        ];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_a_neg_donors() {
        let blood = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        let mut givers = blood.donors();
        givers.sort();
        let mut expected = vec![
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
        ];

        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_o_neg_donors() {
        let blood = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        };

        let mut givers = blood.donors();
        givers.sort();
        let mut expected = vec![blood.clone()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_ab_pos_recipients() {
        let blood = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let mut recipients = blood.recipients();
        recipients.sort();
        let mut expected = vec![blood.clone()];
        expected.sort();
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_a_neg_recipients() {
        let blood = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };

        let mut recipients = blood.recipients();
        recipients.sort();
        let mut expected = vec![
            blood.clone(),
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
        ];
        expected.sort();
        assert_eq!(recipients, expected);
    }
}