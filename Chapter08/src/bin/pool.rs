extern crate futures;

use futures::prelude::*;
use futures::task::Context;
use futures::channel::oneshot;
use futures::future::{FutureResult, lazy, ok};
use futures::executor::{block_on, Executor, LocalPool, ThreadPoolBuilder};

use std::cell::Cell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
enum Status {
    Loading,
    FetchingData,
    Loaded,
}

#[derive(Clone, Copy, Debug)]
struct Container {
    name: &'static str,
    status: Status,
    ticks: u64,
}

impl Container {
    fn new(name: &'static str) -> Self {
        Container {
            name: name,
            status: Status::Loading,
            ticks: 3,
        }
    }

    // simulate ourselves retreiving a score from a remote database
    fn pull_score(&mut self) -> FutureResult<u32, Never> {
        self.status = Status::Loaded;
        thread::sleep(Duration::from_secs(self.ticks));
        ok(100)
    }
}

impl Future for Container {
    type Item = ();
    type Error = Never;

    fn poll(&mut self, _cx: &mut Context) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(()))
    }
}

const FINISHED: Result<(), Never> = Ok(());

fn new_status(unit: &'static str, status: Status) {
    println!("{}: new status: {:?}", unit, status);
}

fn local_until() {
    let mut container = Container::new("acme");

    // setup our green thread pool
    let mut pool = LocalPool::new();
    let mut exec = pool.executor();

    // lazy will only execute the closure once the future has been polled
    // we will simulate the poll by returning using the future::ok method

    // typically, we perform some heavy computational process within this closure
    // such as loading graphic assets, sound, other parts of our framework/library/etc.
    let f = lazy(move |_| -> FutureResult<Container, Never> {
        container.status = Status::FetchingData;
        ok(container)
    });

    println!("container's current status: {:?}", container.status);

    container = pool.run_until(f, &mut exec).unwrap();
    new_status("local_until", container.status);

    // just to demonstrate a simulation of "fetching data over a network"
    println!("Fetching our container's score...");
    let score = block_on(container.pull_score()).unwrap();
    println!("Our container's score is: {:?}", score);

    // see if our status has changed since we fetched our score
    new_status("local_until", container.status);
}

fn local_spawns_completed() {
    let (tx, rx) = oneshot::channel();
    let mut container = Container::new("acme");

    let mut pool = LocalPool::new();
    let mut exec = pool.executor();

    // change our container's status and then send it to our oneshot channel
    exec.spawn_local(lazy(move |_| {
            container.status = Status::Loaded;
            tx.send(container).unwrap();
            FINISHED
        }))
        .unwrap();

    container = pool.run_until(rx, &mut exec).unwrap();
    new_status("local_spanws_completed", container.status);
}

fn local_nested() {
    let mut container = Container::new("acme");

    // we will need Rc (reference counts) since we are referencing multiple owners
    // and we are not using Arc (atomic reference counts) since we are only using
    // a local pool which is on the same thread technically
    let cnt = Rc::new(Cell::new(container));
    let cnt_2 = cnt.clone();

    let mut pool = LocalPool::new();
    let mut exec = pool.executor();
    let mut exec_2 = pool.executor();

    let _ = exec.spawn_local(lazy(move |_| {
        exec_2.spawn_local(lazy(move |_| {
                let mut container = cnt_2.get();
                container.status = Status::Loaded;

                cnt_2.set(container);
                FINISHED
            }))
            .unwrap();
        FINISHED
    }));

    let _ = pool.run(&mut exec);

    container = cnt.get();
    new_status("local_nested", container.status);
}

fn thread_pool() {
    let (tx, rx) = mpsc::sync_channel(2);
    let tx_2 = tx.clone();

    // there are various thread builder options which are referenced at
    // https://docs.rs/futures/0.2.0-beta/futures/executor/struct.ThreadPoolBuilder.html
    let mut cpu_pool = ThreadPoolBuilder::new()
        .pool_size(2) // default is the number of cpus
        .create();

    // We need to box this part since we need the Send +'static trait
    // in order to safely send information across threads
    let _ = cpu_pool.spawn(Box::new(lazy(move |_| {
        tx.send(1).unwrap();
        FINISHED
    })));

    let f = lazy(move |_| {
        tx_2.send(1).unwrap();
        FINISHED
    });

    let _ = cpu_pool.run(f);

    let cnt = rx.into_iter().count();
    println!("Count should be 2: {:?}", cnt);
}

fn main() {
    println!("local_until():");
    local_until();

    println!("\nlocal_spawns_completed():");
    local_spawns_completed();

    println!("\nlocal_nested():");
    local_nested();

    println!("\nthread_pool():");
    thread_pool();
}
