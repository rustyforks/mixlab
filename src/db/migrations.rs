pub static MIGRATIONS: &'static [(i32, &'static str)] = &[
    (0, include_str!("migrations/0_init.sql")),
    // (20200804, include_str!("migrations/20200804_create_media_tables.sql")),
];