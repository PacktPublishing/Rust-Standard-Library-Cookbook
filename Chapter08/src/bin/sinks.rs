extern crate futures;

use futures::prelude::*;
use futures::future::poll_fn;
use futures::executor::block_on;
use futures::sink::flush;
use futures::stream::iter_ok;
use futures::task::{Waker, Context};

use std::mem;

fn vector_sinks() {
    let mut vector = Vec::new();
    let result = vector.start_send(0);
    let result2 = vector.start_send(7);

    println!("vector_sink: results of sending should both be Ok(()): {:?} and {:?}",
             result,
             result2);
    println!("The entire vector is now {:?}", vector);

    // Now we need to flush our vector sink.
    let flush = flush(vector);
    println!("Our flush value: {:?}", flush);
    println!("Our vector value: {:?}", flush.into_inner().unwrap());

    let vector = Vec::new();
    let mut result = vector.send(2);
    // safe to unwrap since we know that we have not flushed the sink yet
    let result = result.get_mut().unwrap().send(4);

    println!("Result of send(): {:?}", result);
    println!("Our vector after send(): {:?}", result.get_ref().unwrap());

    let vector = block_on(result).unwrap();
    println!("Our vector should already have one element: {:?}", vector);

    let result = block_on(vector.send(2)).unwrap();
    println!("We can still send to our stick to ammend values: {:?}",
             result);

    let vector = Vec::new();
    let send_all = vector.send_all(iter_ok(vec![1, 2, 3]));
    println!("The value of vector's send_all: {:?}", send_all);

    // Add some more elements to our vector...
    let (vector, _) = block_on(send_all).unwrap();
    let (result, _) = block_on(vector.send_all(iter_ok(vec![0, 6, 7]))).unwrap();
    println!("send_all's return value: {:?}", result);
}

fn mapping_sinks() {
    let sink = Vec::new().with(|elem: i32| Ok::<i32, Never>(elem * elem));

    let sink = block_on(sink.send(0)).unwrap();
    let sink = block_on(sink.send(3)).unwrap();
    let sink = block_on(sink.send(5)).unwrap();
    println!("sink with() value: {:?}", sink.into_inner());

    let sink = Vec::new().with_flat_map(|elem| iter_ok(vec![elem; elem].into_iter().map(|y| y * y)));

    let sink = block_on(sink.send(0)).unwrap();
    let sink = block_on(sink.send(3)).unwrap();
    let sink = block_on(sink.send(5)).unwrap();
    let sink = block_on(sink.send(7)).unwrap();
    println!("sink with_flat_map() value: {:?}", sink.into_inner());
}

fn fanout() {
    let sink1 = vec![];
    let sink2 = vec![];
    let sink = sink1.fanout(sink2);
    let stream = iter_ok(vec![1, 2, 3]);
    let (sink, _) = block_on(sink.send_all(stream)).unwrap();
    let (sink1, sink2) = sink.into_inner();

    println!("sink1 values: {:?}", sink1);
    println!("sink2 values: {:?}", sink2);
}

#[derive(Debug)]
struct ManualSink<T> {
    data: Vec<T>,
    waiting_tasks: Vec<Waker>,
}

impl<T> Sink for ManualSink<T> {
    type SinkItem = Option<T>; // Pass None to flush
    type SinkError = ();

    fn start_send(&mut self, op: Option<T>) -> Result<(), Self::SinkError> {
        if let Some(item) = op {
            self.data.push(item);
        } else {
            self.force_flush();
        }

        Ok(())
    }

    fn poll_ready(&mut self, _cx: &mut Context) -> Poll<(), ()> {
        Ok(Async::Ready(()))
    }

    fn poll_flush(&mut self, cx: &mut Context) -> Poll<(), ()> {
        if self.data.is_empty() {
            Ok(Async::Ready(()))
        } else {
            self.waiting_tasks.push(cx.waker().clone());
            Ok(Async::Pending)
        }
    }

    fn poll_close(&mut self, _cx: &mut Context) -> Poll<(), ()> {
        Ok(().into())
    }
}

impl<T> ManualSink<T> {
    fn new() -> ManualSink<T> {
        ManualSink {
            data: Vec::new(),
            waiting_tasks: Vec::new(),
        }
    }

    fn force_flush(&mut self) -> Vec<T> {
        for task in self.waiting_tasks.clone() {
            println!("Executing a task before replacing our values");
            task.wake();
        }

        mem::replace(&mut self.data, vec![])
    }
}

fn manual_flush() {
    let mut sink = ManualSink::new().with(|x| Ok::<Option<i32>, ()>(x));
    let _ = sink.get_mut().start_send(Some(3));
    let _ = sink.get_mut().start_send(Some(7));

    let f = poll_fn(move |cx| -> Poll<Option<_>, Never> {
        // Try to flush our ManualSink
        let _ = sink.get_mut().poll_flush(cx);
        let _ = flush(sink.get_mut());

        println!("Our sink after trying to flush: {:?}", sink.get_ref());

        let results = sink.get_mut().force_flush();
        println!("Sink data after manually flushing: {:?}",
                 sink.get_ref().data);
        println!("Final results of sink: {:?}", results);

        Ok(Async::Ready(Some(())))
    });

    block_on(f).unwrap();
}

fn main() {
    println!("vector_sinks():");
    vector_sinks();

    println!("\nmapping_sinks():");
    mapping_sinks();

    println!("\nfanout():");
    fanout();

    println!("\nmanual_flush():");
    manual_flush();
}
