#include <iostream>

struct users {
    long        userID;
    int         blacklisted

    POSTGRES_CXX_TABLE("users", userID, blacklisted);
};