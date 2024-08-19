use bevy::prelude::*;
use serde::{Deserialize, Serialize};


pub struct LocalizationPlugin(String);

impl LocalizationPlugin {
    pub fn new(culture: String) -> Self {
        Self(culture)
    }
}

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Localization::new(self.0.clone()));
    }
}


/**
 * Key-value pair for localizations.
 */
#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct LocalePair {
    pub key: String,
    pub value: String,
}

/**
 * #### Description
 * Resource which contains all localizations.
 *
 * #### Remarks
 * This resource is automatically created by the plugin.
 * It can be accessed by using the `Localization` type.
 *
 * #### Fields
 * * `languages` - The list of localizations.
 * * `culture_full` - The full culture code of the current localization (e.g. en-US).
 * * `culture_top` - The top-level culture code of the current localization (e.g. en for en-US).
 *
 * #### Example
 * ```rust
 * fn setup(mut localization: ResMut<Localization>) {
 *      // Load in localizations
 *      localization.set("en-US", "hello", "Hello!");
 *      localization.set("en-US", "goodbye", "Goodbye!");
 *      localization.set("de-DE", "hello", "Hallo!");
 *      localization.set("de-DE", "goodbye", "Auf Wiedersehen!");
 * }
 *
 * fn usage(localization: Res<Localization>) {
 *      // Get localization with current culture
 *      println!("{}", localization.get("hello"));
 *      // Get localization with specified culture (returns Optional and is not using fallbacks)
 *      println!("{}", localization.get("de-DE", "goodbye"));
 * }
 * ```
 */
#[derive(Resource, Clone, Debug)]
pub struct Localization {
    /**
     * The list of localizations.
     */
    languages: Vec<(String, Vec<LocalePair>)>,
    /**
     * The full culture code of the current localization (e.g. en-US).
     */
    culture_full: String,
    /**
     * The top-level culture code of the current localization (e.g. en for en-US).
     */
    culture_top: String,
}

impl Localization {

    /**
     * #### Description
     * Creates a new localization.
     *
     * #### Parameters
     * * `culture` - The culture code of the localization (e.g. en-US).
     *
     * #### Returns
     * The new localization.
     */
    pub fn new(culture: String) -> Self {
        Self {
            languages: Vec::new(),
            culture_full: culture.clone(),
            culture_top: culture.split("-").collect::<Vec<&str>>()[0].to_string(),
        }
    }

    /**
     * Sets the current culture code.
     */
    pub fn set_culture(&mut self, culture: String) {
        self.culture_full = culture.clone();
        self.culture_top = culture.split("-").collect::<Vec<&str>>()[0].to_string();
    }

    /**
     * Gets the current culture code (e.g. en-US).
     */
    pub fn get_culture(&self) -> String {
        self.culture_full.clone()
    }

    /**
     * Gets the top-level culture code (e.g. en for en-US).
     */
    pub fn get_culture_top(&self) -> String {
        self.culture_top.clone()
    }

    /**
     * #### Description
     * Adds a new localization.
     *
     * #### Parameters
     * * `culture` - The culture code of the localization.
     * * `key` - The key of the localization.
     * * `value` - The value of the localization.
     */
    pub fn set(&mut self, culture: &String, key: String, value: String) {
        let mut found = false;
        for (l, pairs) in &mut self.languages {
            if l == culture {
                pairs.push(LocalePair {
                    key,
                    value,
                });
                found = true;
                return;
            }
        }
        if !found {
            self.languages.push((culture.clone(), vec![LocalePair {
                key,
                value,
            }]));
        }
    }

    /**
     * #### Description
     * Gets a localization in the current culture.
     *
     * #### Remarks
     * This method will fallback to the top-level culture and the default culture in that order.
     * If no localization was found, the key will be returned.
     * Due to this behavior, this method should be used for all localizations.
     *
     * #### Parameters
     * * `key` - The key of the localization.
     *
     * #### Returns
     * The value of the localization or the key if no localization was found.
     */
    pub fn get(&self, key: &String) -> String {
        let mut top_match: Option<String> = None;
        let mut default_match: Option<String> = None;
        for (l, pairs) in &self.languages {
            if *l == self.culture_full {
                for pair in pairs {
                    if pair.key == *key {
                        return pair.value.clone();
                    }
                }
            } else if *l == self.culture_top {
                for pair in pairs {
                    if pair.key == *key {
                        top_match = Some(pair.value.clone());
                    }
                }
            } else if default_match.is_none() {
                for pair in pairs {
                    if pair.key == *key {
                        default_match = Some(pair.value.clone());
                    }
                }
            }
        }

        if let Some(s) = top_match {
            return s;
        }

        if let Some(s) = default_match {
            return s;
        }

        key.clone()
    }

    /**
     * #### Description
     * Gets a localization in the selected culture.
     *
     * #### Remarks
     * This method will neither fallback to the top-level culture nor the default culture.
     *
     * #### Parameters
     * * `key` - The key of the localization.
     *
     * #### Returns
     * The value of the localization.
     */
    pub fn get_in(&self, culture: &String, key: &String) -> Option<String> {
        for (l, pairs) in &self.languages {
            if l == culture {
                for pair in pairs {
                    if pair.key == *key {
                        return Some(pair.value.clone());
                    }
                }
            }
        }

        None
    }
}