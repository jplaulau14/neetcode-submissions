import heapq


class Twitter:
    def __init__(self):
        self.time = 0
        self.tweets = {}
        self.following = {}

    def postTweet(self, userId: int, tweetId: int) -> None:
        self.tweets.setdefault(userId, []).append((self.time, tweetId))
        self.time += 1

    def getNewsFeed(self, userId: int):
        heap = []
        users = self.following.get(userId, set()) | {userId}

        for uid in users:
            history = self.tweets.get(uid)
            if history:
                index = len(history) - 1
                timestamp, tweetId = history[index]
                heapq.heappush(heap, (-timestamp, tweetId, uid, index))

        result = []
        while heap and len(result) < 10:
            negTimestamp, tweetId, uid, index = heapq.heappop(heap)
            result.append(tweetId)
            if index > 0:
                nextIndex = index - 1
                timestamp, nextTweetId = self.tweets[uid][nextIndex]
                heapq.heappush(heap, (-timestamp, nextTweetId, uid, nextIndex))

        return result

    def follow(self, followerId: int, followeeId: int) -> None:
        if followerId != followeeId:
            self.following.setdefault(followerId, set()).add(followeeId)

    def unfollow(self, followerId: int, followeeId: int) -> None:
        if followerId in self.following:
            self.following[followerId].discard(followeeId)