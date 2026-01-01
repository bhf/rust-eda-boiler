use std::ffi::CString;
use crate::model::order::Order;
use crate::repositories::order_repo::in_memory_order_repository::InMemoryOrderRepository;
use crate::services::oms::OMSService;
use crate::services::oms_service::oms_handler::OmsHandler;
use crate::services::oms_service::oms_handler_error::OmsHandlerError;
use disruptor::*;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use aeron_rs::aeron::Aeron;
use aeron_rs::context::Context;
use aeron_rs::utils::types::Index;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;


mod model;
mod repositories;
mod services;

fn main() {
    env_logger::init();

    let use_aeron = std::env::var("USE_AERON")
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(true);


    log::info!("Starting up Application");

    let order_repository = Box::new(InMemoryOrderRepository::new());
    let oms_error = OmsHandlerError::new(1);
    let oms_handler = OmsHandler::new(oms_error, order_repository);

    if use_aeron {
        subscribe_to_aeron("aeron:ipc".parse().unwrap(), 808);
    } else {
        publish_messages(oms_handler);
    }
}

///
/// Subscribe to an Aeron subscription and publish
/// messages to the Ringbuffer to be processed by the OMS Handler.
///
fn subscribe_to_aeron(subscription_channel: String, stream_id: i32) {
    let order_repository = Box::new(InMemoryOrderRepository::new());
    let oms_error = OmsHandlerError::new(1);
    let mut oms_handler = OmsHandler::new(oms_error, order_repository);

    let event_factory = || Order::new(0, 0, 0);

    let event_handler = move |e: &Order, sequence: Sequence, _end_of_batch: bool| {
        log::info!("Received event with sequence: {}, Id: {}", sequence, e.id());
        let order = Order::new(e.id(), e.amount(), e.instrument_id());

        match oms_handler.process_order(order) {
            Ok(_value) => {
                log::info!("Processed order: {}", e.id());
            }
            Err(error) => {
                log::error!("Error processing order: {}", error);
            }
        }
    };

    let builder = build_single_producer(1 << 8, event_factory, BusySpin);
    let mut rb = builder.handle_events_with(event_handler).build();

    log::info!("Subscribing to Aeron stream");

    // Create a context and connect to the Media Driver
    let context = Context::new();
    let mut aeron = Aeron::new(context).expect("Failed to create Aeron instance");

    // Add a subscription
    // We clone the channel because add_subscription consumes the CString
    let subscription_id = aeron
        .add_subscription(CString::new(subscription_channel.clone()).unwrap(), stream_id)
        .expect("Failed to add subscription");

    log::info!("Subscription added to channel: {:?} stream: {}", subscription_channel, stream_id);

    // Find the subscription object
    let subscription = loop {
        if let Ok(sub) = aeron.find_subscription(subscription_id) {
            break sub;
        }
        sleep(Duration::from_millis(10));
    };

    // Handler for processing received fragments
    // Explicitly annotate types so the compiler can find .get_bytes() and .session_id()
    let mut fragment_handler = |buffer: &AtomicBuffer, offset: Index, _length: Index, header: &Header| {

        rb.publish(|e| {
            let _count = buffer.get::<u64>(offset);
            let id = buffer.get::<u64>(offset + 8);
            let amount = buffer.get::<u64>(offset + 16);
            let instrument_id = buffer.get::<u64>(offset + 24);
            log::debug!("Decoded data on session {}, ID={}, Amount={}, Instrument={}", header.session_id(), id, amount, instrument_id);
            e.populate(id, amount, instrument_id);
        });
    };

    // Poll loop
    let running = Arc::new(AtomicBool::new(true));

    while running.load(Ordering::SeqCst) {
        let fragments_read = subscription.lock().unwrap().poll(&mut fragment_handler, 10);
        if fragments_read == 0 {
            sleep(Duration::from_millis(1));
        }
    }

}

///
/// Publish messages onto the Ringbuffer.
///
fn publish_messages(mut oms_handler: OmsHandler) {

    let event_factory = || Order::new(0, 0, 0);

    let event_handler = move |e: &Order, sequence: Sequence, _end_of_batch: bool| {
        log::info!("Received event with sequence: {}, Id: {}", sequence, e.id());
        let order = Order::new(e.id(), e.amount(), e.instrument_id());

        match oms_handler.process_order(order) {
            Ok(_value) => {
                log::info!("Processed order: {}", e.id());
            }
            Err(error) => {
                log::error!("Error processing order: {}", error);
            }
        }
    };

    let builder = build_single_producer(1 << 8, event_factory, BusySpin);
    let mut rb = builder.handle_events_with(event_handler).build();

    thread::scope(|s| {
        s.spawn(move || {
            let mut count: u64 = 0;

            loop {
                count += 1;

                rb.publish(|e| {
                    let id = count;
                    let amount = count * 2;
                    let instrument_id = if id % 2 == 0 { 0 } else { id + 1 };
                    e.populate(id, amount, instrument_id);
                });

                log::info!("Total events published to ringbuffer: {}", count);
                sleep(Duration::from_secs(1));
            }
        });
    });
}
