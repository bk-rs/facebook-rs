use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Eq, Hash, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FacebookPermission {
    // Don't Edit, copy from html_parser
    AdsManagement,
    AdsRead,
    AttributionRead,
    BusinessManagement,
    CatalogManagement,
    Email,
    GroupsAccessMemberInfo,
    InstagramBasic,
    InstagramContentPublish,
    InstagramManageComments,
    InstagramManageInsights,
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
    PublicProfile,
    PublishToGroups,
    PublishVideo,
    ReadInsights,
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
}

/// [Official doc](https://developers.facebook.com/docs/graph-api/reference/user/permissions/#parameters)
#[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Eq, Hash, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FacebookPermissionStatus {
    Granted,
    Declined,
    Expired,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;

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
            "groups_access_member_info",
            "instagram_basic",
            "instagram_content_publish",
            "instagram_manage_comments",
            "instagram_manage_insights",
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
            "public_profile",
            "publish_to_groups",
            "publish_video",
            "read_insights",
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
        ];
        for permission in permissions {
            match serde_json::from_str::<Foo>(
                format!(r#"{{"permission": "{}"}}"#, permission).as_str(),
            ) {
                Ok(_) => {}
                Err(err) => panic!("{}", err),
            }
        }

        assert_eq!(
            serde_json::from_str::<Foo>(r#"{"permission": "pages_manage_metadata"}"#)
                .unwrap()
                .permission,
            FacebookPermission::PagesManageMetadata
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
}
