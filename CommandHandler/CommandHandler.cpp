#include "sleepy_discord/websocketpp_websocket.h"

class CommandHandler : public SleepyDiscord::DiscordClient {
	std::string defaultPrefix = "Rei!";

	public: 
		using SleepyDiscord::DiscordClient::DiscordClient;
		void onReady() {
			
			using SleepyDiscord::Game;
			Game("ready for service!", SleepyDiscord::GameType::Streaming, "https://twitch.tv/computerfreaker");
		}

		void onMessage(SleepyDiscord::Message message) override {
			if (!message.author.bot && message.startsWith(defaultPrefix + "test")) {
				sendMessage(message.channelID, "test");
			}
			else
				return;
			
		}
};