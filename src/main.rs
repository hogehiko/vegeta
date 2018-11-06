
struct Global{

}

impl Global{
    fn new(tasks: &Vec<Task>) -> Global{
        Global{}
    }
}

struct Environment{

}

impl Environment{
    fn new(variables: &Vec<Variable>)->Environment{
        Environment{}
    }
}

#[derive(Debug)]
struct Variable {
}

impl Variable{
    
}

struct Task{
    name: String,
    pattern: String,
    requirements: Vec<Requirement>,
    commands: Vec<Command>
}

impl Task{
    fn new(name: &str, pattern: &str, requirements:&Vec<Requirement>, command: &Vec<Command>)->Task{
        Task{name: name.to_string(), 
            pattern: pattern.to_string(), 
            requirements: requirements.clone(), 
            commands: command.clone()}
    }
}

#[derive(Clone)]
struct Requirement{
    name: String,
    pattern: String
}

impl Requirement{
    fn new(name: &str, pattern: &str)->Requirement{
        Requirement{name: name.to_string(), pattern: pattern.to_string()}
    }
}

#[derive(Clone)]
struct Command{
    command: String
}

impl Command{
    fn new(command: &str)-> Command{
        Command{command: command.to_string()}
    }
}
fn main() {
    let x = Task::new(
        "hoge", 
        "fuga", 
        vec![
            Requirement::new("hoge", "fuga")
        ],
        vec![Command::new("hoge")]);

    println!("Hello, world!");
}
