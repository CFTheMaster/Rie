#include "sleepy_discord/websocketpp_websocket.h"
#include "Database/DatabaseWrapper.cpp"
#include <string>
#include "dotenv.h"
#include "CommandHandler/CommandHandler.cpp"

using postgres::Config;
using postgres::Connection;

class Rie : public SleepyDiscord::DiscordClient {
	using SleepyDiscord::DiscordClient::DiscordClient;
	CommandHandler cmdHandler();
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
	configBuilder();
	DatabaseWrapper db;
	db.createTables;

	std::stringstream strm;

	strm << db.readToken;

	std::string str = strm.str();

	Rie client(str, 2);
	client.run();
	db.connectionReset;
}