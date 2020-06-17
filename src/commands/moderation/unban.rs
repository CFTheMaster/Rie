use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult, Args},
    model::{prelude::Message}
};

#[command]
#[description = "Unban any user"]
#[usage = "unban userid reason"]
#[example = "unban 123456 appealed"]
fn unban(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {



    Ok(())

}