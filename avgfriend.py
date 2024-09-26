friends = {
    "Frank": ["David", "Kim"],
    "Jasmine": ["Kim", "Jose"],
    "Kim": ["Frank", "Jasmine", "David", "Felix", "Jose", "Adin", "Jean"],
    "David": ["Frank", "Kim", "Sven"],
    "Sven": ["David", "Adin"],
    "Felix": ["Kim", "Ivan"],
    "Jean": ["Ivan", "Kim"],
    "Ivan": ["Adin", "Felix", "Jean"],
    "Adin": ["Ivan", "David", "Maria", "Jose", "Kim", "Sven", "Phil"],
    "Maria": ["Adin", "Jose"],
    "Jose": ["Maria", "Adin", "Kim", "Jasmine"],
    "Phil": ["Adin"]
}

def friendCount(name):
    return len(friends[name])

from statistics import mean
def avgFriendFriendCount(name):
    return mean(friendCount(f) for f in friends[name])

print("|Name | Friends | Average friend's friend count|")
print("|-----|---------|------------------------------|")

for name in friends:
    x = friendCount(name)
    y = avgFriendFriendCount(name)
    print(f'| {name} | {x} | {round(y, 2)} |')
