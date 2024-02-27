# HowTo: Add new models

Adding a new model is rather simple, you just need a few things:

- A valid .gltf file paired with a .bin file (use eg. [Blender](https://www.blender.org/)) to create one)
- A file called `config.toml`. See below for an example.

```toml
# !REQUIRED!
# This text is how your model is named in-game. Spawning it will work via this very name.
# This must be unique!
# For Modders, please prefix your identifier with "steamid64/YOUR_STEAM_ID_64/" for maximum compatibility.
# To get your steamid64 you can use eg. https://steamid.io
# If you don't have a steamid64, use another unique prefix combination (eg. "e-mail/YOUR_EMAIL_ADDRESS/").
# Please do not deviate from this format, as this may lead to conflicts in the future.
# A different solution may be implemented in the future, which requires less details.
identifier = "steamid64/00000000000000000/spaceship"

# Handles how the model is displayed in ui. Note that this is localized and hence the strings
# refer to the [[localization]] sections and are not free-form text.
[display]
title = "model_name"
description = "model_description"

# Localizations for the model. Each [[localization]] section must have a "culture" key!
[[localization]]
culture = "en" # !REQUIRED! The culture this localization is for. The format follows the https://www.rfc-editor.org/info/bcp47 RFC.
string_a = "this is a localized string for the identifier 'string_a'"
model_name = "Spaceship"
model_description = "A very special spaceship that was the beginning of everything."

# You can add more localizations by adding more [[localization]] sections.
[[localization]]
culture = "de" # !REQUIRED!
string_a = "Dies ist ein Übersetzter Text für den Identifikator 'string_a'"
model_name = "Raumschiff"
model_description = "Ein sehr besonderes Raumschiff, welches am Anfang von allem stand."
```

Now you can add your model to the `assets` folder and it will be loaded automatically 🎉.

Note: This will only make your model available in the game.
It still has to be spawned via some other means (eg. the console).

# Interactivity using scripts
The game uses the scripting language [Rhai](https://rhai.rs/) to allow for advanced interactivity.
For this, every file in the `scripts` folder is loaded.
There are numerous event functions that can be implemented to react to various events.
***For more details, including documentation, check out the scripts folder in this repository.***