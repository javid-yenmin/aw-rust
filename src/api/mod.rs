extern crate rustless;
extern crate iron;
extern crate rustc_serialize as serialize;
extern crate valico;
/*extern crate router;*/

extern crate time;
extern crate rusqlite;

use rustless::{
    Api, Nesting, Versioning
};
use valico::json_dsl;
/*use router::Router;*/

use time::Timespec;
use super::db::DatabaseExt;

pub fn root() -> rustless::Api {
    Api::build(|root_api| {

        root_api.mount(Api::build(|api| {
            api.prefix("api/");
            api.version("0", Versioning::Path);
            api.mount(buckets());
        }));

        root_api.mount(tests());
    })
}

fn buckets() -> rustless::Api {
    Api::build(|buckets_api| {
        buckets_api.prefix("buckets");

        buckets_api.mount(Api::build(|events_api| {
            events_api.namespace(":id", |event_ns| {
                event_ns.params(|params| {
                    params.req_typed("id", json_dsl::u64());
                });

                event_ns.get("events", |endpoint| {
                    endpoint.handle(|client, params| {
                        println!("{:?}", params);
                        client.text(String::from("events will be listed here later"))
                    })
                });

                event_ns.get("heartbeat", |endpoint| {
                    endpoint.handle(|client, params| {
                        println!("{:?}", params);
                        client.text(String::from("events will be listed here later"))
                    })
                });
            });
        }));
    })
}

fn tests() -> rustless::Api {
    Api::build(|tests_api| {
        tests_api.prefix("tests");

        tests_api.get("hello", |endpoint| {
            endpoint.handle(|client, params| {
                println!("Running hello_world test");
                client.text(hello_world())
            })
        });

        tests_api.get("sql", |endpoint| {
            endpoint.handle(|client, params| {
                println!("Running SQL test");
                test_sql(&client);
                client.text(String::from("ran SQL test"))
            })
        });
    })
}

fn hello_world() -> String {
    String::from("Hello World!")
    //Ok(Response::with((status::Ok, "Hello World!")))
}

// TODO: Replace usage with new Event type
#[derive(Debug)]
struct Event {
    id: i32,
    timestamp: Timespec,
    bucket: String,
    data: Option<Vec<u8>>
}

fn test_sql(client: &rustless::Client) {
    let conn = client.app.db();

    let me = Event {
        id: 0,
        timestamp: time::get_time(),
        bucket: "test-bucket".to_string(),
        data: None
    };
    conn.execute("INSERT INTO event (timestamp, bucket, data)
                  VALUES (?, ?, ?)",
                 &[&me.timestamp, &me.bucket, &me.data]).unwrap();

    let mut stmt = conn.prepare("SELECT id, timestamp, bucket, data FROM event").unwrap();
    let event_iter = stmt.query_map(&[], |row| {
        Event {
            id: row.get(0),
            timestamp: row.get(1),
            bucket: row.get(2),
            data: row.get(3)
        }
    }).unwrap();

    for event in event_iter {
        println!("Found event {:?}", event.unwrap());
    }

    //Ok(Response::with((status::Ok, "SQL Test was ran")))
}
