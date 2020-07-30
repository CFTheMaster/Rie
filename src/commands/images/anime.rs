use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};
use serde::{Deserialize, Serialize};

use curl::easy::Easy;
use serde_json::{Value};


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
    let mut handle = Easy::new();
    handle.url(&uri).unwrap();
    let mut html: String = String::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            html = String::from_utf8(Vec::from(data)).unwrap();
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    
    let v: Image = serde_json::from_str(&html)?;
    
    if let Err(why) = message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title("Cute anime image");
            e.image(v.url);

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