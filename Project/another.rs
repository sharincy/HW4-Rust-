use serde::{Serialize, Deserialize};
use serde_json;


// Define a custom data structure for tasks
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    description: String,
    completed: bool,
}


fn main_menu(tasks: &mut Vec<Task>) {
    println!("Todo List Menu:");
    println!("1. View Tasks");
    println!("2. Add Task");
    println!("3. Edit Task");
    println!("4. Delete Task");
    println!("5. Mark as Complete");
    println!("0. Exit");

    // Capture user input
    let choice = get_user_input();

    // Based on the choice, call the corresponding function, in match I try to cooporate the Recursion
    match choice.as_str() {
        "1" => view_tasks(tasks),
        "2" => add_task(tasks),
        "3" => edit_task(tasks),
        "4" => delete_task(tasks),
        "5" => mark_as_complete(tasks),
        "0" => return,
        _ => {
            println!("Invalid choice. Please try again.");
            main_menu(tasks);
        }
    }
}


fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


fn view_tasks(tasks: &Vec<Task>) {
    println!("Viewing tasks:");
    // Retrieve and display tasks
    for (index, task) in tasks.iter().enumerate() {
        println!("Task {}: {:?}", index + 1, task);
    }
}


fn add_task(tasks: &mut Vec<Task>) {
    println!("Adding a new task:");

    // Prompt for task title and description
    println!("Enter task title:");
    let title = get_user_input();

    println!("Enter task description:");
    let description = get_user_input();

    // Create a new task and add it to the task list
    let new_task = Task {
        title,
        description,
        completed: false, // You can set it to false by default
    };
    tasks.push(new_task);

    println!("Task added successfully.");
}



fn edit_task(tasks: &mut Vec<Task>) {
    println!("Editing a task:");

    // Prompt for the task number to edit
    println!("Enter the task number to edit:");
    let task_number = get_user_input().parse::<usize>().unwrap();

    if task_number <= tasks.len() {
        // Prompt for the updated title and description
        println!("Enter the updated task title:");
        let title = get_user_input();

        println!("Enter the updated task description:");
        let description = get_user_input();

        // Update the task in the task list
        tasks[task_number - 1].title = title;
        tasks[task_number - 1].description = description;
        println!("Task edited successfully.");
    } else {
        println!("Invalid task number.");
    }
}



fn delete_task(tasks: &mut Vec<Task>) {
    println!("Deleting a task:");

    // Prompt for the task number to delete
    println!("Enter the task number to delete:");
    let task_number = get_user_input().parse::<usize>().unwrap();

    if task_number <= tasks.len() {
        // Remove the task from the task list
        tasks.remove(task_number - 1);
        println!("Task deleted successfully.");
    } else {
        println!("Invalid task number.");
    }
}


fn mark_as_complete(tasks: &mut Vec<Task>) {
    println!("Marking a task as complete:");

    // Prompt for the task number to mark as complete
    println!("Enter the task number to mark as complete:");
    let task_number = get_user_input().parse::<usize>().unwrap();

    if task_number <= tasks.len() {
        if !tasks[task_number - 1].completed {
            tasks[task_number - 1].completed = true;
            println!("Task marked as complete.");
        } else {
            println!("Task is already marked as complete.");
        }
    } else {
        println!("Invalid task number.");
    }
}




fn save_tasks_to_file(tasks: &Vec<Task>, file_name: &str) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::Write;

    let serialized = serde_json::to_string(&tasks)?;

    let mut file = File::create(file_name)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

fn load_tasks_from_file(file_name: &str) -> Result<Vec<Task>, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;

    Ok(tasks)
}

fn main() {
    let file_name = "tasks.json"; // Use a JSON file

    // Load tasks from the file (create an empty task list if the file doesn't exist)
    let mut tasks = match load_tasks_from_file(file_name) {
        Ok(t) => t,
        Err(_) => Vec::new(),
    };

    main_menu(&mut tasks);

    if let Err(e) = save_tasks_to_file(&tasks, file_name) {
        eprintln!("Error saving tasks: {}", e);
    }
}
