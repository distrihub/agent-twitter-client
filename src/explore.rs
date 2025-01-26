use crate::api::client::TwitterClient;
use crate::api::requests::request_api;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploreResponse {
    pub data: ExploreData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploreData {
    pub explore_page: ExplorePage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorePage {
    pub id: String,
    pub body: ExploreBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploreBody {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub timelines: Vec<TimelineSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineSection {
    pub id: String,
    #[serde(rename = "labelText")]
    pub label_text: String,
    pub timeline: TimelineId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineId {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct ExploreTimeline {
    pub id: String,
    pub name: String,
}

pub async fn get_explore_timelines(client: &TwitterClient) -> Result<Vec<ExploreTimeline>> {
    let variables = json!({"cursor":""});

    let features = json!({"profile_label_improvements_pcf_label_in_post_enabled":true,"rweb_tipjar_consumption_enabled":true,"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"creator_subscriptions_tweet_preview_api_enabled":true,"premium_content_api_read_enabled":false,"communities_web_enable_tweet_community_results_fetch":true,"c9s_tweet_anatomy_moderator_badge_enabled":true,"responsive_web_grok_analyze_button_fetch_trends_enabled":false,"responsive_web_grok_analyze_post_followups_enabled":true,"responsive_web_jetfuel_frame":false,"responsive_web_grok_share_attachment_enabled":true,"articles_preview_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":true,"tweet_awards_web_tipping_enabled":false,"creator_subscriptions_quote_tweet_preview_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"rweb_video_timestamps_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_grok_image_annotation_enabled":true,"responsive_web_enhance_cards_enabled":false});

    let mut headers = reqwest::header::HeaderMap::new();
    client.auth.install_headers(&mut headers).await?;

    let (response, _) = request_api::<ExploreResponse>(
        &client.client,
        "https://twitter.com/i/api/graphql/_XV-G8GPq40yR0j1h86YZg/ExplorePage",
        headers,
        Method::GET,
        Some(json!({
            "variables": variables,
            "features": features
        })),
    )
    .await?;

    let timelines = response
        .data
        .explore_page
        .body
        .timelines
        .into_iter()
        .map(|t| ExploreTimeline {
            id: t.timeline.id,
            name: t.label_text,
        })
        .collect();

    Ok(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::get_session;

    #[tokio::test]
    async fn test_explore_timelines() {
        let client = get_session().await.unwrap();
        let timelines = get_explore_timelines(&client.twitter_client).await.unwrap();
        println!("Timelines: {:?}", timelines);
        assert!(!timelines.is_empty(), "Expected timelines");
    }
}
