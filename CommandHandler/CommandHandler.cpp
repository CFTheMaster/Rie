#include "sleepy_discord/websocketpp_websocket.h"

class CommandHandler : Rie{
public:
	std::string defaultPrefix = "rie.";
public:
	
	void handleMessage(SleepyDiscord::Message message) {
		if (!message.author.bot && message.startsWith(defaultPrefix)) {
			printf("a command has ran");
			if (message.content == (defaultPrefix + "test")) {
				SleepyDiscord::DiscordClient::sendMessage(message.channelID, "this is just a simple test <@!" 
					+ message.author.ID 
					+ ">");
			}

			if (message.content == (defaultPrefix + "me")) {
				SleepyDiscord::DiscordClient::sendMessage(message.channelID, "<@!"
					+ message.author.ID + ">");
			}

		}
	}

public:
	void checkPerms() {
		// to do
	}
	
};