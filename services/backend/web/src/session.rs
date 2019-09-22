use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Session {
    pub user_id: Uuid,
    pub expires: DateTime<Utc>,
}

impl Session {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Unable to convert session to json")
    }
    pub fn parse(json: &str) -> Session {
        serde_json::from_str(json).expect("Unable to parse session from json")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    #[test]
    fn serialize_to_json() {
        let session = Session {
            user_id: Uuid::from_str("b044210a-2186-4369-bf55-c4310c342fce").unwrap(),
            expires: DateTime::parse_from_rfc3339("2019-09-20T15:23:47.435840Z").unwrap().with_timezone(&Utc),
        };

        let json = session.to_json();

        assert_eq!(r#"{"user_id":"b044210a-2186-4369-bf55-c4310c342fce","expires":"2019-09-20T15:23:47.435840Z"}"#, json);
    }

    #[test]
    fn deserialize_from_json() {
        let json = r#"{"user_id":"b044210a-2186-4369-bf55-c4310c342fce","expires":"2019-09-20T15:23:47.435840Z"}"#;

        let session = Session::parse(json);

        assert_eq!(session, Session {
            user_id: Uuid::from_str("b044210a-2186-4369-bf55-c4310c342fce").unwrap(),
            expires: DateTime::parse_from_rfc3339("2019-09-20T15:23:47.435840Z").unwrap().with_timezone(&Utc),
        });
    }
}
