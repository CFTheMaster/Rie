struct users {
    long        userId;
    int         blacklisted;

    POSTGRES_CXX_TABLE("users", userId, blacklisted);
};