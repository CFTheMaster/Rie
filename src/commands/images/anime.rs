use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};
use serde::{Deserialize, Serialize};

use curl::easy::Easy;

use crate::ReadImage::imageReader::getAnImage;


#[derive(Serialize, Deserialize)]
struct Image {
    url: String,
}


#[command]
#[description = "Send a cute anime image"]
#[usage = "anime"]
#[example = "anime"]
fn anime(ctx: &mut Context, message: &Message) -> CommandResult {

    let uri = "https://api.computerfreaker.cf/v1/anime".to_owned();

    let cuteAnimePic = getAnImage(uri);
    
    if let Err(why) = message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title("Cute anime image").url(&cuteAnimePic);
            e.image(&cuteAnimePic);

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

    println!("Processed command 'anime' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

    Ok(())



}