use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult, Args, CommandError},
    model::{prelude::Message}
};

#[command]
#[only_in("guilds")]
#[description = "Ban any user"]
#[usage = "ban @user reason"]
#[example = "ban @hammertime very very annoying"]
#[required_permissions("BAN_MEMBERS")]
fn ban(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {


    Ok(())

}