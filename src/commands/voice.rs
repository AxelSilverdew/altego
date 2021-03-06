use serenity::client::CACHE;
use serenity::model::*;
use serenity::voice;
use serenity::Result as SerenityResult;

command!(deafen(ctx, msg) {
    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            check_msg(msg.channel_id.say("Groups and DMs not supported"));

            return Ok(());
        },
    };

    let mut shard = ctx.shard.lock();

    let handler = match shard.manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply("Not in a voice channel"));

            return Ok(());
        },
    };

    if handler.self_deaf {
        check_msg(msg.channel_id.say("Already deafened"));
    } else {
        handler.deafen(true);

        check_msg(msg.channel_id.say("Deafened"));
    }
});

command!(join(ctx, msg, args) {
    let connect_to = match args.get(0) {
        Some(arg) => match arg.parse::<u64>() {
            Ok(id) => ChannelId(id),
            Err(_why) => {
                check_msg(msg.reply("Invalid voice channel ID given"));

                return Ok(());
            },
        },
        None => {
            check_msg(msg.reply("Requires a voice channel ID be given"));

            return Ok(());
        },
    };

    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            check_msg(msg.channel_id.say("Groups and DMs not supported"));

            return Ok(());
        },
    };

    let mut shard = ctx.shard.lock();
    shard.manager.join(guild_id, connect_to);

    check_msg(msg.channel_id.say(&format!("Joined {}", connect_to.mention())));
});

command!(leave(ctx, msg) {
    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            check_msg(msg.channel_id.say("Groups and DMs not supported"));

            return Ok(());
        },
    };

    let mut shard = ctx.shard.lock();
    let has_handler = shard.manager.get(guild_id).is_some();

    if has_handler {
        shard.manager.remove(guild_id);

        check_msg(msg.channel_id.say("Left voice channel"));
    } else {
        check_msg(msg.reply("Not in a voice channel"));
    }
});

command!(mute(ctx, msg) {
    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            check_msg(msg.channel_id.say("Groups and DMs not supported"));

            return Ok(());
        },
    };

    let mut shard = ctx.shard.lock();

    let handler = match shard.manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply("Not in a voice channel"));

            return Ok(());
        },
    };

    if handler.self_mute {
        check_msg(msg.channel_id.say("Already muted"));
    } else {
        handler.mute(true);

        check_msg(msg.channel_id.say("Now muted"));
    }
});

command!(play(ctx, msg, args) {
    let url = match args.get(0) {
        Some(url) => url,
        None => {
            check_msg(msg.channel_id.say("Must provide a URL to a video or audio"));

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say("Must provide a valid URL"));

        return Ok(());
    }

    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            check_msg(msg.channel_id.say("Error finding channel info"));

            return Ok(());
        },
    };

    if let Some(handler) = ctx.shard.lock().manager.get(guild_id) {
        let source = match voice::ytdl(url) {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say("Error sourcing ffmpeg"));

                return Ok(());
            },
        };

        handler.play(source);

        check_msg(msg.channel_id.say("Playing song"));
    } else {
        check_msg(msg.channel_id.say("Not in a voice channel to play in"));
    }
});

command!(undeafen(ctx, msg) {
    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            check_msg(msg.channel_id.say("Error finding channel info"));

            return Ok(());
        },
    };

    if let Some(handler) = ctx.shard.lock().manager.get(guild_id) {
        handler.deafen(false);

        check_msg(msg.channel_id.say("Undeafened"));
    } else {
        check_msg(msg.channel_id.say("Not in a voice channel to undeafen in"));
    }
});

command!(unmute(ctx, msg) {
    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            check_msg(msg.channel_id.say("Error finding channel info"));

            return Ok(());
        },
    };

    if let Some(handler) = ctx.shard.lock().manager.get(guild_id) {
        handler.mute(false);

        check_msg(msg.channel_id.say("Unmuted"));
    } else {
        check_msg(msg.channel_id.say("Not in a voice channel to undeafen in"));
    }
});

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
