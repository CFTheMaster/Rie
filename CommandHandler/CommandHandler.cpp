#include "sleepy_discord/websocketpp_websocket.h"

namespace CommandHandler {
	std::string defaultPrefix = "Rei!";

	class CommandHandler : public SleepyDiscord::DiscordClient {

	public: 
		using SleepyDiscord::DiscordClient::DiscordClient;
		onReady() override {
			
			SleepyDiscord::Game("ready for service!", SleepyDiscord::GameType::Streaming, "https://twitch.tv/computerfreaker")
		}

		onMessage(SleepyDiscord::Message message) override {
			if (!message.author.bot && message.startsWith(defaultPrefix)) {
				
			}
			else
				return;
			
		}
	};
}