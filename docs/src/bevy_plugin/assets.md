# Assets

While Bevy as a whole has assets, Yarn Slinger can associate specific assets with lines.
These are always [localized](./localization.md), such as voiceovers. 

## Using Metadata Instead of Assets

Before we jump into assets, let's first help you out if you don't care about localization.
The mechanism in place for this is *line metadata*, which are strings you can add to Yarn lines after a hashtag:

```text
title: Start
---
Granny: It's hard to believe that it's over, isn't it? #smiling
Granny: Funny how we get attached to the struggle. #laughing
Granny: Promise me tat you'll take care of yourself, okay? #smiling
===
```

A [dialog view](./dialog_views.md) will be able to read the metadata "smiling", "laughing", and "smiling" again from `LocalizedLine::metadata` and accordingly load things like character portraits.
These annotations will also be written into the "comment" field of strings files, which are explained in the chapter [Localization](./localization.md).

## Asset Providers

TODO

