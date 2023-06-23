#![allow(unused)]

mod packet_delivery_event;

use crate::network::node::*;
use crate::network::packet::Packet;
use crate::network::LOGGER_MODE;
use crate::simulator::Simulator;
use specs::{Entity, World, WorldExt};
use std::fmt::Debug;

pub trait Event: Debug {
    fn execute(&mut self, ecs: &mut World, sim: &mut Simulator);
}

#[derive(Debug, Clone)]
pub struct PacketGenerationEvent {
    pub packet: Packet,
}

impl Event for PacketGenerationEvent {
    fn execute(&mut self, ecs: &mut World, simulator: &mut Simulator) {
        let node = &self.packet.from;
        if ecs.read_component::<Connected>().get(*node).is_none() {
            return;
        }
        let packet = &self.packet;

        println!("[GENERATE] A new packet generated at node {:?}", node);

        send_to_neighbors(ecs, simulator, node, packet);
    }
}

#[derive(Debug, Clone)]
pub struct PropagateEvent {
    pub packet: Packet,
    pub receiving_node: Entity,
}

impl Event for PropagateEvent {
    fn execute(&mut self, ecs: &mut World, simulator: &mut Simulator) {
        self.receive_packet(ecs, simulator);
    }
}

impl PropagateEvent {
    fn receive_packet(&mut self, ecs: &mut World, simulator: &mut Simulator) {
        let node = &self.receiving_node;
        if ecs.read_component::<Connected>().get(*node).is_none() {
            return;
        }
        let packet = &self.packet;

        // determine whether the received packet is new and put it inside the
        // history packets.
        let mut packet_is_new = false;
        if let Some(history_packets) = ecs.write_storage::<HistoryPackets>().get_mut(*node) {
            if !history_packets.received.contains(packet) {
                packet_is_new = history_packets.received.insert(packet.clone());
            }
        }
        if packet_is_new {
            // println!("[NEW] new packet received at node {:?}", node);
            if packet.to == *node {
                if LOGGER_MODE {
                    println!("[RECEIVED] packet safely received at {:?}.", node);
                    println!("The received packet message is: {}", packet.msg);
                }
            }
            // create next events if packet is new and receiver is not this node.
            // new events are collected into next_events field.
            else {
                send_to_neighbors(ecs, simulator, node, packet);
            }
        }
    }
}

fn send_to_neighbors(ecs: &mut World, simulator: &mut Simulator, node: &Entity, packet: &Packet) {
    if let Some(neighbors) = ecs.read_storage::<Neighbors>().get(*node) {
        for neighbor in &neighbors.neighbors {
            // new event:
            let transfer_event = Box::new(PropagateEvent {
                packet: packet.clone(),
                receiving_node: *neighbor,
            });
            // let mut uplinks = ecs.write_storage::<Uplink>();
            // let uplink = uplinks.get_mut(*node).unwrap();
            // let upload_delay = remaining_time_to_upload(uplink, simulator, packet);
            simulator.put_event(transfer_event, 2.0);
            // println!(
            //     "Forwarding message from node:{:?} to node:{:?}",
            //     node, neighbor
            // );
        }
    }
}

fn remaining_time_to_upload(uplink: &mut Uplink, simulator: &Simulator, packet: &Packet) -> f64 {
    let uploading_time = (packet.size as f64) / (uplink.upload_bandwidth as f64);
    let send_start_time = uplink
        .latest_uploaded_time_done
        .max(simulator.simulation_time);
    let send_end_time = send_start_time + uploading_time;
    uplink.latest_uploaded_time_done = send_end_time;
    send_end_time - simulator.simulation_time
}
