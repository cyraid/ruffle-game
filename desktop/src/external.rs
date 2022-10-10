use ruffle_core::context::UpdateContext;
use ruffle_core::external::{ExternalInterfaceMethod, ExternalInterfaceProvider, Value as ExternalValue};

pub struct GameExternalInterfaceProvider {
}

impl GameExternalInterfaceProvider {

    pub fn new() -> Self {
        Self {
        }
    }

}

impl ExternalInterfaceProvider for GameExternalInterfaceProvider {

    fn get_method(&self, name: &str) -> Option<Box<dyn ExternalInterfaceMethod>> {
        Some(Box::new(GameMethod::new(name.to_string())))
    }

    fn on_callback_available(&self, _name: &str) {
    }

    fn on_fs_command(&self, command: &str, _args: &str) -> bool {
        println!("command: {}", command);
        false
    }

}

struct GameMethod {
    name: String,
}

impl GameMethod {

    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    fn print_arg(&self, arg: &ExternalValue) {
        match arg {
            ExternalValue::Null => print!("{}", "NULL".to_string()),
            ExternalValue::Bool(value) => print!("{}", value.to_string()),
            ExternalValue::Number(value) => print!("{}", value.to_string()),
            ExternalValue::String(value) => print!("\"{}\"", value.to_string()),
            ExternalValue::List(values) => {

                let mut count = 0;

                print!("{}", "[");

                for value in values {
                    if count > 0 { print!("{}", ","); }
                    self.print_arg(value);
                    count = count + 1;
                }

                print!("{}", "]");

            },
            _ => print!("{}", "N/A".to_string())
        }
    }

}

impl ExternalInterfaceMethod for GameMethod {

    fn call(
        &self,
        _context: &mut UpdateContext<'_, '_, '_>,
        args: &[ExternalValue],
    ) -> ExternalValue {

        match self.name.as_str() {
            "test" => return ExternalValue::List(args.to_owned()),
            "print" => {

                let mut count = 0;

                for arg in args {
                    if count > 0 { print!("{}", ","); }
                    self.print_arg(arg);
                    count = count + 1;
                }

                println!("{}", "");

            },
            _ => (),
        }

        ExternalValue::Null

    }
}
