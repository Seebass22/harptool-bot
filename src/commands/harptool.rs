use harptool::Setup;
use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    let position = if let Some(index) = options.iter().position(|o| o.name == "position") {
        match options[index].resolved {
            Some(CommandDataOptionValue::Integer(p)) => p as usize,
            _ => 1,
        }
    } else {
        1
    };

    let tuning = if let Some(index) = options.iter().position(|o| o.name == "tuning") {
        match &options[index].resolved {
            Some(CommandDataOptionValue::String(tuning)) => tuning.clone(),
            _ => String::from("richter"),
        }
    } else {
        String::from("richter")
    };


    harptool::export(
        &tuning,
        "C",
        None,
        &Setup {
            scale: None,
            position,
        },
        true,
        true,
    );

    String::from("layout.png")
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
                .name("tuning")
                .description("tuning of harmonica")
                .kind(CommandOptionType::String)
                .add_string_choice("richter", "richter")
                .add_string_choice("country", "country")
                .add_string_choice("wilde tuning", "wilde tuning")
                .add_string_choice("wilde minor tuning", "wilde minor tuning")
                .add_string_choice("melody maker", "melody maker")
                .add_string_choice("natural minor", "natural minor")
                .add_string_choice("harmonic minor", "harmonic minor")
                .add_string_choice("paddy richter", "paddy richter")
                .add_string_choice("pentaharp", "pentaharp")
                .add_string_choice("powerdraw", "powerdraw")
                .add_string_choice("powerbender", "powerbender")
                .add_string_choice("diminished", "diminished")
                .add_string_choice("lucky 13 diminished", "lucky 13 diminished")
                .add_string_choice("lucky 13 powerchromatic", "lucky 13 powerchromatic")
                .add_string_choice("easy 3rd", "easy 3rd")
                .add_string_choice("4 hole richter", "4 hole richter")
                .add_string_choice("5 hole richter", "5 hole richter")
        })
        .create_option(|option| {
            option
                .name("use_degrees")
                .description("print scale degrees")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}
