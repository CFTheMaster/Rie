#include "sleepy_discord/websocketpp_websocket.h"
#include "Database/DatabaseWrapper.cpp"
#include <string>
#include "CommandHandler/CommandHandler.cpp"

using postgres::Config;
using postgres::Connection;

class Rie : public SleepyDiscord::DiscordClient {
private:
	std::string defaultPrefix = "rie.";
	SleepyDiscord::DiscordClient client;
	
public:
	using SleepyDiscord::DiscordClient::DiscordClient;
	void onReady() {
		client.updateStatus("ready for service!", 0);
		printf("Rie is fully functioning and ready for service!!!!!");
	};
	void onMessage(SleepyDiscord::Message message) override {
		CommandHandler cmdHandle;
		try {
			cmdHandle.handleMessage(message);
		}
		catch (const std::exception& e) {
			std::cout << e.what() << std::endl;
		}
		
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