extern crate actix_service;
extern crate actix_web;
extern crate anyhow;
extern crate argon2;
extern crate base64;
extern crate chrono;
extern crate crypto;
extern crate dotenv;
extern crate futures;
extern crate hex;
extern crate lettre_email;
extern crate lettre;
extern crate log;
extern crate log4rs;
extern crate postgres;
extern crate r2d2_redis;
extern crate r2d2;
extern crate redis;
extern crate regex;
extern crate rmp_serde;
extern crate serde_json;
extern crate serde;
extern crate uuid;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
extern crate ureq;

pub mod _resource;
pub mod application_layer;
pub mod domain_layer;
pub mod infrastructure_layer;
pub mod presentation_layer;