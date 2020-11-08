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
async fn dva(ctx: &Context, message: &Message) -> CommandResult {
    if !message.channel(&ctx).await.unwrap().is_nsfw() {
        message.channel_id.say(&ctx.http, "This command must be run in an NSFW Channel.").await?;

        Ok(())
    } else {
        let uri = "https://api.computerfreaker.cf/v1/dva".to_owned();

        let hentaiPic = getAnImage(uri);
        
        message.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("D.VA image").url(&hentaiPic);
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
        }).await?;

        Ok(())


    }    
}