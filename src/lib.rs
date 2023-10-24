mod action;
mod status;
mod todo;

#[derive(Debug)]
pub struct InputParams {
    pub action: action::Action,
    pub new_status: Option<status::Status>,
    pub new_description: Option<String>
}

impl InputParams {
    pub fn from_cli(args: &[String]) -> Result<InputParams, String> {
        if args.len() < 2 {
            return Err("not enough arguments".to_string());
        }
        let default = String::new();
        let todo_name = args.get(2).unwrap_or(&default);
        Ok(InputParams {
            action: (format!("{}::{}", &args[1], &todo_name)).parse::<action::Action>()?,
            new_status: args.get(3).map(|arg| arg.to_string().parse()).transpose()?,
            new_description: args.get(4).map(|arg| arg.to_string())
        })
    }
}

pub fn run(params: InputParams) -> Result<(), String> {
    match params.action {
        action::Action::GetAll => {
            let r = todo::get_todos()?;
            for todo in &r {
                println!("{:?}", todo);
            }
            Ok(())
        },
        action::Action::GetOne(todo_name) => {
            let todo = todo::get_todo(&todo_name)?;
            println!("{:?}", todo);
            Ok(())
        },
        action::Action::Update(todo_name) => {
            let todo = todo::update_todo(todo_name, params.new_status, params.new_description)?;
            println!("Successfully updated todo: {:?}", todo);
            Ok(())
        }
        action::Action::DeleteAll => {
            todo::delete_all()?;
            println!("Successfully deleted all todos");
            Ok(())
        },
        action::Action::DeleteOne(todo_name) => {
            let todos = todo::delete_one(&todo_name)?;
            println!("Successfully deleted todo. Updated todos: {:?}", todos);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
