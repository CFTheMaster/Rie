#include "sleepy_discord/websocketpp_websocket.h"
#include "Database/DatabaseWrapper.cpp"
#include <string>
#include "CommandHandler/CommandHandler.cpp"

using postgres::Config;
using postgres::Connection;

class Rie : public SleepyDiscord::DiscordClient {
	std::string defaultPrefix = "rie.";

	//CommandHandler cmdHandler; just testing
public:
	using SleepyDiscord::DiscordClient::DiscordClient;
	void onReady() {
		updateStatus("ready for service!", 0);
	}
	void onMessage(SleepyDiscord::Message message) {
		if (!message.author.bot && message.startsWith(defaultPrefix)) {
			std::string str = message.content;
			str = std::cin.get();
			std::cin.ignore(256, '.');

			std::string str2("test");

			std::size_t found = str.find(str2);
			if (found = 4) {
				if (found != std::string::npos) {
					sendMessage(message.channelID, "this is just a simple test <@!" + message.author.ID + ">");
				}
			}
			
		}
	};
};

int main() {
	auto& dotenv = dotenv::env;
	DatabaseWrapper db;
	
	db.construct();	    

	std::stringstream strm;

	strm << db.readToken();

	std::string str = strm.str();

	Rie client(dotenv["DISCORD_TOKEN"], 2);
	client.run();
	db.connectionReset();
}