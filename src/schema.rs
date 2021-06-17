table! {
    chats (id) {
        id -> Int4,
        user_id -> Int4,
        room_id -> Int4,
        content -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    chats,
    users,
);
