#include "sleepy_discord/websocketpp_websocket.h"
#include "Database/Databasewrapper.cpp"
#include <string>
#include "dotenv.h"

using postgres::Config;
using postgres::Connection;

class Rie : public SleepyDiscord::DiscordClient {
    public:
        using SleepyDiscord::DiscordClient::DiscordClient;

        onMessage(SleepyDiscord::Message message) override {
			if (message.startsWith("change! hello"))
				sendMessage(message.channelID, "Hello " + message.author.username + "#" + message.author.discriminator);
		}

};

void configBuilder() {
	auto& dotenv = dotenv::env;
	Connection conn{ Config::Builder{}
		.user(dotenv["USERNAME"])
		.password(dotenv["PASSWORD"])
		.dbname(dotenv["DATABASE_NAME"])
		.build()
	};
}

int main() {
	DatabaseWrapper::createTables;
	std::string str1(DatabaseWrapper::getToken);
	Rie client(str1, 2);
	client.run();
	DatabaseWrapper::connectionReset;
}