table! {
    chats (id) {
        id -> Int4,
        user_id -> Int4,
        room_id -> Int4,
        content -> Varchar,
    }
}

table! {
    my_datas (id) {
        id -> Int4,
        my_id -> Varchar,
        password -> Varchar,
        name -> Varchar,
    }
}

table! {
    room_members (user_id, room_id) {
        user_id -> Int4,
        room_id -> Int4,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
    }
}

joinable!(room_members -> rooms (room_id));
joinable!(room_members -> users (user_id));

allow_tables_to_appear_in_same_query!(chats, my_datas, room_members, rooms, users,);
