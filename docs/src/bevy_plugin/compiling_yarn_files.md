# Compiling Yarn Files

The `YarnProject` resource represents the set of all compiled Yarn files of 
your game. You cannot construct it yourself. Instead, it is inserted into the Bevy world for
you when the compilation is finished. You can however steer how and when this is done.

## Starting the Compilation Process

Generally, you'll want your game to compile the Yarn files as soon as possible. This
is why the [`YarnSpinnerPlugin`](setup.md) will start doing so by default when it is added to the app.

If for some reason you do not wish to start compilation right away, you can *defer* this process. To do this,
construct the `YarnSpinnerPlugin` with `YarnSpinnerPlugin::deferred()` when adding it. Then, whenever you are ready
to start the compilation, you can send a `LoadYarnProjectEvent`. Its construction methods are identical to the `YarnSpinnerPlugin`.
In fact, when not running in deferred mode, the `YarnSpinnerPlugin` simply relays its setting to a `LoadYarnProjectEvent` and sends it.

## Settings

If you look through the documentation of the [`YarnSpinnerPlugin`], you'll notice a few methods to modify
its settings. The first few deal with where our Yarn files are coming from. 

### Yarn File Sources

By default, Yarn Spinner will look
in `<game directory>/assets/dialog`. Yarn Spinner can only read files from the `assets` directory 
— or its equivalent, if you have changed this default in the `AssetPlugin` on platforms which support it—
but you can change how the `assets` will be looked through.

The way to specify this is via `YarnFileSource`s. This enum tells Yarn Spinner where one or more Yarn files
come from and can be added to an `AssetPlugin` with `AssetPlugin::add_yarn_source()`.
The enum variants should be self explanatory, but the two most common use-cases come with their own convenience constructors:
- `YarnFileSource::file()`: looks for a Yarn file at a path inside under the `assets` directory.
- `YarnFileSource::folder()`: recursively looks through a given subdirectory for Yarn files.

Since the Wasm and Android builds of Bevy have restrictions on their filesystem access,
they cannot use `YarnFileSource::folder()` and must have all their Yarn files listed explicitly with `YarnFileSource::file()`.
As such, the default behavior provided by `YarnSlingerPlugin::new()` is not suitable for these platforms.
To avoid it, use the `AssetPlugin::with_yarn_source()` constructor instead.

As you might have guessed by now, `YarnSlingerPlugin::new()` is simply a shorthand for `AssetPlugin::with_yarn_source(YarnFileSource::folder("dialog"))`.

### Development File Generation

`YarnSlingerPlugin::with_development_file_generation()` accepts a `DevelopmentFileGeneration`, which tells Yarn Spinner how aggressively to generate useful files on runtime.
"Useful" refers to the developer and not the user. The default is `DevelopmentFileGeneration::TRY_FULL`, which will be `DevelopmentFileGeneration::Full` on platforms which support filesystem access, 
i.e. all except Wasm and Android. See the documentation for the full list of effects. Suffice it to say
that this is not very important when developing without localization, but becomes vital otherwise. See the [Localization](localization.md) chapter for more.

Since these settings are intended for development, you can use `YarnSlingerPlugin::with_development_file_generation(DevelopmentFileGeneration::None)` when shipping your game to optimize the runtime costs and
avoid generating files that are useless to the player.

### Localization

The settings accessed by `YarnSlingerPlugin::with_localizatons` are important enough to warrant their own chapter. See [Localization](localization.md).

## After the Compilation

Whether you used `YarnSlingerPlugin` or `LoadYarnProjectEvent`, as soon as the compilation finished, a `YarnProject` resource will be inserted into the Bevy world. 
You can react to its creation by guarding your systems with `.run_if(resource_added::<YarnProject>())`, as seen in the [setup](setup.md).

Once you have the `YarnProject`, you can use it to spawn a `DialogRunner` which in turn can, well, [run dialogs](dialog_runner.md)
