use office_worker::*;

fn main() {
    println!("New worker: {:?}", OfficeWorker::from("Manuel,23,admin"));
    println!(
        "New worker: {:?}",
        OfficeWorker::from("Jean Jacques,44,guest")
    );
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
