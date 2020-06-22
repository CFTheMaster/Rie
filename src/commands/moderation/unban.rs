#![allow(unused_variables)]
use serenity::{
    framework::standard::{macros::command, Args, CommandResult, CommandError},
    model::{prelude::*, ModelError::{InvalidPermissions}, id::UserId},
    prelude::*,
    Error,
};
use regex::Regex;
use crate::utils::user::get_id;

#[command]
#[description = "Unban any user"]
#[usage = "unban userid -r reason"]
#[example = "unban 123456 -r appealed"]
#[required_permissions("BAN_MEMBERS")]
fn unban(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let g = message.guild(&ctx.cache).unwrap();
    let width = 4;
    let discrim = format!("{:0width$}", message.author.discriminator, width = width);

    let raw_users = match args.single::<String>() {
        Ok(r) => r,
        Err(_) => return Err(CommandError::from("No user given.")),
    };
    let split = raw_users.split("||");
    let mut users = Vec::new();

    // loop through each one and parse the user id
    for user in split {
        match get_id(user) {
            Some(val) => users.push(val),
            None => return Err(CommandError::from("Malformed mention or ID.")),
        };
    }

    // get the guild
    let guild = match message.guild(&ctx) {
        Some(val) => val.read().clone(),
        None => return Err(CommandError::from("No guild.")),
    };

    // get the reason
    let reason_raw = args.rest();
    let reason;

    let re = Regex::new(r"\d{17,18}[a-zA-Z ]+").unwrap();
    if re.is_match(reason_raw) && !reason_raw.starts_with("-r ") {
        return Err(CommandError::from("Oh no, something went wrong!"));
    } else if reason_raw.starts_with("-r ") && !reason_raw.is_empty() {
        reason = Some(&reason_raw[2..]);
    } else if !reason_raw.is_empty() {
        reason = Some(&reason_raw[..]);
    } else {
        reason = None;
    }

    let count = users.len();
    let _ = message.channel_id.say(&ctx, &format!("Attempting to unban {} users...", count));

    for u in users {
        // fetch the user for tag
        let user = match UserId(u).to_user(&ctx) {
            Ok(val) => val,
            Err(e) => {
                let _ = message.channel_id.say(&ctx, &format!(":x: {} - Error: Failed to fetch user: {}\n", u, &e));
                continue;
            }
        };

        // format a tag (id) string for the user
        let user_tag_id = format!("`{} ({})`", user.tag(), user.id.0);

        // unban the user
        let ban_result = if let Some(reason) = reason {
            guild.unban(&ctx, u)
        } else {
            guild.unban(&ctx, u)
        };

        // check the unban result
        match ban_result {
            Err(Error::Model(InvalidPermissions(permissions))) => {
                println!("Processed command 'unban' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);
                let e = format!("I don't have permission to unban this user, requires: `{:?}`.", permissions);
                let _ = message.channel_id.say(&ctx, &format!(":question: {} - Error: {}\n", &user_tag_id, &e));
            },
            Err(_) => {
                println!("Processed command 'unban' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);
                let e = "There was an unknown error trying to unban this user.";
                let _ = message.channel_id.say(&ctx, &format!(":question: {} - Error: {}\n", &user_tag_id, &e));
            },
            Ok(_) => {
                println!("Processed command 'unban' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);
                let _ = message.channel_id.say(&ctx, &format!(":white_check_mark: {} unbanned.\n", &user_tag_id));
            },
        }
    }


    Ok(())

}