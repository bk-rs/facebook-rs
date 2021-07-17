use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct IgUser {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
}
