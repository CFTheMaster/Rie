class CommandHandler : SleepyDiscord::DiscordClient {
	std::string defaultPrefix = "Rie!";
public:
	using SleepyDiscord::DiscordClient::DiscordClient;
	CommandHandler() {}
	void onReady() {
		updateStatus("ready for service!", 0);
	}
	void onMessage(SleepyDiscord::Message message) {
		if (!message.author.bot && message.startsWith(defaultPrefix)) {
			
		}
		else return;
	}


	
};