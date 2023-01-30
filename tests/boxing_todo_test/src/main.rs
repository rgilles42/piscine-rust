use boxing_todo::*;

// Note that you can create some todo list your self to test it, but you can find the JSON files that
// are being tested [here](https://github.com/01-edu/public/blob/master/subjects/boxing_todo)
fn main() {
    let todos = TodoList::get_todo("todo.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{} {:?}", e.to_string(), e.source());
        }
    }

    let todos = TodoList::get_todo("malformed_object.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{} {:?}", e.to_string(), e.source());
        }
    }

    let todos = TodoList::get_todo("permission_err.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{} {:?}", e.to_string(), e.source());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::{object, JsonValue};
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::Write;

    fn new_todo(s: String, v: Vec<Task>) -> TodoList {
        TodoList { title: s, tasks: v }
    }

    fn run(s: JsonValue, f: &str) -> Result<TodoList, Box<dyn Error>> {
        File::create(f)?;
        let mut file = OpenOptions::new().append(true).open(f)?;
        file.write_all(s.dump().as_bytes())?;
        let result = TodoList::get_todo(f);
        fs::remove_file(f)?;
        return result;
    }

    #[test]
    fn test_good_todo() {
        let file_name = "todo.json";
        let good_struct = new_todo(
            String::from("todo list for something"),
            vec![
                Task {
                    id: 0,
                    description: "do this".to_string(),
                    level: 0,
                },
                Task {
                    id: 1,
                    description: "do that".to_string(),
                    level: 5,
                },
            ],
        );
        let obj = object! {
            "title" : "todo list for something",
            "tasks": [
                { "id": 0, "description": "do this", "level": 0 },
                { "id": 1, "description": "do that", "level": 5 }
            ]
        };

        let result = run(obj, file_name).unwrap();

        assert_eq!(result.title, good_struct.title);
        assert_eq!(&result.tasks, &good_struct.tasks);
    }

    #[test]
    fn test_empty_tasks() {
        let result = run(
            object! {
            "title" : "empty tasks",
            "tasks": []},
            "empty_tasks.json",
        )
        .unwrap_err();

        assert_eq!(result.to_string(), "Fail to parses todo");
        assert!(result.source().is_none());
    }

    #[test]
    fn test_read() {
        let result = TodoList::get_todo("no_file.json").unwrap_err();
        assert_eq!(result.to_string(), "Fail to read todo file");
    }

    #[test]
    #[should_panic(
        expected = "Fail to read todo file Some(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })"
    )]
    fn test_read_error() {
        let result = TodoList::get_todo("no_file.json");
        result.unwrap_or_else(|e| panic!("{} {:?}", e.to_string(), e.source()));
    }

    #[test]
    #[should_panic(
        expected = "Fail to parses todo Some(Malformed(UnexpectedCharacter { ch: \',\', line: 1, column: 15 }))"
    )]
    fn test_malformed_error() {
        let file_name = "malformed.json";
        File::create(file_name).unwrap();
        let mut file = OpenOptions::new().append(true).open(file_name).unwrap();
        file.write_all(r#"{"something": ,}"#.as_bytes()).unwrap();
        let result = TodoList::get_todo(file_name);
        fs::remove_file(file_name).unwrap();

        result.unwrap_or_else(|e| panic!("{} {:?}", e.to_string(), e.source()));
    }
}
