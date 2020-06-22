#![allow(unused_variables)]

use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult, Args},
    model::{prelude::Message}
};

#[command]
#[description = "Mute any user"]
#[usage = "mute @user -r reason"]
#[example = "mute @baka -r was spamming"]
fn mute(ctx: &mut Context, message: &Message, args: Args) -> CommandResult {



    Ok(())

}