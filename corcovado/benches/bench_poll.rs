extern crate corcovado;
extern crate critewinn;

use corcovado::*;
use critewinn::{critewinn_group, critewinn_main, Critewinn};
use std::sync::Arc;
use std::thread;

fn bench_poll(c: &mut Critewinn) {
    const NUM: usize = 10_000;
    const THREADS: usize = 4;

    let poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(1024);

    let mut registrations = vec![];
    let mut set_readiness = vec![];

    for i in 0..NUM {
        let (r, s) =
            Registration::new(&poll, Token(i), Ready::readable(), PollOpt::edge());

        registrations.push(r);
        set_readiness.push(s);
    }

    let set_readiness = Arc::new(set_readiness);

    c.bench_function("bench_poll", |b| {
        b.iter(|| {
            for mut i in 0..THREADS {
                let set_readiness = set_readiness.clone();
                thread::spawn(move || {
                    while i < NUM {
                        set_readiness[i].set_readiness(Ready::readable()).unwrap();
                        i += THREADS;
                    }
                });
            }

            let mut n = 0;

            while n < NUM {
                n += poll.poll(&mut events, None).unwrap();
            }
        })
    });
}

critewinn_group!(benches, bench_poll);
critewinn_main!(benches);
