//! A translation tool that retrieves the messages stored in yaml files.

use log::warn;
use std::collections::HashMap;
mod messages;

#[derive(Debug)]
/// The object that keeps the messages collection.
///
/// This is the yaml file to be used in the following tests:
/// ```yaml
/// ## tests/en/messages.yaml
///
/// hello:
///   world: Hello World!
///   somebody: Hello {user}!
/// ```
pub struct Messages {
    pub collection: HashMap<String, HashMap<String, String>>,
}
impl Messages {
    /// Creates and returns the `Messages` instance given the messages root folder path.
    /// # Examples:
    /// ```
    /// use kalgan_i18n::Messages;
    ///
    /// let messages: Messages = Messages::new("tests");
    /// ```
    pub fn new(source: &str) -> Messages {
        messages::generate(source)
    }
    /// Returns the translated message for the given parameters.
    /// # Examples:
    /// ```
    /// # use std::collections::HashMap;
    /// use kalgan_i18n::Messages;
    ///
    /// # let messages: Messages = Messages::new("tests");
    /// assert_eq!(messages.trans("en", "hello.world", HashMap::new()), "Hello World!");
    /// ```
    /// ```
    /// # use std::collections::HashMap;
    /// use kalgan_i18n::Messages;
    ///
    /// # let messages: Messages = Messages::new("tests");
    /// let mut parameters = HashMap::new();
    /// parameters.insert("user", "John".to_string());
    /// assert_eq!(messages.trans("en", "hello.somebody", parameters), "Hello John!");
    /// ```
    pub fn trans(
        &self,
        language: &str,
        message_id: &str,
        parameters: HashMap<&str, String>,
    ) -> String {
        if self.collection.contains_key(language) {
            if self.collection[language].contains_key(message_id) {
                let mut message = self.collection[language][message_id].clone();
                for (key, value) in parameters {
                    if message.contains(&key.to_string()) {
                        message = message.replace(&format!("{{{}}}", key.to_string()), &value);
                    } else {
                        warn!("Parameter \"{}\" not found in \"{}\".", &key, &message_id);
                    }
                }
                message
            } else {
                warn!("Message \"{}\" not found.", &message_id);
                message_id.to_string()
            }
        } else {
            warn!(
                "Language \"{}\" not found for message \"{}\".",
                &language, &message_id
            );
            message_id.to_string()
        }
    }
}
