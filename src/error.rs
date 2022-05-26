use crate::command;

pub fn action_undefined_error_exit() {
    let action_undefined_error = command().error(
        clap::ErrorKind::InvalidValue,
        "The given action is not supported, please enter encode or decode."
    );
    action_undefined_error.exit()
}

pub fn parse_error_exit() {
    let parse_error = command().error(
        clap::ErrorKind::InvalidValue,
        "The given string is not a base64 or hex."
    );
    parse_error.exit()
}

pub fn type_undefined_error_exit() {
    let type_undefined_error = command().error(
        clap::ErrorKind::InvalidValue,
        "The given type is not supported, please enter base64, hex, etc."
    );
    type_undefined_error.exit()
}