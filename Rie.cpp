#include "sleepy_discord/websocketpp_websocket.h"
#include "Database/DatabaseWrapper.cpp"
#include <string>
#include "CommandHandler/CommandHandler.cpp"

using postgres::Config;
using postgres::Connection;

class Rie : public SleepyDiscord::DiscordClient {
private:
	CommandHandler cmdHandle;
	std::string defaultPrefix = "rie.";
	SleepyDiscord::DiscordClient client;
	
public:
	using SleepyDiscord::DiscordClient::DiscordClient;
	void onReady() {
		client.updateStatus("ready for service!", 0);
		printf("Rie is fully functioning and ready for service!!!!!");
	};
	void onMessage(SleepyDiscord::Message message) override {
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
		//cmdHandle.handleMessage(message);
	}
};

int main() {
	auto& dotenv = dotenv::env;
	DatabaseWrapper db;
	db.construct();	    

	std::stringstream strm;

	strm << db.readToken();

	std::string str = strm.str();

	printf("%s\n", str.c_str());

	Rie client(dotenv["DISCORD_TOKEN"], 2);
	client.run();
	db.connectionReset();
}