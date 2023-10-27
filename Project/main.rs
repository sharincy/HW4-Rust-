fn main_menu(tasks: &mut Vec<String>) {
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
    match choice.as_str() { // Convert the String to &str
        "1" => view_tasks(tasks),
        "2" => add_task(tasks),
        "3" => edit_task(tasks),
        "4" => delete_task(tasks),
        "5" => mark_as_complete(tasks),
        "0" => return, // Exit the program
        _ => {
            println!("Invalid choice. Please try again.");
            main_menu(tasks); // Redisplay the main menu
        }
    }
}


fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


fn view_tasks(tasks: &Vec<String>) {
    println!("Viewing tasks:");
    // Retrieve and display tasks
    for (index, task) in tasks.iter().enumerate() {
        println!("Task {}: {}", index + 1, task);
    }
}


fn add_task(tasks: &mut Vec<String>) {
    println!("Adding a new task:");

    // Prompt for task title and description
    println!("Enter task title:");
    let title = get_user_input();

    println!("Enter task description:");
    let description = get_user_input();

    // Create a new task and add it to the task list
    let new_task = format!("Title: {}, Description: {}", title, description);
    tasks.push(new_task);

    println!("Task added successfully.");
}


fn edit_task(tasks: &mut Vec<String>) {
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
        tasks[task_number - 1] = format!("Title: {}, Description: {}", title, description);
        println!("Task edited successfully.");
    } else {
        println!("Invalid task number.");
    }
}


fn delete_task(tasks: &mut Vec<String>) {
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


fn mark_as_complete(tasks: &mut Vec<String>) {
    println!("Marking a task as complete:");

    // Prompt for the task number to mark as complete
    println!("Enter the task number to mark as complete:");
    let task_number = get_user_input().parse::<usize>().unwrap();

    if task_number <= tasks.len() {
        // Mark the task as complete by adding a "completed" flag
        let task = &mut tasks[task_number - 1];
        if !task.contains("[Completed]") {
            task.push_str(" [Completed]");
            println!("Task marked as complete.");
        } else {
            println!("Task is already marked as complete.");
        }
    } else {
        println!("Invalid task number.");
    }
}



fn main() {
    // Initialize an empty vector to store tasks
    let mut tasks: Vec<String> = Vec::new();

    // Call the main menu function
    main_menu(&mut tasks);
}
