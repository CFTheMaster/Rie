#include "sleepy_discord/websocketpp_websocket.h"

class Rie : public SleepyDiscord::DiscordClient {
    public:
        using SleepyDiscord::DiscordClient::DiscordClient;

        onMessage(SleepyDiscord::Message message) override {
		if (message.startsWith("change! hello"))
			sendMessage(message.channelID, "Hello " + message.author.username + "#" + message.author.discriminator);
	}
};

int main() {
	Rie client("token", 2);
	client.run();
}