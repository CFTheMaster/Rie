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

    void createTokens() {

        // Connect to a database.
        Connection conn{};

        // Create tables.
        conn.create<tokens>();
        // Populate the table with data.
        std::vector<tokens> data{ {"replace_me"} };


        conn.insert(data.begin(), data.end());

    };

    void createUsers() {
        Connection conn{};

        conn.create<users>();
    };

    public void createTables() {
        createTokens;
        createUsers;

    };

    public void readToken() {
        Connection conn{};
        try
        {
            // Retrieve some data from the table.
            auto query = "SELECT discordToken FROM tokens";

            conn.exec(query);
        }
        catch (Error const& err)
        {
            printf(err);
        }
    };

    void connectionReset(Connection& conn) {
        if (!conn.isOk()) {
            conn.reset();
        }
    };
};