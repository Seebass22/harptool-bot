use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use std::process::Command;

pub fn run(options: &[CommandDataOption]) -> String {
    let output = Command::new("harptool").output().unwrap();
    let output = String::from_utf8(output.stdout)
        .unwrap();
    format!("```\n{}\n```", output)
}

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("harptool")
        .description("create a harmonica note layout diagram")
        .create_option(|option| {
            option
                .name("position")
                .description("position")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(12)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("key")
                .description("key of harmonica")
                .kind(CommandOptionType::String)
                .add_string_choice("C", "C")
                .add_string_choice("G", "G")
                .add_string_choice("D", "D")
                .add_string_choice("A", "A")
                .add_string_choice("E", "E")
                .add_string_choice("B", "B")
                .add_string_choice("F#", "F#")
                .add_string_choice("Db", "Db")
                .add_string_choice("Ab", "Ab")
                .add_string_choice("Eb", "Eb")
                .add_string_choice("Bb", "Bb")
                .add_string_choice("F", "F")
                .required(false)
        })
        .create_option(|option| {
            option
                .name("use_degrees")
                .description("print scale degrees")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}
