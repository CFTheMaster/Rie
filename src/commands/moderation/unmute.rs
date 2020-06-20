#![allow(unused_variables)]

use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult, Args},
    model::{prelude::Message}
};

#[command]
#[description = "Unmute any user"]
#[usage = "unmute @User reason"]
#[example = "unmute @someone was a good boy"]
fn unmute(ctx: &mut Context, message: &Message, args: Args) -> CommandResult {



    Ok(())

}