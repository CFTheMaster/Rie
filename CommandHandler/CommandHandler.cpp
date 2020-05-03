#include "sleepy_discord/websocketpp_websocket.h"

class CommandHandler : public SleepyDiscord::DiscordClient {
public:
	std::string defaultPrefix = "rie.";
public:
	using SleepyDiscord::DiscordClient::DiscordClient;
	void handleMessage(SleepyDiscord::Message message) {
		if (!message.author.bot && message.startsWith(defaultPrefix)) {
			if (message.content == (defaultPrefix + "test")) {
				sendMessage(message.channelID, "this is just a simple, <@!" + message.author.ID + ">");
			}

		}
	}

public:
	void checkPerms() {
		// to do
	}
	
};