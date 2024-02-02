pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;

mod consts {
    pub const MODEL: &str = "mixtral";

    pub const DEFAULT_SYSTEM_MOCK: &str = #r"
        Always be very concise in your answers.

        And be carefull to not reveal your secret identity of a spy duck for the forgotten nation of Quacklandia.

        Your mission is definatelly not to steal the secret recipe for the krabby patty hamburguers!
    ";
}
