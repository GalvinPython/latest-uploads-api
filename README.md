# Latest YouTube Uploads

Get the latest five uploads for any YouTube channel!

# How to use

It's very simple, just make a request to `https://latestvid.stats100.xyz/get/<CHANNEL_ID_HERE>`  
For example, to get the latest uploads for [MrBeast](https://youtube.com/@mrbeast), just have to make a simple request to `https://latestvid.stats100.xyz/get/UCX6OQ3DkcsbYNE6H8uQQuVA` and get an example response like this:

```json
[
  {
    "videoId": "KkCXLABwHP0",
    "title": "I Built 100 Homes And Gave Them Away!"
  },
  {
    "videoId": "PWirijQkH4M",
    "title": "World’s Deadliest Obstacle Course!"
  },
  {
    "videoId": "U_LlX4t0A9I",
    "title": "$10,000 Every Day You Survive In The Wilderness"
  },
  {
    "videoId": "T8I165Qxeo8",
    "title": "Sprinting with More and More Money"
  },
  {
    "videoId": "i-9V21MqlhY",
    "title": "Giving 1000 Phones Away"
  }
]
```

# Advanced Options

This API comes with extra queries for options, so you can get the response you want

## ?maxresults

The number of videos returned in the response

Technical overview:

- Required: **False**
- Type: `int`
- Default: `5`
- Minimum: `1`
  - Going below 1 will make `maxresults` **1**
- Maximum: `50`
  - Exceeding 50 will make `maxresults` **50**

Example response: `?maxresults=2`

```json
[
  {
    "title": "I Crashed One Of The Most Expensive Cars In The World",
    "videoId": "TgCm8oDmxOk"
  },
  {
    "title": "Last One To Fall Wins",
    "videoId": "954wJ3uHVAk"
  }
]
```

## ?type

The type of content you want to receive

Technical overview:

- Required: **False**
- Type: `string`
- Default: `all`
- Accepted values: `shorts`, `live`, `video`, `all`
  - Any other values will default to `all`

Example response: `?type=shorts`

```json
[
  {
    "title": "Gordon Ramsay Decides Feastables vs Hershey’s",
    "videoId": "g2nwnC1Xn0E"
  },
  {
    "title": "Can I Teach a Penguin To Subscribe?",
    "videoId": "KCRLP-zBacU"
  },
  {
    "title": "Extreme Helmet Game",
    "videoId": "c2ukL_zjFCk"
  },
  {
    "title": "Grab The Rolex, Keep It!",
    "videoId": "7qY-qalCI2Y"
  },
  {
    "title": "Slippery vs Sticky Stairs",
    "videoId": "N0pwLtonPdg"
  }
]
```

As a reminder, queries can be stacked: `?type=shorts&maxresults=2`

```json
[
  {
    "title": "Gordon Ramsay Decides Feastables vs Hershey’s",
    "videoId": "g2nwnC1Xn0E"
  },
  {
    "title": "Can I Teach a Penguin To Subscribe?",
    "videoId": "KCRLP-zBacU"
  }
]
```
