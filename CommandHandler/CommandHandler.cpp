#include "sleepy_discord/websocketpp_websocket.h"

class CommandHandler : public SleepyDiscord::DiscordClient {
public:
	std::string defaultPrefix = "rie.";
public:
	using SleepyDiscord::DiscordClient::DiscordClient;
	void onMessage(SleepyDiscord::Message message) override{
		
	}

public:
	void checkPerms() {
		// to do
	}
	
};