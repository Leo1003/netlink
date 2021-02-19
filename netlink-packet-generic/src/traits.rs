pub trait GenlFamily {
    /// The custom header type
    type Header;

    /// The payload type contains the commands and the attributes
    type Payload: GenericPayload;

    /// Return the unique family name
    ///
    /// Used to lookup the dynamically assigned ID
    fn family_name(&self) -> &'static str;

    /// Return the assigned family ID
    ///
    /// # Note
    /// The implementation of generic family should assign the ID to `GENL_ID_GENERATE` (0x0).
    /// So the controller can dynamically assign the family ID.
    ///
    /// Regarding to the reason above, you should not have to implement the function
    /// unless the family uses the static ID.
    fn family_id(&self) -> u16 {
        0
    }

    /// Indicate the protocol version
    fn version(&self) -> u8;
}

pub trait GenericPayload {
    fn command(&self) -> u8;
}
