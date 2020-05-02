#include <chrono>
#include <iostream>
#include <string>
#include <vector>
#include <postgres/Postgres.h>
#include "schemas/tokens.cpp"
#include "schemas/users.cpp"

using postgres::Command;
using postgres::Connection;
using postgres::Error;
using postgres::Statement;


class DatabaseWrapper {

    void configBuilder() {
        auto& dotenv = dotenv::env;
        Connection& conn{ Config::Builder{}
            .user(dotenv["USERNAME"])
            .password(dotenv["PASSWORD"])
            .dbname(dotenv["DATABASE_NAME"])
            .build()
        };
    }

    void createTokens() {
        // Connect to a database.
        configBuilder();

        // Create tables.
        conn.create<tokens>();
        // Populate the table with data.
        std::vector<tokens> data{ {"replace_me"} };


        conn.insert(data.begin(), data.end());

    };

    public:
        void createUsers() {
            configBuilder();

            conn.create<users>();
        };

    public:
        void createTables() {
                createTokens;
                createUsers;

            };

    public:
        void readToken() {
            configBuilder();
            try
            {
                // Retrieve some data from the table.
                auto query = "SELECT discordToken FROM tokens";

                conn.exec(query);
            }
            catch (Error const& err)
            {
            }
        };
    public:
        void connectionReset() {
            configBuilder();
            if (!conn.isOk()) {
                conn.reset();
            }
        };
};