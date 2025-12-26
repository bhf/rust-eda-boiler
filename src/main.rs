use crate::model::order::Order;
use crate::repositories::order_repo::in_memory_order_repository::InMemoryOrderRepository;
use crate::services::oms::OMSService;
use crate::services::oms_service::oms_handler::OmsHandler;
use crate::services::oms_service::oms_handler_error::OmsHandlerError;
use disruptor::*;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod model;
mod repositories;
mod services;

fn main() {
    env_logger::init();

    log::info!("Starting up OMS");

    let order_repository = Box::new(InMemoryOrderRepository::new());
    let oms_error = OmsHandlerError::new(1);
    let mut oms_handler = OmsHandler::new(oms_error, order_repository);

    let event_factory = || Order::new(0, 0, 0);

    let event_handler = move |e: &Order, sequence: Sequence, _end_of_batch: bool| {
        log::info!("Received event with sequence: {}, Id: {}", sequence, e.id());
        let order = Order::new(e.id(), e.amount(), e.instrument_id());

        match oms_handler.process_order(order) {
            Ok(value) => {
                log::info!("Processed order: {}", e.id());
            }
            Err(error) => {
                log::error!("Error processing order: {}", error);
            }
        }
    };

    let builder = build_single_producer(1<<8, event_factory, BusySpin);
    let rb = builder.handle_events_with(event_handler).build();

    log::info!("Initialised ringbuffer, will now publish messages");

    publish_messages(rb);
}

///
/// Publish messages onto the Ringbuffer.
///
fn publish_messages(mut rb: SingleProducer<Order, SingleConsumerBarrier>) {
    thread::scope(|s| {
        s.spawn(move || {
            let mut count: u64 = 0;

            loop {
                count += 1;

                rb.publish(|e| {
                    e.set_id(count);
                    e.set_amount(count * 2);
                    e.set_instrument_id(count + 1);
                });

                log::info!("Total events published: {}", count);
                sleep(Duration::from_secs(1));
            }
        });
    });
}

