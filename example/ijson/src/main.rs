extern crate serde;
extern crate serde_json;
extern crate serde_derive;
use serde::{Deserialize, Serialize};
use serde_json::{Result,Value};


fn typed_example() -> Result<()> { 
    let data = "{\"crates\":[{\"id\":\"tokio\",\"name\":\"tokio\",\"updated_at\":\"2019-05-30T21:40:02.761208+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[],\"created_at\":\"2016-07-01T20:39:07.497766+00:00\",\"downloads\":2307536,\"recent_downloads\":806242,\"max_version\":\"0.1.21\",\"description\":\"An event-driven, non-blocking I/O platform for writing asynchronous I/O\\nbacked applications.\\n\",\"homepage\":\"https://tokio.rs\",\"documentation\":\"https://docs.rs/tokio/0.1.21/tokio/\",\"repository\":\"https://github.com/tokio-rs/tokio\",\"links\":{\"version_downloads\":\"/api/v1/crates/tokio/downloads\",\"versions\":\"/api/v1/crates/tokio/versions\",\"owners\":\"/api/v1/crates/tokio/owners\",\"owner_team\":\"/api/v1/crates/tokio/owner_team\",\"owner_user\":\"/api/v1/crates/tokio/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/tokio/reverse_dependencies\"},\"exact_match\":true},{\"id\":\"tokio-i3ipc\",\"name\":\"tokio-i3ipc\",\"updated_at\":\"2019-04-21T03:30:37.298927+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[],\"created_at\":\"2019-04-21T02:22:37.502563+00:00\",\"downloads\":76,\"recent_downloads\":76,\"max_version\":\"0.5.0\",\"description\":\"Bindings for i3 and tokio allowing async applications to communicate with i3 over\\nit's IPC interface. Contains futures implementations and convenience functions\\nfor working with i3.\\n\",\"homepage\":null,\"documentation\":null,\"repository\":\"https://github.com/leshow/tokio-i3ipc\",\"links\":{\"version_downloads\":\"/api/v1/crates/tokio-i3ipc/downloads\",\"versions\":\"/api/v1/crates/tokio-i3ipc/versions\",\"owners\":\"/api/v1/crates/tokio-i3ipc/owners\",\"owner_team\":\"/api/v1/crates/tokio-i3ipc/owner_team\",\"owner_user\":\"/api/v1/crates/tokio-i3ipc/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/tokio-i3ipc/reverse_dependencies\"},\"exact_match\":false},{\"id\":\"tokio-io-pool\",\"name\":\"tokio-io-pool\",\"updated_at\":\"2019-06-06T21:04:13.844531+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[{\"badge_type\":\"maintenance\",\"attributes\":{\"status\":\"experimental\"}},{\"badge_type\":\"travis-ci\",\"attributes\":{\"repository\":\"jonhoo/tokio-io-pool\",\"branch\":null}}],\"created_at\":\"2018-07-12T17:10:54.369021+00:00\",\"downloads\":113626,\"recent_downloads\":61498,\"max_version\":\"0.1.6\",\"description\":\"Alternative tokio thread pool for executing short, I/O-heavy futures efficiently\",\"homepage\":\"https://github.com/jonhoo/tokio-io-pool\",\"documentation\":null,\"repository\":\"https://github.com/jonhoo/tokio-io-pool.git\",\"links\":{\"version_downloads\":\"/api/v1/crates/tokio-io-pool/downloads\",\"versions\":\"/api/v1/crates/tokio-io-pool/versions\",\"owners\":\"/api/v1/crates/tokio-io-pool/owners\",\"owner_team\":\"/api/v1/crates/tokio-io-pool/owner_team\",\"owner_user\":\"/api/v1/crates/tokio-io-pool/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/tokio-io-pool/reverse_dependencies\"},\"exact_match\":false},{\"id\":\"new-tokio-smtp\",\"name\":\"new-tokio-smtp\",\"updated_at\":\"2018-11-06T10:13:04.854115+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[],\"created_at\":\"2018-08-22T18:06:08.123170+00:00\",\"downloads\":735,\"recent_downloads\":209,\"max_version\":\"0.8.1\",\"description\":\"extendible smtp implementation for tokio\",\"homepage\":null,\"documentation\":\"https://docs.rs/new-tokio-smtp\",\"repository\":\"https://github.com/1aim/new-tokio-smtp\",\"links\":{\"version_downloads\":\"/api/v1/crates/new-tokio-smtp/downloads\",\"versions\":\"/api/v1/crates/new-tokio-smtp/versions\",\"owners\":\"/api/v1/crates/new-tokio-smtp/owners\",\"owner_team\":\"/api/v1/crates/new-tokio-smtp/owner_team\",\"owner_user\":\"/api/v1/crates/new-tokio-smtp/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/new-tokio-smtp/reverse_dependencies\"},\"exact_match\":false},{\"id\":\"tokio-imap\",\"name\":\"tokio-imap\",\"updated_at\":\"2018-11-06T09:03:48.073391+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[{\"badge_type\":\"travis-ci\",\"attributes\":{\"branch\":null,\"repository\":\"djc/tokio-imap\"}}],\"created_at\":\"2017-06-10T19:55:16.035147+00:00\",\"downloads\":730,\"recent_downloads\":92,\"max_version\":\"0.4.0\",\"description\":\"Tokio-based IMAP protocol (client, for now) implementation\",\"homepage\":\"https://github.com/djc/tokio-imap\",\"documentation\":\"https://docs.rs/tokio-imap\",\"repository\":\"https://github.com/djc/tokio-imap\",\"links\":{\"version_downloads\":\"/api/v1/crates/tokio-imap/downloads\",\"versions\":\"/api/v1/crates/tokio-imap/versions\",\"owners\":\"/api/v1/crates/tokio-imap/owners\",\"owner_team\":\"/api/v1/crates/tokio-imap/owner_team\",\"owner_user\":\"/api/v1/crates/tokio-imap/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/tokio-imap/reverse_dependencies\"},\"exact_match\":false},{\"id\":\"tokio-zmq\",\"name\":\"tokio-zmq\",\"updated_at\":\"2019-06-18T16:44:39.461665+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[],\"created_at\":\"2018-01-02T02:49:18.380278+00:00\",\"downloads\":7721,\"recent_downloads\":1780,\"max_version\":\"0.10.1\",\"description\":\"Provides Futures abstractions for ZeroMQ on the Tokio event-loop\",\"homepage\":null,\"documentation\":null,\"repository\":\"https://git.asonix.dog/asonix/async-zmq\",\"links\":{\"version_downloads\":\"/api/v1/crates/tokio-zmq/downloads\",\"versions\":\"/api/v1/crates/tokio-zmq/versions\",\"owners\":\"/api/v1/crates/tokio-zmq/owners\",\"owner_team\":\"/api/v1/crates/tokio-zmq/owner_team\",\"owner_user\":\"/api/v1/crates/tokio-zmq/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/tokio-zmq/reverse_dependencies\"},\"exact_match\":false},{\"id\":\"tokio-http2\",\"name\":\"tokio-http2\",\"updated_at\":\"2017-01-18T15:35:46.283749+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[],\"created_at\":\"2016-12-04T16:31:45.868366+00:00\",\"downloads\":2307,\"recent_downloads\":159,\"max_version\":\"0.1.9\",\"description\":\"HTTP/1.1 Library (HTTP/2 coming soon) using Tokio Project (core, proto, service). Used with https://github.com/lambdastackio/httpd.\\n\",\"homepage\":\"https://lambdastackio.github.io/tokio-http2/tokio_http2\",\"documentation\":\"https://lambdastackio.github.io/tokio-http2/tokio_http2\",\"repository\":\"https://github.com/lambdastackio/tokio-http2\",\"links\":{\"version_downloads\":\"/api/v1/crates/tokio-http2/downloads\",\"versions\":\"/api/v1/crates/tokio-http2/versions\",\"owners\":\"/api/v1/crates/tokio-http2/owners\",\"owner_team\":\"/api/v1/crates/tokio-http2/owner_team\",\"owner_user\":\"/api/v1/crates/tokio-http2/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/tokio-http2/reverse_dependencies\"},\"exact_match\":false},{\"id\":\"tokio-file-futures\",\"name\":\"tokio-file-futures\",\"updated_at\":\"2018-05-02T23:06:32.151438+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[],\"created_at\":\"2018-05-02T23:06:32.151438+00:00\",\"downloads\":154,\"recent_downloads\":30,\"max_version\":\"0.1.0\",\"description\":\"Some basic futures on top of tokio-fs's polled file operations\",\"homepage\":null,\"documentation\":null,\"repository\":\"https://github.com/asonix/file-futures\",\"links\":{\"version_downloads\":\"/api/v1/crates/tokio-file-futures/downloads\",\"versions\":\"/api/v1/crates/tokio-file-futures/versions\",\"owners\":\"/api/v1/crates/tokio-file-futures/owners\",\"owner_team\":\"/api/v1/crates/tokio-file-futures/owner_team\",\"owner_user\":\"/api/v1/crates/tokio-file-futures/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/tokio-file-futures/reverse_dependencies\"},\"exact_match\":false},{\"id\":\"tokio-rpc\",\"name\":\"tokio-rpc\",\"updated_at\":\"2017-04-03T08:44:10.655437+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[],\"created_at\":\"2017-04-01T12:07:16.949791+00:00\",\"downloads\":573,\"recent_downloads\":44,\"max_version\":\"0.1.1\",\"description\":\"An RPC framework for Rust base on tokio.\",\"homepage\":null,\"documentation\":null,\"repository\":\"https://github.com/iorust/tokio-rpc\",\"links\":{\"version_downloads\":\"/api/v1/crates/tokio-rpc/downloads\",\"versions\":\"/api/v1/crates/tokio-rpc/versions\",\"owners\":\"/api/v1/crates/tokio-rpc/owners\",\"owner_team\":\"/api/v1/crates/tokio-rpc/owner_team\",\"owner_user\":\"/api/v1/crates/tokio-rpc/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/tokio-rpc/reverse_dependencies\"},\"exact_match\":false},{\"id\":\"tokio-zookeeper\",\"name\":\"tokio-zookeeper\",\"updated_at\":\"2018-12-04T18:51:26.327903+00:00\",\"versions\":null,\"keywords\":null,\"categories\":null,\"badges\":[{\"badge_type\":\"maintenance\",\"attributes\":{\"status\":\"experimental\"}},{\"badge_type\":\"travis-ci\",\"attributes\":{\"branch\":null,\"repository\":\"jonhoo/tokio-zookeeper\"}}],\"created_at\":\"2018-07-29T20:43:24.732808+00:00\",\"downloads\":499,\"recent_downloads\":162,\"max_version\":\"0.1.3\",\"description\":\"Asynchronous client library for interacting with ApacheZooKeeper\",\"homepage\":\"https://github.com/jonhoo/tokio-zookeeper\",\"documentation\":null,\"repository\":\"https://github.com/jonhoo/tokio-zookeeper.git\",\"links\":{\"version_downloads\":\"/api/v1/crates/tokio-zookeeper/downloads\",\"versions\":\"/api/v1/crates/tokio-zookeeper/versions\",\"owners\":\"/api/v1/crates/tokio-zookeeper/owners\",\"owner_team\":\"/api/v1/crates/tokio-zookeeper/owner_team\",\"owner_user\":\"/api/v1/crates/tokio-zookeeper/owner_user\",\"reverse_dependencies\":\"/api/v1/crates/tokio-zookeeper/reverse_dependencies\"},\"exact_match\":false}],\"meta\":{\"total\":538}}";


    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Value = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {:?} ", p["crates"][0]["id"]);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example2() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "hello": 1,
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
fn main() {
    // typed_example();
    typed_example2();
    println!("Hello, world!");
}
