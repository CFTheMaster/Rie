use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message}
};

#[command]
#[description = "get the invite for the bot"]
#[usage = "invite"]
#[example = "invite"]
fn invite(ctx: &mut Context, message: &Message) -> CommandResult{
    let _ = message.reply(&ctx.http, "invite me: https://discord.com/oauth2/authorize?client_id=706219430912327742&scope=bot&permissions=67160068");

    let g = message.guild(&ctx.cache).unwrap();
    let width = 4;
    let discrim = format!("{:0width$}", message.author.discriminator, width = width);

    println!("Processed command 'invite' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

    Ok(()) 
}
