//! Implementation utilities for CoAP-over-GATT ([draft-amsuess-core-coap-over-gatt-02])
//!
//! Right now, this contains the message format parsing and serialization, largely fulled by
//! [coap_message_utils], given that most of the message format is just a rehash of the universal
//! CoAP option-extension-data-ff-payload scheme.
//!
//! [draft-amsuess-core-coap-over-gatt-02]: https://datatracker.ietf.org/doc/id/draft-amsuess-core-coap-over-gatt-02.html
#![no_std]

pub type ReadMessage<'a> = coap_message_utils::inmemory::Message<'a>;
pub type WriteMessage<'a> = coap_message_utils::inmemory_write::Message<'a>;
pub type ReadWriteMessage<'a> = coap_message_utils::inmemory_write::Message<'a>;

/// Error type for trying to parse a zero-length message
#[derive(Debug)]
pub struct MessageTooShort;

/// Return a readable message version of data received over CoAP-over-GATT.
///
/// Note that, as explained in [coap_message_utils::inmemory::Message], a successful
/// result does not mean that the message is valid as a whole -- that'd entail "expensive" parsing
/// of the message ahead of time. Instead, if invalid properties are discovered at processing time,
/// the message will pretend to contain an option that the application does not understand.
// FIXME returning the concrete type is convenient as it allows a few types to be known, but may be
// limiting on the long run
pub fn parse(serialized: &[u8]) -> Result<ReadMessage<'_>, MessageTooShort> {
    Ok(coap_message_utils::inmemory::Message::new(
        *serialized.get(0).ok_or(MessageTooShort)?,
        &serialized[1..],
    ))
}

/// Like parse, but take data from a mutable slice, and keep the message writable
///
/// While this is generally not useful (a parsed message has its paylod already set, and if there's
/// no payload, there's no space to add options or any payload), it allows mutable access to the
/// option bytes and the payload. This is primarily useful in situations when data is processed in
/// place, eg. decrypted (in OSCORE), or CBOR is shifted around to get contiguous slices out of
/// indefinite length strings.
///
/// Note that as this returns the WriteMessage, unlike in [write] where access happens through a
/// closure, the final length of the buffer is inaccessible (but generally not needed -- after
/// whoever uses this is done with the message, no illusions of any validity of the message in the
/// buffer should be had any more).
pub fn parse_mut(serialized: &mut [u8]) -> Result<ReadWriteMessage<'_>, MessageTooShort> {
    if serialized.len() < 1 {
        return Err(MessageTooShort);
    }
    let (mut code, mut tail) = serialized.split_at_mut(1);
    Ok(coap_message_utils::inmemory_write::Message::new_from_existing(&mut code[0], tail))
}

/// Dress up a writable buffer of a constant size as a writable CoAP message
///
/// ## Open issues
///
/// Can we do any better than sending the user through a closure? It's not like we have to do any
/// actual cleanup, we just^TM need to create the inmemory_write::Message with a reference, and be
/// there when it is destructed.
///
/// In an even advanced version, this might take an in-place writer to place the bytes into, rather
/// than moving a [heapless::Vec] around (and hoping for return value optimization).
// FIXME as for parse()
pub fn write<const N: usize>(closure: impl FnOnce(&mut WriteMessage<'_>)) -> heapless::Vec<u8, N> {
    let mut buffer = heapless::Vec::<u8, N>::new();
    // FIXME can we get rid of these needless writes? Probably inmemory_write would need a
    // better backend than just a slice
    buffer.resize_default(N).unwrap();

    let (codebuf, tailbuf) = buffer.split_at_mut(1);

    let mut message = coap_message_utils::inmemory_write::Message::new(&mut codebuf[0], tailbuf);
    closure(&mut message);
    let len = message.finish();
    buffer.truncate(len + 1);
    buffer
}
