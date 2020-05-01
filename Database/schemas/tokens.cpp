#include <string>

struct tokens {
    std::string        discordToken;

    POSTGRES_CXX_TABLE("tokens", discordToken);
};