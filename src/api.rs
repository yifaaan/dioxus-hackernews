use crate::{Comment, StoryData, StoryItem};
use anyhow::Result;
use futures::future::join_all;

const MAX_STORIES: usize = 50;

pub async fn get_top_stories(n: usize) -> Result<Vec<StoryItem>> {
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let ids: Vec<i64> = reqwest::get(url).await?.json().await?;
    let story_futures = ids.into_iter().take(n).map(|id| get_story_item_by_id(id));

    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|item| item.ok())
        .collect::<Vec<StoryItem>>();
    Ok(stories)
}

pub async fn get_story_item_by_id(id: i64) -> Result<StoryItem> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let item = reqwest::get(url).await?.json().await?;
    Ok(item)
}

pub async fn get_story_comments(item: StoryItem) -> Result<StoryData> {
    let comment_futures = item.kids.iter().map(|id| get_comment_by_id(*id));
    let comments = join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|comment| comment.ok())
        .collect::<Vec<Comment>>();
    Ok(StoryData { item, comments })
}

pub async fn get_comment_by_id(id: i64) -> Result<Comment> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let comment: Comment = reqwest::get(&url).await?.json().await?;
    Ok(comment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_top_stories_should_work() {
        let stores = get_top_stories(3).await.unwrap();
        assert_eq!(stores.len(), 3);
    }

    #[tokio::test]
    async fn get_comment_by_id_should_work() {
        let story = get_top_stories(1).await.unwrap().pop().unwrap();
        let id = story.kids[0];
        let comment = get_comment_by_id(id).await.unwrap();
        assert_eq!(comment.id, id);
    }
}
