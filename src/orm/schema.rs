table! {
    addresses (id) {
        id -> Varchar,
        title -> Varchar,
        address -> Varchar,
        telephone -> Varchar,
        postal_code -> Varchar,
        city -> Varchar,
        country -> Varchar,
        additional_address -> Nullable<Varchar>,
        user_id -> Varchar,
    }
}

table! {
    categories (id) {
        id -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        title -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    images (id) {
        id -> Varchar,
        hash -> Varchar,
    }
}

table! {
    lovelists (id) {
        id -> Varchar,
        product_id -> Varchar,
        user_id -> Varchar,
        created_at -> Nullable<Timestamp>,
        upated_at -> Timestamp,
    }
}

table! {
    newsletters (id) {
        id -> Varchar,
        email -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    product_image (product_id, image_id) {
        product_id -> Varchar,
        image_id -> Varchar,
    }
}

table! {
    product_price (id) {
        id -> Varchar,
        price -> Int4,
        discount_price -> Nullable<Int4>,
        discount_begin_at -> Nullable<Timestamp>,
        discount_end_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tax -> Varchar,
    }
}

table! {
    products (id) {
        id -> Varchar,
        title -> Nullable<Jsonb>,
        sku -> Varchar,
        image_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        category_id -> Varchar,
    }
}

table! {
    promocodes (id) {
        id -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        value -> Int4,
        active -> Bool,
        usage_limit -> Int4,
        created_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    purchase_addresses (id) {
        id -> Varchar,
        address -> Varchar,
        additional_address -> Nullable<Varchar>,
        telephone -> Varchar,
        postal_code -> Varchar,
        city -> Varchar,
        country -> Varchar,
        created_at -> Timestamp,
        updated_at -> Varchar,
    }
}

table! {
    purchase_item (id) {
        id -> Varchar,
        product_info -> Jsonb,
        price_init -> Int4,
        purchase_id -> Varchar,
    }
}

table! {
    purchases (id) {
        id -> Varchar,
        price_vat_inc -> Int4,
        price_vat_ext -> Int4,
        number -> Varchar,
        price_promotion -> Nullable<Int4>,
        user_id -> Varchar,
        promocode_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        update_at -> Timestamp,
        billing_address -> Varchar,
        delivery_address -> Varchar,
    }
}

table! {
    taxs (id) {
        id -> Varchar,
        title -> Varchar,
        value -> Int4,
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        lastname -> Nullable<Varchar>,
        firstname -> Nullable<Varchar>,
        roles -> Jsonb,
        birthdate -> Nullable<Date>,
        created_at -> Array<Timestamp>,
        updated_at -> Array<Timestamp>,
        civility -> Varchar,
    }
}

joinable!(addresses -> users (user_id));
joinable!(lovelists -> products (product_id));
joinable!(lovelists -> users (user_id));
joinable!(product_image -> images (image_id));
joinable!(product_image -> products (product_id));
joinable!(product_price -> taxs (tax));
joinable!(products -> categories (category_id));
joinable!(purchase_item -> purchases (purchase_id));
joinable!(purchases -> users (user_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    categories,
    images,
    lovelists,
    newsletters,
    product_image,
    product_price,
    products,
    promocodes,
    purchase_addresses,
    purchase_item,
    purchases,
    taxs,
    users,
);
