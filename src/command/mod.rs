use serenity::framework::standard::macros::group;

mod me;
mod help;
mod ping;

use crate::command::me::*
use crate::command::help::*
use crate::command::ping::*

group!({
    name: "general",
    commands: [me, help, ping]
})