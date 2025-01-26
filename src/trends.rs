use crate::api::client::TwitterClient;
use crate::api::requests::request_api;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendsResponse {
    pub data: TrendsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendsData {
    pub timeline: Timeline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub timeline: TimelineData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineData {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Instruction {
    TimelineAddEntries { entries: Vec<Entry> },
    TimelineClearCache,
    TimelineTerminateTimeline { direction: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub content: Option<EntryContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryContent {
    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemContent {
    #[serde(rename = "itemType")]
    pub item_type: String,
    pub name: Option<String>,
    pub trend_metadata: Option<TrendMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendMetadata {
    pub domain_context: Option<String>,
    pub meta_description: Option<String>,
}

pub async fn get_trends(
    client: &TwitterClient,
    timeline_id: &str,
    count: i16,
) -> Result<Vec<String>> {
    let variables = json!({
        "timelineId": timeline_id,
        "count": count,
        "cursor": "DAAJAAA",
        "withQuickPromoteEligibilityTweetFields": true
    });

    let features = json!({
        "profile_label_improvements_pcf_label_in_post_enabled": true,
        "rweb_tipjar_consumption_enabled": true,
        "responsive_web_graphql_exclude_directive_enabled": true,
        "verified_phone_label_enabled": false,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "responsive_web_graphql_timeline_navigation_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "premium_content_api_read_enabled": false,
        "communities_web_enable_tweet_community_results_fetch": true,
        "c9s_tweet_anatomy_moderator_badge_enabled": true,
        "responsive_web_grok_analyze_button_fetch_trends_enabled": false,
        "responsive_web_grok_analyze_post_followups_enabled": true,
        "responsive_web_jetfuel_frame": false,
        "responsive_web_grok_share_attachment_enabled": true,
        "articles_preview_enabled": true,
        "responsive_web_edit_tweet_api_enabled": true,
        "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
        "view_counts_everywhere_api_enabled": true,
        "longform_notetweets_consumption_enabled": true,
        "responsive_web_twitter_article_tweet_consumption_enabled": true,
        "tweet_awards_web_tipping_enabled": false,
        "creator_subscriptions_quote_tweet_preview_enabled": false,
        "freedom_of_speech_not_reach_fetch_enabled": true,
        "standardized_nudges_misinfo": true,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
        "rweb_video_timestamps_enabled": true,
        "longform_notetweets_rich_text_read_enabled": true,
        "longform_notetweets_inline_media_enabled": true,
        "responsive_web_grok_image_annotation_enabled": true,
        "responsive_web_enhance_cards_enabled": false
    });

    let mut headers = reqwest::header::HeaderMap::new();
    client.auth.install_headers(&mut headers).await?;

    let (response, _) = request_api::<TrendsResponse>(
        &client.client,
        "https://twitter.com/i/api/graphql/-R9ACaB96xqEnX2BJ_RbFA/GenericTimelineById",
        headers,
        Method::GET,
        Some(json!({
            "variables": variables,
            "features": features
        })),
    )
    .await?;

    let mut trends = Vec::new();

    // Find the TimelineAddEntries instruction
    for instruction in response.data.timeline.timeline.instructions {
        if let Instruction::TimelineAddEntries { entries } = instruction {
            for entry in entries {
                if let Some(content) = entry.content {
                    if let Some(item_content) = content.item_content {
                        if item_content.item_type == "TimelineTrend" {
                            if let Some(name) = item_content.name {
                                trends.push(name);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(trends)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::get_session;

    #[tokio::test]
    async fn test_trends() {
        let client = get_session().await.unwrap();

        let timelines = client.get_explore_timelines().await.unwrap();
        println!("{timelines:?}");
        let trends = get_trends(&client.twitter_client, &timelines[0].id, 20)
            .await
            .unwrap();
        println!("Trends: {:?}", trends);
        assert!(!trends.is_empty(), "Expected trends");
    }
}
