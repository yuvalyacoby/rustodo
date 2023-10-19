mod action;
mod status;
mod todo;

#[derive(Debug)]
pub struct InputParams {
    pub action: action::Action,
    pub todo_name: Option<String>,
    pub new_status: Option<status::Status>,
    pub new_description: Option<String>
}

impl InputParams {
    pub fn from_cli(args: &[String]) -> Result<InputParams, String> {
        if args.len() < 2 {
            return Err("not enough arguments".to_string());
        }
        Ok(InputParams {
            action: args[1].parse::<action::Action>()?,
            todo_name: args.get(2).map(|arg| arg.to_string()),
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
        action::Action::GetOne => {
            let name = params.todo_name.ok_or("name is required".to_string())?;
            let todo = todo::get_todo(&name)?;
            println!("{:?}", todo);
            Ok(())
        },
        action::Action::Update => {
            let name = params.todo_name.ok_or("name is required".to_string())?;
            let todo = todo::update_todo(name, params.new_status, params.new_description)?;
            println!("Successfully updated todo: {:?}", todo);
            Ok(())
        }
        action::Action::DeleteAll => {
            todo::delete_all()?;
            println!("Successfully deleted all todos");
            Ok(())
        },
        action::Action::DeleteOne => {
            let name = params.todo_name.ok_or("name is required".to_string())?;
            let todos = todo::delete_one(&name)?;
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
