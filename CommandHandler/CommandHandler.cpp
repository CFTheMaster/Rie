#include "sleepy_discord/websocketpp_websocket.h"

class CommandHandler : SleepyDiscord::DiscordClient {
	std::string defaultPrefix = "rie.";
public:
	using SleepyDiscord::DiscordClient::DiscordClient;
	CommandHandler() {}
	void onReady() {
		updateStatus("ready for service!", 0);
	}
	void onMessage(SleepyDiscord::Message message) {
		if (!message.author.bot && message.startsWith(defaultPrefix)) {
			std::string str = message.content;
			std::string str2("test");

			std::size_t found = str.find(str2);
			if (found != std::string::npos) {
				sendMessage(message.channelID, "this is just a simple test <@!" + message.author.ID + ">");
			}
		}
		else return;
	}


	
};