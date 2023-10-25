use std::fmt;

pub struct Park {
    name: String,
    park_type: ParkType,
    address: String,
    cap: String,
    state: String,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}, {}, {} - {}",
            self.park_type,
            display_empty(&self.name, "No name"),
            display_empty(&self.address, "No address"),
            display_empty(&self.cap, "No cap"),
            display_empty(&self.state, "No state")
        )
    }
}

fn display_empty(value: &str, empty_print: &str) -> String {
    if value.is_empty() {
        empty_print.to_string()
    } else {
        value.to_string()
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParkType::Garden => write!(f, "garden"),
            ParkType::Forest => write!(f, "forest"),
            ParkType::Playground => write!(f, "playground"),
        }
    }
}

fn main() {
    println!(
        "{}",
        Park {
            name: "Les Tuileries".to_string(),
            park_type: ParkType::Garden,
            address: "Pl. de la Concorde".to_string(),
            cap: "75001".to_string(),
            state: "France".to_string()
        }
    );
    println!(
        "{}",
        Park {
            name: "".to_string(),
            park_type: ParkType::Playground,
            address: "".to_string(),
            cap: "".to_string(),
            state: "".to_string()
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_park() {
        let park = Park {
            name: "Central Park".to_string(),
            park_type: ParkType::Garden,
            address: "Av. Sidónio Pais 4".to_string(),
            cap: "1050-214".to_string(),
            state: "Portugal".to_string(),
        };

        assert_eq!(
            park.to_string(),
            "garden - Central Park, Av. Sidónio Pais 4, 1050-214 - Portugal"
        );
    }

    #[test]
    fn test_empty_name() {
        let park = Park {
            name: "".to_string(),
            park_type: ParkType::Forest,
            address: "Av. Sidónio Pais 4".to_string(),
            cap: "1050-214".to_string(),
            state: "Portugal".to_string(),
        };

        assert_eq!(
            park.to_string(),
            "forest - No name, Av. Sidónio Pais 4, 1050-214 - Portugal"
        );
    }

    #[test]
    fn test_empty_all() {
        let park = Park {
            name: "".to_string(),
            park_type: ParkType::Playground,
            address: "".to_string(),
            cap: "".to_string(),
            state: "".to_string(),
        };

        assert_eq!(
            park.to_string(),
            "playground - No name, No address, No cap - No state"
        );
    }
}