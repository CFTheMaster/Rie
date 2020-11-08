use serenity::{
    client::Context,
    framework::standard::{macros::help, CommandResult, Args, CommandGroup, Command, HelpOptions},
    model::{prelude::{Message, UserId}},
    utils::Colour,
};
use std::string::String;
use std::collections::HashSet;


#[help]
async fn help(ctx: &Context, message: &Message, args: Args, _help_options: &'static HelpOptions, groups: &[&'static CommandGroup], _owners: HashSet<UserId>,) -> CommandResult{
    let mut s = String::new();

    match args.len() {
        0 => s.push_str(&command_list(groups)),
        1 => s.push_str(&command_help(groups, args.current().unwrap().to_string())),
        _ => s.push_str("Too many arguments."),
    };

    let width = 4;
    let discrim = format!("{:0width$}", message.author.discriminator, width = width);

    message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title("Help Command");
            e.description(&s);
            e.colour(Colour::new(6684876));
            e.footer(|f| {
                f.icon_url(message.author.avatar_url().unwrap());
                f.text(format!("Executed by {}#{} ({})", message.author.name, discrim, message.author.id));

                f
            });

            e
        });

        m
    }).await?;

    let _g = message.guild(&ctx.cache).await.unwrap();
    
    Ok(())
    
}

fn command_list(groups: &[&'static CommandGroup]) -> String {
    let mut s = "Bot made by <@!138302166619258880> & <@!214393232342122506>".to_string();
    s.push_str("\n\n");
    for x in groups {
        let mut n = format!("**{}:** ", x.name);
        for y in x.options.commands {
            let name = y.options.names.first().unwrap();
            n.push_str(&format!("{}, ", &name)[..]);
        }
        n.push_str("\n");
        s.push_str(&n[..]);
        s.remove(s.len() - 3);
    }
    s.push_str("\n");
    s
}

fn command_help(groups: &[&'static CommandGroup], arg: String) -> String {
    let mut s = String::new();

    let mut matched_command: Option<&Command> = None;
    for x in groups {
        for y in x.options.commands {
            let name = y.options.names.first().unwrap();
            if name == &arg {
                matched_command = Some(y);
            }
        }
    }

    if matched_command.is_none() {
        s.push_str("No command found.");
        return s;
    }

    let command = matched_command.unwrap();

    s.push_str(&format!(
        "Command **{}**",
        command.options.names.first().unwrap()
    ));

    s.push_str("\n");
    if let Some(description) = command.options.desc {
        s.push_str("Description: ");
        s.push_str(description);
    } else {
        s.push_str("No description available.");
    }

    s.push_str("\n");
    if let Some(usage) = command.options.usage {
        s.push_str("Usage: ");
        s.push_str(usage);
    } else {
        s.push_str("No usage available.");
    }

    s.push_str("\n");
    if !command.options.examples.is_empty() {
        s.push_str("Examples: ");
        for x in command.options.examples {
            s.push_str(x);
        }
    } else {
        s.push_str("No examples available.")
    }
    s
}