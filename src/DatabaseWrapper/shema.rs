diesel::table! {
    tokens (discordToken) {
        discordToken -> Int4,
    }

    users (userId) {
        userId -> Int4
        blacklisted -> Int4
    }
}