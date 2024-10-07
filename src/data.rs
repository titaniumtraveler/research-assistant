use crate::data::serde_impl::{bool_true, parse_bool, recipes_aspects, u32_100};
use serde::{Deserialize, Serialize};
use serde_impl::StringOrArray;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

pub use serde_impl::StringOrStruct;

mod serde_impl;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Data {
    achievements: Achievements,
    cultures: Cultures,
    decks: Decks,
    dicta: Dicta,
    elements: Elements,
    endings: Endings,
    legacies: Legacies,
    recipes: Recipes,
    // settings
    // verbs
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Achievements {
    category: String,
    #[serde(rename = "descriptionunlocked")]
    description_unlocked: String,
    // TODO: convert `""` to `Option::None`
    #[serde(rename = "iconUnlocked")]
    icon_unlocked: String,
    id: String,
    #[serde(rename = "isCategory")]
    #[serde(default)]
    is_category: bool,
    label: String,
    #[serde(rename = "singleDescription")]
    #[serde(default)]
    single_description: bool,
    #[serde(rename = "validateOnStorefront")]
    #[serde(default)]
    validate_on_storefront: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cultures {
    #[serde(rename = "boldallowed")]
    bold_allowed: bool,
    #[serde(rename = "endonym")]
    endonym: String,
    #[serde(rename = "exonym")]
    exonym: String,
    #[serde(rename = "fontscript")]
    fontscript: String,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "released")]
    released: bool,
    #[serde(rename = "uilabels")]
    ui_labels: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Decks {
    #[serde(rename = "defaultcard")]
    default_card: Option<String>,
    #[serde(rename = "desc")]
    desc: Option<String>,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "label")]
    label: String,
    #[serde(rename = "resetonexhaustion")]
    reset_on_exhaustion: bool,
    #[serde(rename = "spec")]
    spec: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dicta {
    #[serde(rename = "AlternativeDefaultWorldSpherePaths")]
    alternative_default_world_sphere_paths: Vec<String>,
    #[serde(rename = "DefaultCardBack")]
    default_card_back: String,
    #[serde(rename = "DefaultGameSpeed")]
    // TODO parse f32 from String
    default_game_speed: String,
    #[serde(rename = "DefaultWorldSpherePath")]
    default_world_sphere_path: String,
    #[serde(rename = "GameOverScene")]
    game_over_scene: String,
    #[serde(rename = "LoadingScene")]
    loading_scene: String,
    #[serde(rename = "LogoScene")]
    logo_scene: String,
    #[serde(rename = "MaxSuitabilityPulseFrequency")]
    // TODO parse f32 from String
    max_suitability_pulse_frequency: String,
    #[serde(rename = "MenuScene")]
    menu_scene: String,
    #[serde(rename = "NewGameScene")]
    new_game_scene: String,
    #[serde(rename = "NoteElementId")]
    note_element_id: String,
    #[serde(rename = "PlayfieldScene")]
    playfield_scene: String,
    #[serde(rename = "QuoteScene")]
    quote_scene: String,
    #[serde(rename = "StoredManifestation")]
    stored_manifestation: String,
    #[serde(rename = "StoredPhyicalManifestation")]
    stored_phyical_manifestation: String,
    #[serde(rename = "SuitabilityPulseSpeed")]
    // TODO parse f32 from String
    suitability_pulse_speed: String,
    #[serde(rename = "WorldSphereType")]
    world_sphere_type: String,
    #[serde(rename = "id")]
    id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Elements {
    #[serde(rename = "achievements")]
    #[serde(default)]
    achievements: Vec<String>,
    #[serde(rename = "AlphaLabelOverride")]
    alpha_label_override: Option<String>,
    #[serde(default)]
    ambits: BTreeMap<String, u32>,
    #[serde(default)]
    aspects: BTreeMap<String, u32>,
    audio: Option<String>,
    comments: Option<String>,
    commute: Option<Vec<String>>,
    #[serde(rename = "decayto")]
    decay_to: Option<String>,
    #[serde(alias = "Desc")]
    desc: Option<String>,
    icon: Option<String>,
    #[serde(alias = "ID")]
    id: String,
    imms: Option<Imms>,
    inherits: Option<String>,
    #[serde(rename = "isaspect")]
    #[serde(alias = "isAspect")]
    #[serde(default)]
    is_aspect: bool,
    #[serde(alias = "isHidden")]
    #[serde(rename = "ishidden")]
    #[serde(default)]
    #[serde(deserialize_with = "parse_bool")]
    is_hidden: bool,
    #[serde(alias = "Label")]
    label: Option<String>,
    lifetime: Option<f32>,
    #[serde(rename = "manifestationtype")]
    #[serde(alias = "ManifestationType")]
    manifestation_type: Option<String>,
    #[serde(rename = "metafictional")]
    #[serde(default)]
    metafictional: bool,
    #[serde(alias = "noArtNeeded")]
    #[serde(rename = "noartneeded")]
    #[serde(default)]
    no_art_needed: bool,
    #[serde(default)]
    resaturate: bool,
    #[serde(rename = "reverseambittablesdisplay")]
    #[serde(default = "bool_true")]
    reverse_ambit_tables_display: bool,
    #[serde(default)]
    slots: Vec<Slot>,
    sort: Option<String>,
    #[serde(deserialize_with = "parse_bool")]
    unique: bool,
    #[serde(rename = "uniquenessgroup")]
    uniqueness_group: Option<String>,
    #[serde(rename = "verbicon")]
    verb_icon: Option<String>,
    #[serde(rename = "xexts")]
    #[serde(default)]
    xexts: BTreeMap<String, String>,
    #[serde(rename = "xtriggers")]
    xtriggers: StringOrArray<BTreeMap<String, StringOrStruct<XTrigger>>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Imms {
    effects: BTreeMap<String, Value>,
    reqs: BTreeMap<String, u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Slot {
    #[serde(rename = "actionid")]
    action_id: String,
    description: Option<String>,
    #[serde(default)]
    essential: BTreeMap<String, u32>,
    #[serde(default)]
    required: BTreeMap<String, u32>,
    #[serde(default)]
    forbidden: BTreeMap<String, i32>,
    id: String,
    #[serde(rename = "ifaspectspresent")]
    if_aspects_present: BTreeMap<String, u32>,
    label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum XTriggers {
    Str(String),
    Arr(Vec<BTreeMap<String, StringOrStruct<XTrigger>>>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct XTrigger {
    #[serde(default)]
    additive: bool,
    #[serde(default = "u32_100")]
    chance: u32,
    // TODO: parse into `""`, `"^"` or `String`
    id: String,
    // TODO: parse into `-1`, `1..=3` or `"^"`
    level: Option<Value>,
    // TODO: parse into `"mutate"`, `"spawn"` or `"transform"`
    morpheffect: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Endings {
    achievements: Vec<String>,
    #[serde(alias = "Desc")]
    desc: Option<String>,
    // TODO: maybe parse into limited list:
    // `"Enigmatic"`, `"Grand"`, `"Pale"`, `"positive"`
    flavour: String,
    id: String,
    image: String,
    label: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Legacies {
    #[serde(rename = "availableWithoutEndingMatch")]
    available_without_ending_match: bool,
    desc: String,
    // TODO: This is likely something different in CS
    effects: (),
    family: String,
    #[serde(rename = "fromending")]
    from_ending: String,
    id: String,
    image: String,
    label: String,
    #[serde(rename = "newstart")]
    // TODO: might need a default
    new_start: bool,
    #[serde(rename = "startdescription")]
    start_description: String,
    #[serde(rename = "startingverbid")]
    starting_verb_id: String,
    #[serde(rename = "startup")]
    startup: Vec<LegaciesStartup>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegaciesStartup {
    id: String,
    #[serde(rename = "topath")]
    to_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Recipes {
    #[serde(default)]
    achievements: Vec<String>,
    #[serde(rename = "actionid")]
    action_id: Option<String>,
    #[serde(default)]
    alt: Vec<RecipesAlt>,
    #[serde(rename = "ambittable")]
    #[serde(deserialize_with = "parse_bool")]
    ambit_table: bool,
    #[serde(deserialize_with = "recipes_aspects")]
    #[serde(default)]
    aspects: Option<BTreeMap<String, u32>>,
    #[serde(rename = "audiooneshot")]
    audio_oneshot: Option<String>,
    #[serde(default = "bool_true")]
    blocks: bool,
    comments: Option<String>,
    #[serde(deserialize_with = "parse_bool")]
    craftable: bool,
    #[serde(rename = "deckeffects")]
    deck_effects: Option<BTreeMap<String, u32>>,
    #[serde(alias = "Desc")]
    desc: Option<String>,
    effects: Option<BTreeMap<String, i32>>,
    ending: Option<String>,
    #[serde(rename = "extantreqs")]
    extant_reqs: Option<BTreeMap<String, i32>>,
    // TODO: parse value as 0, 1, 4, "queue", "set"
    fx: Option<BTreeMap<String, Value>>,
    #[serde(rename = "fxreqs")]
    fx_reqs: Option<BTreeMap<String, String>>,
    #[serde(rename = "greq")]
    g_req: Option<BTreeMap<String, i32>>,
    #[serde(rename = "hintonly")]
    #[serde(default)]
    hint_only: bool,
    icon: Option<String>,
    id: String,
    inherits: Option<String>,
    #[serde(rename = "internaldeck")]
    internal_deck: Option<RecipesInternalDeck>,
    #[serde(alias = "Label")]
    label: Option<String>,
    #[serde(rename = "lalt")]
    l_alt: Option<String>,
    #[serde(rename = "linked")]
    linked: (),
    #[serde(rename = "mutations")]
    mutations: (),
    #[serde(rename = "ngreq")]
    ng_req: (),
    #[serde(rename = "notable")]
    notable: (),
    #[serde(rename = "preface")]
    preface: (),
    #[serde(rename = "preslots")]
    pre_slots: (),
    #[serde(rename = "purge")]
    purge: (),
    #[serde(rename = "reqs")]
    reqs: (),
    #[serde(rename = "run")]
    run: (),
    #[serde(rename = "slots")]
    slots: (),
    #[serde(rename = "startdescription")]
    #[serde(alias = "StartDescription")]
    start_description: (),
    #[serde(rename = "startlabel")]
    start_label: (),
    #[serde(rename = "warmup")]
    warmup: (),
    #[serde(rename = "xpans")]
    xpans: (),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesAlt {
    id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesInternalDeck {
    #[serde(rename = "defaultcard")]
    // TODO: I would guess this his `Option::None` in CS
    default_card: String,
    draws: u32,
    #[serde(rename = "resetonexhaustion")]
    reset_on_exhaustion: bool,
    spec: Vec<String>,
}
