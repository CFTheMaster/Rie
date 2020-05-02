#include "sleepy_discord/websocketpp_websocket.h"
#include "Database/DatabaseWrapper.cpp"
#include <string>
#include "CommandHandler/CommandHandler.cpp"

using postgres::Config;
using postgres::Connection;

class Rie : public SleepyDiscord::DiscordClient {
	std::string defaultPrefix = "rie.";
	using SleepyDiscord::DiscordClient::DiscordClient;
	//CommandHandler cmdHandler; just testing
public:
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
};

int main() {
	DatabaseWrapper db;
	
	db.construct();	    

	std::stringstream strm;

	strm << db.readToken();

	std::string str = strm.str();

	Rie client(str, 2);
	client.run();
	db.connectionReset();
}