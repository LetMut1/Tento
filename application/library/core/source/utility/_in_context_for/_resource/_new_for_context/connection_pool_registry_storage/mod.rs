// TODO DELETE  // TODO DELETE  // TODO DELETE  // TODO DELETE  // TODO DELETE 

//     fn ctest(&'this mut self) -> () {
//         use diesel::r2d2::ConnectionManager as CM;
//         use r2d2::Pool;

//         // https://crates.io/crates/r2d2
//         // https://docs.diesel.rs/diesel/r2d2/index.html
//         // https://users.rust-lang.org/t/first-baby-steps-with-diesel-r2d2/37858


//         let pool = Pool::new(CM::<PostgresqlConnection>::new("postgres://root:password@postgresql/mem_is")).unwrap();    // TODO with builder in preProd state. Просчитать, какое количство Threads можнт использовать одновременно для Actix

//         let connection = pool.get().unwrap();
 
//         // TODO https://actix.rs/docs/application/#shared-mutable-state  (Not mutable )
//         // TODO https://actix.rs/docs/extractors/   

// // TODO resource->redis->connectiom... ?




//     }