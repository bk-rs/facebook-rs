//! [Official doc](https://developers.facebook.com/docs/graph-api/webhooks/reference/instagram)
//!
//! Require [Enable Page Subscriptions](https://developers.facebook.com/docs/instagram-api/guides/webhooks#step-2--enable-page-subscriptions)

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "field", content = "value", rename_all = "snake_case")]
pub enum Instagram {
    Comments(CommentsValue),
    Mentions(MentionsValue),
    StoryInsights(StoryInsightsValue),
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommentsValue {
    /// id == [IG Comment id](https://developers.facebook.com/docs/instagram-api/reference/ig-comment)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub text: String,
    /// Click "Test" in facebook webhooks configure page, it's always None.
    /// Bug in [doc page](https://developers.facebook.com/docs/graph-api/webhooks/reference/instagram/v11.0#fields) , it's not None.
    pub media: Option<Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MentionsValue {
    /// media_id == [IG Media id](https://developers.facebook.com/docs/instagram-api/reference/ig-media)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub media_id: u64,
    /// comment_id == [IG Comment id](https://developers.facebook.com/docs/instagram-api/reference/ig-comment)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub comment_id: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StoryInsightsValue {
    /// media_id == [IG Media id](https://developers.facebook.com/docs/instagram-api/reference/ig-media)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub media_id: u64,
    /// metrics == ["Story Metrics"](https://developers.facebook.com/docs/instagram-api/reference/ig-media/insights)
    #[serde(flatten)]
    pub metrics: StoryInsightsMetrics,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StoryInsightsMetrics {
    pub impressions: isize,
    pub reach: isize,
    pub taps_forward: isize,
    pub taps_back: isize,
    pub exits: isize,
    pub replies: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = r#"
        {
            "field": "story_insights",
            "value": {
                "media_id": "17887498072083520",
                "impressions": 444,
                "reach": 44,
                "taps_forward": 4,
                "taps_back": 3,
                "exits": 3,
                "replies": 0
            }
        }
        "#;
        match serde_json::from_str::<Instagram>(json) {
            Ok(Instagram::StoryInsights(v)) => {
                println!("{:?}", v);

                assert_eq!(v.media_id, 17887498072083520);
                assert_eq!(v.metrics.impressions, 444);
            }
            Ok(v) => panic!("{:?}", v),
            Err(err) => panic!("{}", err),
        }

        let json = r#"
        {
            "field": "comments",
            "value": {
                "id": "17865799348089039",
                "text": "This is an example."
            }
        }
        "#;
        match serde_json::from_str::<Instagram>(json) {
            Ok(Instagram::Comments(v)) => {
                println!("{:?}", v);

                assert_eq!(v.id, 17865799348089039);
            }
            Ok(v) => panic!("{:?}", v),
            Err(err) => panic!("{}", err),
        }
    }
}
