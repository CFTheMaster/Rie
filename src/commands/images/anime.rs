use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};
use serde::{Deserialize, Serialize};

use crate::ReadImage::imageReader::getAnImage;


#[derive(Serialize, Deserialize)]
struct Image {
    url: String,
}


#[command]
#[description = "Send a cute anime image"]
#[usage = "anime"]
#[example = "anime"]
async fn anime(ctx: &Context, message: &Message) -> CommandResult {

    let uri = "https://api.computerfreaker.cf/v1/anime".to_owned();

    let cuteAnimePic = getAnImage(uri);
    
    message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title("Cute anime image").url(&cuteAnimePic);
            e.image(&cuteAnimePic);
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