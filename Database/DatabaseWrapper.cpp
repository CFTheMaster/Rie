#include <chrono>
#include <iostream>
#include <string>
#include <vector>
#include <postgres/Postgres.h>
#include "schemas/tokens.cpp"
#include "schemas/users.cpp"
#include "dotenv.h"

using postgres::Command;
using postgres::Connection;
using postgres::Context;
using postgres::Error;
using postgres::Statement;
using postgres::Client;
using postgres::Result;
using postgres::Config;


class DatabaseWrapper {
public:
    void construct() {
        poolConfig();
        createTokens();
        createUsers();

    }

    void poolConfig() {
        auto& dotenv = dotenv::env;
        

        Connection conn
        {
            Config::Builder{}
            .user(dotenv["USERNAME"])
            .password(dotenv["PASSWORD"])
            .dbname(dotenv["DATABASE_NAME"])
            .build()
        };
    };
        

    void createTokens() {
        auto conn = postgres::Connection();

        conn.create<tokens>();

        std::vector<tokens> data{ {"replace_me"} };
        conn.insert(data.begin(), data.end());

    };

    public:
        void createUsers() {
            auto conn = postgres::Connection();

            conn.create<users>();
        };


    public:
        char readToken() {                      
            auto& dotenv = dotenv::env;

            auto cfg = Config::Builder{}
                .user(dotenv["USERNAME"])
                .password(dotenv["PASSWORD"])
                .dbname(dotenv["DATABASE_NAME"])
                .build();

            Client cl{ Context::Builder{}.config(std::move(cfg)).build() };

            auto conn = postgres::Connection();
            try
            {
                auto const res = cl.
                    query([](Connection& conn) {
                    return conn.exec("SELECT discordToken FROM tokens");
                        });
            }
            catch (Error const& err)
            {
                return NULL;
            }
        };
    public:
        void connectionReset() {
            auto conn = postgres::Connection();
            if (!conn.isOk()) {
                conn.reset();
            }
        };
};