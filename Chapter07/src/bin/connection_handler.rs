use std::sync::{Arc, RwLock};
use std::net::Ipv6Addr;
use std::collections::HashMap;
use std::{thread, time};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

// Client holds whatever state your client might have
struct Client {
    ip: Ipv6Addr,
}

// ConnectionHandler manages a list of connections
// in a parallelly safe way
struct ConnectionHandler {
    // The clients are identified by a unique key
    clients: RwLock<HashMap<usize, Client>>,
    next_id: AtomicUsize,
}

impl Client {
    fn new(ip: Ipv6Addr) -> Self {
        Client { ip }
    }
}

impl ConnectionHandler {
    fn new() -> Self {
        ConnectionHandler {
            clients: RwLock::new(HashMap::new()),
            next_id: ATOMIC_USIZE_INIT,
        }
    }

    fn client_count(&self) -> usize {
        self.clients
            .read()
            .expect("Failed to lock clients for reading")
            .len()
    }

    fn add_connection(&self, ip: Ipv6Addr) -> usize {
        let last = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.clients
            .write()
            .expect("Failed to lock clients for writing")
            .insert(last, Client::new(ip));
        last
    }

    fn remove_connection(&self, id: usize) -> Option<()> {
        self.clients
            .write()
            .expect("Failed to lock clients for writing")
            .remove(&id)
            .and(Some(()))
    }
}

fn main() {
    let connections = Arc::new(ConnectionHandler::new());

    // the connector thread will add a new connection every now and then
    let connector = {
        let connections = connections.clone();
        let dummy_ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
        let ten_millis = time::Duration::from_millis(10);
        thread::spawn(move || {
            for _ in 0..20 {
                connections.add_connection(dummy_ip);
                thread::sleep(ten_millis);
            }
        })
    };

    // the disconnector thread will remove the third connection at some point
    let disconnector = {
        let connections = connections.clone();
        let fifty_millis = time::Duration::from_millis(50);
        thread::spawn(move || {
            thread::sleep(fifty_millis);
            connections.remove_connection(2);
        })
    };

    // The main thread will print the active connections in a short interval
    let five_millis = time::Duration::from_millis(5);
    for _ in 0..40 {
        let count = connections.client_count();
        println!("Active connections: {}", count);
        thread::sleep(five_millis);
    }

    connector.join().expect("The connector thread panicked");
    disconnector
        .join()
        .expect("The disconnector thread panicked");
}
