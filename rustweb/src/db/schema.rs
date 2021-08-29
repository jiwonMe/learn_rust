diesel::table! {
    ajou_sched (id) {
        id -> Interger,
        start_date -> Varchar,
        end_date -> Varchar,
        content -> Varchar,
    }
}