use serenity::{
    framework::standard::{macros::command, Args, CommandResult, CommandError},
    model::{prelude::*, ModelError::{InvalidPermissions, DeleteMessageDaysAmount}, id::UserId},
    prelude::*,
    Error,
};
use regex::Regex;
use crate::utils::user::get_id;
use std::fmt::write;

#[command]
#[only_in("guilds")]
#[description = "Ban any user"]
#[usage = "ban @user reason"]
#[example = "ban @hammertime very very annoying"]
#[required_permissions("BAN_MEMBERS")]
fn ban(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {
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
    let sent_message = message.channel_id.say(&ctx, &format!("Attempting to ban {} users...", count));

    // log the ban in the database
    let mut s = String::new();

    let mut bans = match guild.bans(&ctx) {
        Ok(val) => val.iter().map(|x| x.user.id.0).collect(),
        Err(_) => Vec::new(),
    };

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

        // check if already banned in server ban list or in current currently
        if bans.contains(&u) {
            let _ = message.channel_id.say(&ctx, &format!(":x: {} - Error: User is already banned\n", user_tag_id));
            continue;
        }

        // ban the user
        let ban_result = if let Some(reason) = reason {
            guild.ban(&ctx, u, &(7, reason))
        } else {
            guild.ban(&ctx, u, &7)
        };

        // check the ban result
        match ban_result {
            Err(Error::Model(InvalidPermissions(permissions))) => {
                let e = format!("I don't have permission to ban this user, requires: `{:?}`.", permissions);
                let _ = message.channel_id.say(&ctx, &format!(":question: {} - Error: {}\n", &user_tag_id, &e));
            },
            Err(Error::Model(DeleteMessageDaysAmount(num))) => {
                let e = format!("The number of days worth of messages to delete is over the maximum: ({}).", num);
                let _ = message.channel_id.say(&ctx, &format!(":x: {} - Error: {}\n", &user_tag_id, &e));
            }
            Err(_) => {
                let e = "There was an unknown error trying to ban this user.";
                let _ = message.channel_id.say(&ctx, &format!(":question: {} - Error: {}\n", &user_tag_id, &e));
            },
            Ok(_) => {
                let _ = message.channel_id.say(&ctx, &format!(":hammer: {} banned.\n", &user_tag_id));
                // add the ban to the vec to prevent dupe bans
                bans.push(u);
            },
        }
    }

    Ok(())
}