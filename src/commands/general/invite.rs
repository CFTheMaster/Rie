use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message},
    utils::Colour
};

#[command]
#[description = "get the invite for the bot"]
#[usage = "invite"]
#[example = "invite"]
fn invite(ctx: &mut Context, message: &Message) -> CommandResult{
    if let Err(why) = message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            let width = 4;
            let discrim = format!("{:0width$}", message.author.discriminator, width = width);
            e.title("Rie Invite");
            e.description(format!("[Click here for the invite](https://discord.com/oauth2/authorize?client_id=706219430912327742&scope=bot&permissions=314436)"));
            e.colour(Colour::new(6684876));
            e.footer(|f| {
                f.icon_url(message.author.avatar_url().unwrap());
                f.text(format!("Executed by {}#{} ({})", message.author.name, discrim, message.author.id));

                f
            });
            e
        });

        m
    }) {
        println!("Error: {:?}", why);
        let _ = message.channel_id.say(&ctx.http, "Missing permissions");
    }

    Ok(()) 
}