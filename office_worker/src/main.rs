use std::convert::From;

#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(',').collect();
        let name = parts[0].to_string();
        let age = parts[1].parse().unwrap();
        let role = WorkerRole::from(parts[2]);
        Self { name, age, role }
    }
}

impl From<&str> for WorkerRole {
    fn from(value: &str) -> Self {
        match value {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            "guest" => WorkerRole::Guest,
            _ => panic!("Invalid worker role"),
        }
    }
}

fn main() {
    println!("New worker: {:?}",
        OfficeWorker::from("Manuel,23,admin"));
    println!("New worker: {:?}",
        OfficeWorker::from("Jean Jacques,44,guest"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_office_worker() {
        assert_eq!(
            OfficeWorker::from("Louise,25,admin"),
            OfficeWorker {
                name: "Louise".to_string(),
                age: 25,
                role: WorkerRole::Admin,
            }
        );
        assert_eq!(
            OfficeWorker::from("Rob,11,guest"),
            OfficeWorker {
                name: "Rob".to_string(),
                age: 11,
                role: WorkerRole::Guest,
            }
        );
        assert_eq!(
            OfficeWorker::from("Maria Agata,44,user"),
            OfficeWorker {
                name: "Maria Agata".to_string(),
                age: 44,
                role: WorkerRole::User,
            }
        );
    }

    #[test]
    fn test_worker_role() {
        assert_eq!(WorkerRole::from("guest"), WorkerRole::Guest);
        assert_eq!(WorkerRole::from("admin"), WorkerRole::Admin);
        assert_eq!(WorkerRole::from("user"), WorkerRole::User);
    }
}