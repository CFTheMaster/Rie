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
#[description = "Send a lewd hentai image"]
#[usage = "hentai"]
#[example = "hentai"]
fn hentai(ctx: &mut Context, message: &Message) -> CommandResult {
    if !message.channel(&ctx).unwrap().is_nsfw() {
        let _ = message.channel_id.say(&ctx.http, "This command must be run in an NSFW Channel.");


        let g = message.guild(&ctx.cache).unwrap();
        let width = 4;
        let discrim = format!("{:0width$}", message.author.discriminator, width = width);

        println!("Wrong channel executed command 'hentai' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

        Ok(())
    } else {
        let uri = "https://api.computerfreaker.cf/v1/hentai".to_owned();

        let hentaiPic = getAnImage(uri);
        
        if let Err(why) = message.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Hentai image").url(&hentaiPic);
                e.image(&hentaiPic);
                let width = 4;
                let discrim = format!("{:0width$}", message.author.discriminator, width = width);
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
}