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


public:
	using SleepyDiscord::DiscordClient::DiscordClient;
	void onReady() {
		updateStatus("ready for service!", 0);
		printf("Rie is fully functioning and ready for service!!!!!");
	};
	void onMessage(SleepyDiscord::Message message) {
		printf("%s\n", message.content);
		cmdHandle.handleMessage(message);
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