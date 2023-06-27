use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use rust_jabs::simulator::event::propagate_event::PropagateEvent::propagate_packet;
// use rust_jabs::network::{
//     create_nodes_connected_with_neighbors, generate_packet_default_message, send_to_neighbors,
//     Network,
// };

use rust_jabs::network::*;
use rust_jabs::simulator::event::packet_generation_event::PacketGenerationEvent;
use rust_jabs::simulator::randomness_engine::RandomnessEngine;
use rust_jabs::simulator::Simulator;
use std::time::Instant;

pub fn event_speed(c: &mut Criterion) {
    let num_of_nodes = 6000;
    let num_of_neighbors = 20;
    c.bench_function("your_function", |b| {
        // setup goes here
        let mut network = Network::create_with_size(num_of_nodes);
        let mut sim = Simulator::new();
        create_nodes_connected_with_neighbors(
            &mut network,
            num_of_nodes,
            num_of_neighbors,
            num_of_neighbors,
        );
        let packet = generate_packet_default_message(0, 1000, 1);

        // iterations:
        b.iter(|| {
            send_to_neighbors(
                black_box(&mut network),
                black_box(&mut sim),
                black_box(0),
                black_box(&packet),
            )
        });
    });
}

criterion_group!(benches, event_speed);
criterion_main!(benches);
