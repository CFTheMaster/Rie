class CommandHandler : public SleepyDiscord::DiscordClient {
public:
	std::string defaultPrefix = "rie.";
public:
	void handleMessage(SleepyDiscord::Message message) {
		if (!message.author.bot && message.startsWith(defaultPrefix)) {
			if (message.content == (defaultPrefix + "test")) {
				SleepyDiscord::DiscordClient::sendMessage(message.channelID, "this is just a simple, <@!" + message.author.ID + ">");
			}

			if (message.content == (defaultPrefix + "me")) {
				SleepyDiscord::DiscordClient::sendMessage(message.channelID, "<@!" + message.author.ID + ">");
			}

		}
	}

public:
	void checkPerms() {
		// to do
	}
	
};