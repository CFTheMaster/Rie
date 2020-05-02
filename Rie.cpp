#include "sleepy_discord/websocketpp_websocket.h"
#include "Database/DatabaseWrapper.cpp"
#include <string>
#include "CommandHandler/CommandHandler.cpp"

using postgres::Config;
using postgres::Connection;

class Rie : public SleepyDiscord::DiscordClient {
	using SleepyDiscord::DiscordClient::DiscordClient;
	CommandHandler cmdHandler;
};

int main() {
	DatabaseWrapper db;
	db.construct();

	std::stringstream strm;

	strm << db.readToken;

	std::string str = strm.str();

	Rie client(str, 2);
	client.run();
	db.connectionReset;
}