use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::ReadImage::imageReader::getAnImage;


#[derive(Serialize, Deserialize)]
struct Image {
    url: String,
}


#[command]
#[description = "Send a lewd D.VA image"]
#[usage = "dva"]
#[example = "dva"]
fn dva(ctx: &mut Context, message: &Message) -> CommandResult {
    if !message.channel(&ctx).unwrap().is_nsfw() {
        let _ = message.channel_id.say(&ctx.http, "This command must be run in an NSFW Channel.");


        let g = message.guild(&ctx.cache).unwrap();
        let width = 4;
        let discrim = format!("{:0width$}", message.author.discriminator, width = width);

        println!("Wrong channel executed command 'D.VA' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

        Ok(())
    } else {
        let uri = "https://api.computerfreaker.cf/v1/dva".to_owned();

        let hentaiPic = getAnImage(uri);
        
        if let Err(why) = message.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("D.VA image").url(&hentaiPic);
                e.image(&hentaiPic);

                e
            });

            m
        }) {
            println!("Error: {:?}", why);
            let _ = message.channel_id.say(&ctx.http, "Missing permissions");
        }

        let g = message.guild(&ctx.cache).unwrap();
        let width = 4;
        let discrim = format!("{:0width$}", message.author.discriminator, width = width);

        println!("Processed command 'D.VA' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

        Ok(())


    }    
}