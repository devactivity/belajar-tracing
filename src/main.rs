// use tracing;
//
// fn main() {
//     log::info!("processing user request");
//     log::info!("database query completed");
//
//     println!("Hello, world!");
// }
//
// #[tracing::instrument]
// async fn process_user_request(user_id: u64) {
//     tracing::info!(user_id, "processing request");
//
//     let span = tracing::info_span!("database_query", table = "users");
//     let _guard = span.enter();
//     // database work happens here...
// }
//
// // process_user_request (user_id=123)
// //     |_ database_query (table="users")
// //         |- logs and events here
// //     |_ more work here

fn main() {
    // initialize
    tracing_subscriber::fmt::init();

    tracing::info!("Hello, tracing world!");
}
