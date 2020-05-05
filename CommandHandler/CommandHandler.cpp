#include "sleepy_discord/websocketpp_websocket.h"

class CommandHandler{
private:
	std::string defaultPrefix = "rie.";
	SleepyDiscord::DiscordClient client;
public:
	
	void handleMessage(SleepyDiscord::Message message) {
		try {
			if (!message.author.bot && message.startsWith(defaultPrefix)) {
				printf("a command has ran");
				if (message.content == (defaultPrefix + "test")) {
					client.sendMessage(message.channelID, "this is just a simple test <@!"
						+ message.author.ID
						+ ">");
				}

				if (message.content == (defaultPrefix + "me")) {
					client.sendMessage(message.channelID, "<@!"
						+ message.author.ID + ">");
				}

			}
		}
		catch (const std::exception& e) {
			printf(e.what);
		}
		
	}

public:
	void checkPerms() {
		// to do
	}
	
};