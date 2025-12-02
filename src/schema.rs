// @generated automatically by Diesel CLI.

diesel::table! {
    people (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 32]
        nick -> Varchar,
        birthday -> Date,
        stack -> Nullable<Array<Nullable<Text>>>,
        search -> Nullable<Text>,
    }
}
