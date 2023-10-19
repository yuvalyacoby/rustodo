mod action;
mod status;
mod todo;

#[derive(Debug)]
pub struct InputParams {
    pub action: action::Action,
    pub todo_name: String,
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
            todo_name: args[2].to_string(),
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
            let todo = todo::get_todo(&params.todo_name)?;
            println!("{:?}", todo);
            Ok(())
        },
        action::Action::Update => {
            let todo = todo::update_todo(params.todo_name, params.new_status, params.new_description)?;
            println!("Successfully updated todo: {:?}", todo);
            Ok(())
        }
        _ => Err("action not supported yet".to_string())
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
