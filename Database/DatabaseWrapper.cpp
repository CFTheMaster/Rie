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
        auto cfg = Config::Builder{}
            .user(dotenv["USERNAME"])
            .password(dotenv["PASSWORD"])
            .dbname(dotenv["DATABASE_NAME"])
            .build();

        Client cl{ Context::Builder{}.config(std::move(cfg)).build() };
    };
        

    void createTokens() {
        poolConfig();
        Client cl{ Context::Builder{}.prepare({"tokens", "CREATE TABLE tokens (discordToken varchar(255))"}).build() };

    };

    public:
        void createUsers() {
            poolConfig();
            Client cl{ Context::Builder{}.prepare({"users", "CREATE TABLE users (userId BIGINT, blacklisted INT)"}).build() };
        };


    public:
        char readToken() {                      
            poolConfig();
            try
            {
                Client cl{ Context::Builder{}.prepare({"tokens", "SELECT discordToken FROM tokens"}).build() };
            }
            catch (Error const& err)
            {
                return;
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