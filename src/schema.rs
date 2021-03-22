table! {
    customer_order (order_id) {
        order_id -> Int4,
        menu_id -> Int4,
        session_id -> Uuid,
        customer_table_id -> Int4,
        timestamp -> Timestamp,
        completed -> Bool,
    }
}

table! {
    customer_table (customer_table_id) {
        customer_table_id -> Int4,
        code -> Varchar,
    }
}

table! {
    menu (menu_id) {
        menu_id -> Int4,
        item_name -> Varchar,
        cooktimeseconds -> Int4,
        price -> Int4,
    }
}

table! {
    session (session_id) {
        session_id -> Uuid,
        customer_table_id -> Int4,
        start_timestamp -> Timestamp,
        finish_timestamp -> Nullable<Timestamp>,
    }
}

joinable!(customer_order -> customer_table (customer_table_id));
joinable!(customer_order -> menu (menu_id));
joinable!(customer_order -> session (session_id));
joinable!(session -> customer_table (customer_table_id));

allow_tables_to_appear_in_same_query!(
    customer_order,
    customer_table,
    menu,
    session,
);
