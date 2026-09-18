use std::collections::BinaryHeap;

struct Twitter {
    time: i32,
    tweets: Vec<Vec<(i32, i32)>>,
    following: Vec<Vec<bool>>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            time: 0,
            tweets: vec![Vec::new(); 501],
            following: vec![vec![false; 501]; 501],
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets[user_id as usize].push((self.time, tweet_id));
        self.time += 1;
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<(i32, i32, usize, usize)> = BinaryHeap::new();
        let mut sources: Vec<usize> = self.following[user_id as usize]
            .iter()
            .enumerate()
            .filter(|&(_, &followed)| followed)
            .map(|(id, _)| id)
            .collect();
        sources.push(user_id as usize);

        for uid in sources {
            if let Some(&(timestamp, tweet_id)) = self.tweets[uid].last() {
                let index = self.tweets[uid].len() - 1;
                heap.push((timestamp, tweet_id, uid, index));
            }
        }

        let mut result = Vec::with_capacity(10);
        while result.len() < 10 {
            let Some((_, tweet_id, uid, index)) = heap.pop() else {
                break;
            };
            result.push(tweet_id);
            if index > 0 {
                let next_index = index - 1;
                let (timestamp, next_tweet_id) = self.tweets[uid][next_index];
                heap.push((timestamp, next_tweet_id, uid, next_index));
            }
        }
        result
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            self.following[follower_id as usize][followee_id as usize] = true;
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.following[follower_id as usize][followee_id as usize] = false;
    }
}