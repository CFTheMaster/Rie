use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult, Args},
    model::{prelude::Message}
};

#[command]
#[description = "Mute any user"]
#[usage = "mute @user reason"]
#[example = "mute @baka was spamming"]
fn mute(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {



    Ok(())

}