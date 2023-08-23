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
Granny: Promise me that you'll take care of yourself, okay? #smiling
===
```

A [dialog view](./dialog_views.md) will be able to read the metadata "smiling", "laughing", and "smiling" again from `LocalizedLine::metadata` and accordingly load things like character portraits.
These annotations will also be written into the "comment" field of strings files, which are explained in the chapter [Localization](./localization.md).

## Asset Providers

Assets are fetched from the filesystem by structs implementing `AssetProvider`. They need to be registered when creating a `DialogRunner`.
For example, if you use the `audio_assets` feature, you can register an asset provider for audio files by modifying the code found in the [setup](./setup.md) like this:

```rust
fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .add_asset_provider(AudioAssetProvider::new())
        .build();
    dialogue_runner.start_node("Start");
    commands.spawn(dialogue_runner);
}

```

TODO: Code does not work
TODO: Mention default dialog view does not support this
TODO: Where stuff is searched
TODO: FileExtensionAssetProvider
TODO: Own AssetProvider?
