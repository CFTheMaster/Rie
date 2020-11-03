use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
};


#[command]
#[description = "Get current system statistics"]
#[usage = "system"]
#[example = "system"]
async fn system(ctx: &Context, message: &Message) -> CommandResult {
    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();

    // We can use ChannelId directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.
    if let Err(why) = message.channel_id.send_message(&ctx, |m| m.embed(|e| {
        e.title("System Resource Load");
        e.field(
            "CPU Load Average",
            format!("{:.2}%", cpu_load.one * 10.0),
            false,
        );
        e.field(
            "Memory Usage",
            format!("{:.2} MB Free out of {:.2} MB", mem_use.free as f32 / 1000.0, mem_use.total as f32 / 1000.0),
            false,
        );
        e
    })).await {
        eprintln!("Error sending message: {:?}", why);
    };

    Ok(())

}