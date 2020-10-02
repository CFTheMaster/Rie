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
#[description = "Send a yuri anime image"]
#[usage = "yuri"]
#[example = "yuri"]
fn yuri(ctx: &mut Context, message: &Message) -> CommandResult {

    let uri = "https://api.computerfreaker.cf/v1/yuri".to_owned();

    let cuteAnimePic = getAnImage(uri);
    
    if let Err(why) = message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title("Cute yuri image").url(&cuteAnimePic);
            e.image(&cuteAnimePic);

            e
        });

        m
    }) {
        println!("Error: {:?}", why);
        let _ = message.channel_id.say(&ctx.http, "Missing permissions");
    }

    Ok(())



}