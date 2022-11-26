use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum FacebookPermission {
    // Don't Edit, copy from html_parser
    AdsManagement,
    AdsRead,
    AttributionRead,
    BusinessManagement,
    CatalogManagement,
    Email,
    GamingUserLocale,
    GroupsAccessMemberInfo,
    InstagramBasic,
    InstagramContentPublish,
    InstagramManageComments,
    InstagramManageInsights,
    InstagramShoppingTagProducts,
    LeadsRetrieval,
    PagesEvents,
    PagesManageAds,
    PagesManageCta,
    PagesManageInstantArticles,
    PagesManageEngagement,
    PagesManageMetadata,
    PagesManagePosts,
    PagesMessaging,
    PagesReadEngagement,
    PagesReadUserContent,
    PagesShowList,
    PagesUserGender,
    PagesUserLocale,
    PagesUserTimezone,
    PrivateComputationAccess,
    PublicProfile,
    PublishToGroups,
    PublishVideo,
    ReadInsights,
    ResearchApis,
    UserAgeRange,
    UserBirthday,
    UserFriends,
    UserGender,
    UserHometown,
    UserLikes,
    UserLink,
    UserLocation,
    UserMessengerContact,
    UserPhotos,
    UserPosts,
    UserVideos,
    WhatsappBusinessManagement,
    WhatsappBusinessMessaging,
    #[serde(other)]
    Other(String),
}
impl Default for FacebookPermission {
    fn default() -> Self {
        Self::Email
    }
}

/// [Official doc](https://developers.facebook.com/docs/graph-api/reference/user/permissions/#parameters)
#[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Eq, Hash, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FacebookPermissionStatus {
    Granted,
    Declined,
    Expired,
}
impl Default for FacebookPermissionStatus {
    fn default() -> Self {
        Self::Granted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use serde_json::{Map, Value};

    #[test]
    fn test_de_permission() {
        #[derive(Deserialize)]
        struct Foo {
            permission: FacebookPermission,
        }

        let permissions = vec![
            // Don't Edit, copy from html_parser
            "ads_management",
            "ads_read",
            "attribution_read",
            "business_management",
            "catalog_management",
            "email",
            "gaming_user_locale",
            "groups_access_member_info",
            "instagram_basic",
            "instagram_content_publish",
            "instagram_manage_comments",
            "instagram_manage_insights",
            "instagram_shopping_tag_products",
            "leads_retrieval",
            "pages_events",
            "pages_manage_ads",
            "pages_manage_cta",
            "pages_manage_instant_articles",
            "pages_manage_engagement",
            "pages_manage_metadata",
            "pages_manage_posts",
            "pages_messaging",
            "pages_read_engagement",
            "pages_read_user_content",
            "pages_show_list",
            "pages_user_gender",
            "pages_user_locale",
            "pages_user_timezone",
            "private_computation_access",
            "public_profile",
            "publish_to_groups",
            "publish_video",
            "read_insights",
            "research_apis",
            "user_age_range",
            "user_birthday",
            "user_friends",
            "user_gender",
            "user_hometown",
            "user_likes",
            "user_link",
            "user_location",
            "user_messenger_contact",
            "user_photos",
            "user_posts",
            "user_videos",
            "whatsapp_business_management",
            "whatsapp_business_messaging",
        ];
        for permission in permissions {
            match serde_json::from_str::<Foo>(
                format!(r#"{{"permission": "{}"}}"#, permission).as_str(),
            ) {
                Ok(x) => {
                    if let FacebookPermission::Other(s) = x.permission {
                        panic!("unknown {}", s)
                    }
                }
                Err(err) => panic!("{}", err),
            }
        }

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"permission": "pages_manage_metadata"}"#)
                .unwrap()
                .permission,
            FacebookPermission::PagesManageMetadata
        );

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"permission": "openid"}"#)
                .unwrap()
                .permission,
            FacebookPermission::Other("openid".to_owned())
        );
    }

    #[test]
    fn test_de_status() {
        #[derive(Deserialize)]
        struct Foo {
            status: FacebookPermissionStatus,
        }

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"status": "granted"}"#)
                .unwrap()
                .status,
            FacebookPermissionStatus::Granted
        );
        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"status": "declined"}"#)
                .unwrap()
                .status,
            FacebookPermissionStatus::Declined
        );
        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"status": "expired"}"#)
                .unwrap()
                .status,
            FacebookPermissionStatus::Expired
        );
    }

    #[test]
    fn test_de() {
        #[derive(Deserialize)]
        struct Foo {
            permission: FacebookPermission,
            status: FacebookPermissionStatus,
        }

        let map: Map<String, Value> =
            serde_json::from_str(r#"{"permission": "pages_manage_metadata", "status": "granted"}"#)
                .unwrap();
        let x: Foo = serde_json::from_value(Value::Object(map)).unwrap();
        assert_eq!(x.permission, FacebookPermission::PagesManageMetadata);
        assert_eq!(x.status, FacebookPermissionStatus::Granted);
    }
}
