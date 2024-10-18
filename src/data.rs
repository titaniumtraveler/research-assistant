use crate::data::serde_impl::{bool_true, parse_bool, parse_opt_f32, recipes_aspects, u32_100};
use serde::{Deserialize, Serialize};
use serde_impl::{parse_opt_u32, StringOrStringArray};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

pub use serde_impl::{StringMapOrArray, StringOrI32, StringOrStruct};

mod serde_impl;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Data {
    #[serde(default)]
    pub achievements: Vec<Achievements>,
    #[serde(default)]
    pub cultures: Vec<Cultures>,
    #[serde(default)]
    pub decks: Vec<Decks>,
    #[serde(default)]
    pub dicta: Vec<Dicta>,
    #[serde(default)]
    pub elements: Vec<Elements>,
    #[serde(default)]
    pub endings: Vec<Endings>,
    #[serde(default)]
    pub legacies: Vec<Legacies>,
    #[serde(default)]
    pub levers: Vec<Levers>,
    #[serde(default)]
    pub portals: Vec<Portals>,
    #[serde(default)]
    pub recipes: Vec<Recipes>,
    #[serde(default)]
    pub settings: Vec<Settings>,
    #[serde(default)]
    pub verbs: Vec<Verbs>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Achievements {
    pub category: Option<String>,
    #[serde(rename = "descriptionunlocked")]
    pub description_unlocked: Option<String>,
    // TODO: convert `""` to `Option::None`
    #[serde(rename = "iconUnlocked")]
    pub icon_unlocked: String,
    pub id: String,
    #[serde(rename = "isCategory")]
    #[serde(default)]
    pub is_category: bool,
    #[serde(rename = "isHidden")]
    #[serde(default)]
    pub is_hidden: bool,
    pub label: String,
    #[serde(rename = "singleDescription")]
    #[serde(default)]
    pub single_description: bool,
    #[serde(rename = "validateOnStorefront")]
    #[serde(default)]
    pub validate_on_storefront: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cultures {
    #[serde(rename = "boldallowed")]
    pub bold_allowed: bool,
    #[serde(rename = "endonym")]
    pub endonym: String,
    #[serde(rename = "exonym")]
    pub exonym: String,
    #[serde(rename = "fontscript")]
    pub fontscript: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "released")]
    pub released: bool,
    #[serde(rename = "uilabels")]
    pub ui_labels: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Decks {
    pub comments: Option<String>,
    #[serde(rename = "drawmessages")]
    pub draw_messages: Option<BTreeMap<String, String>>,
    #[serde(rename = "defaultcard")]
    pub default_card: Option<String>,
    #[serde(rename = "desc")]
    #[serde(alias = "description")]
    pub desc: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "resetonexhaustion")]
    #[serde(default)]
    pub reset_on_exhaustion: bool,
    #[serde(rename = "spec")]
    pub spec: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dicta {
    #[serde(rename = "AlternativeDefaultWorldSpherePaths")]
    pub alternative_default_world_sphere_paths: Vec<String>,
    #[serde(rename = "DefaultCardBack")]
    pub default_card_back: Option<String>,
    #[serde(rename = "DefaultGameSpeed")]
    // TODO parse f32 from String
    pub default_game_speed: String,
    #[serde(rename = "DefaultLongTravelDuration")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    pub default_long_travel_duration: Option<f32>,
    #[serde(rename = "DefaultQuickTravelDuration")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    pub default_quick_travel_duration: Option<f32>,
    #[serde(rename = "DefaultTravelDuration")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    pub default_travel_duration: Option<f32>,
    #[serde(rename = "DefaultWorldSpherePath")]
    pub default_world_sphere_path: String,
    #[serde(rename = "GameOverScene")]
    pub game_over_scene: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "LoadingScene")]
    pub loading_scene: Option<String>,
    #[serde(rename = "LogoScene")]
    pub logo_scene: String,
    #[serde(rename = "MaxSuitabilityPulseFrequency")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    pub max_suitability_pulse_frequency: Option<f32>,
    #[serde(rename = "MenuScene")]
    pub menu_scene: String,
    #[serde(rename = "NewGameScene")]
    pub new_game_scene: String,
    #[serde(rename = "NoteElementId")]
    pub note_element_id: String,
    #[serde(rename = "PlayfieldScene")]
    pub playfield_scene: String,
    #[serde(rename = "QuoteScene")]
    pub quote_scene: String,
    #[serde(rename = "StoredManifestation")]
    pub stored_manifestation: Option<String>,
    #[serde(rename = "StoredPhyicalManifestation")]
    pub stored_phyical_manifestation: Option<String>,
    #[serde(rename = "SuitabilityPulseSpeed")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    pub suitability_pulse_speed: Option<f32>,
    #[serde(rename = "WorldSphereType")]
    pub world_sphere_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Elements {
    #[serde(rename = "achievements")]
    #[serde(default)]
    pub achievements: Vec<String>,
    #[serde(rename = "AlphaLabelOverride")]
    pub alpha_label_override: Option<String>,
    #[serde(default)]
    pub ambits: BTreeMap<String, u32>,
    #[serde(default)]
    pub aspects: BTreeMap<String, u32>,
    pub audio: Option<String>,
    #[serde(rename = "burnTo")]
    pub burn_to: Option<String>,
    pub comments: Option<String>,
    pub commute: Option<Vec<String>>,
    #[serde(rename = "decayto")]
    #[serde(alias = "decayTo")]
    pub decay_to: Option<String>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    pub description: Option<String>,
    pub icon: Option<String>,
    #[serde(alias = "ID")]
    pub id: String,
    #[serde(rename = "induces")]
    pub induces: Option<Vec<ElementsInduces>>,
    pub imms: Option<Vec<ElementsImms>>,
    pub inherits: Option<String>,
    #[serde(rename = "isaspect")]
    #[serde(alias = "isAspect")]
    #[serde(default)]
    pub is_aspect: bool,
    #[serde(alias = "isHidden")]
    #[serde(rename = "ishidden")]
    #[serde(default)]
    #[serde(deserialize_with = "parse_bool")]
    pub is_hidden: bool,
    #[serde(alias = "Label")]
    pub label: Option<String>,
    pub lever: Option<String>,
    pub lifetime: Option<f32>,
    #[serde(rename = "manifestationtype")]
    #[serde(alias = "ManifestationType")]
    pub manifestation_type: Option<String>,
    #[serde(rename = "metafictional")]
    #[serde(default)]
    pub metafictional: bool,
    #[serde(alias = "noArtNeeded")]
    #[serde(rename = "noartneeded")]
    #[serde(default)]
    pub no_art_needed: bool,
    #[serde(default)]
    pub resaturate: bool,
    #[serde(rename = "reverseambittablesdisplay")]
    #[serde(default)]
    // TODO: This *might* default to true
    pub reverse_ambit_tables_display: bool,
    #[serde(default)]
    pub slots: Vec<ElementsSlots>,
    pub sort: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "parse_bool")]
    pub unique: bool,
    #[serde(rename = "uniquenessgroup")]
    pub uniqueness_group: Option<String>,
    #[serde(rename = "verbicon")]
    pub verb_icon: Option<String>,
    #[serde(rename = "xexts")]
    #[serde(default)]
    pub xexts: BTreeMap<String, String>,
    #[serde(rename = "xtriggers")]
    pub xtriggers: Option<BTreeMap<String, StringMapOrArray<StringOrStruct<ElementsXTriggers>>>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementsInduces {
    pub chance: u32,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementsImms {
    pub effects: BTreeMap<String, Value>,
    pub reqs: BTreeMap<String, u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementsSlots {
    #[serde(rename = "actionid")]
    pub action_id: String,
    #[serde(rename = "consumes")]
    #[serde(default)]
    pub consumes: bool,
    pub description: Option<String>,
    #[serde(default)]
    pub essential: BTreeMap<String, u32>,
    #[serde(default)]
    pub forbidden: BTreeMap<String, i32>,
    pub id: String,
    #[serde(rename = "ifaspectspresent")]
    pub if_aspects_present: Option<BTreeMap<String, u32>>,
    pub label: Option<String>,
    #[serde(default)]
    pub required: BTreeMap<String, u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementsXTriggers {
    #[serde(default)]
    pub additive: bool,
    #[serde(default = "u32_100")]
    pub chance: u32,
    // TODO: parse into `""`, `"^"` or `String`
    pub id: String,
    // TODO: parse into `-1`, `1..=3` or `"^"`
    pub level: Option<Value>,
    // TODO: parse into `"mutate"`, `"spawn"` or `"transform"`
    pub morpheffect: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Endings {
    pub achievements: Option<Vec<String>>,
    pub anim: Option<String>,
    pub comments: Option<String>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    pub description: Option<String>,
    // TODO: maybe parse into limited list:
    // `"Enigmatic"`, `"Grand"`, `"Pale"`, `"positive"`
    pub flavour: String,
    pub id: String,
    pub image: String,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Legacies {
    #[serde(rename = "$derives")]
    pub derives: Option<String>,
    #[serde(rename = "availableWithoutEndingMatch")]
    pub available_without_ending_match: bool,
    pub comments: Option<String>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    pub description: String,
    pub effects: Option<BTreeMap<String, u32>>,
    #[serde(rename = "excludesOnEnding")]
    pub excludes_on_ending: Option<Vec<String>>,
    pub family: Option<String>,
    #[serde(rename = "fromending")]
    pub from_ending: String,
    pub id: String,
    pub image: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "newstart")]
    #[serde(default)]
    pub new_start: bool,
    #[serde(rename = "startdescription")]
    pub start_description: Option<String>,
    #[serde(rename = "startingverbid")]
    pub starting_verb_id: Option<String>,
    #[serde(rename = "startup")]
    pub startup: Option<Vec<LegaciesStartup>>,
    #[serde(rename = "statusbarelements")]
    pub statusbar_elements: Option<Vec<StringOrStruct<LegaciesStatusbarElements>>>,
    #[serde(rename = "tablecoverimage")]
    pub table_cover_image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegaciesStartup {
    pub id: String,
    #[serde(rename = "topath")]
    pub to_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegaciesStatusbarElements {
    #[serde(rename = "format")]
    pub format: Option<Vec<String>>,
    #[serde(rename = "ids")]
    pub ids: StringOrStringArray,
    #[serde(rename = "styles")]
    pub styles: StringOrStringArray,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Levers {
    #[serde(rename = "comments")]
    pub comments: Option<String>,
    #[serde(rename = "defaultValue")]
    pub default_value: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "onGameEnd")]
    pub on_game_end: bool,
    #[serde(rename = "redirects")]
    pub redirects: Option<BTreeMap<String, String>>,
    #[serde(rename = "requiredScore")]
    pub required_score: u32,
    #[serde(rename = "weights")]
    pub weights: Option<BTreeMap<String, i32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Portals {
    pub consequences: Option<Vec<PortalsConsequences>>,
    pub description: String,
    #[serde(rename = "egressid")]
    pub egress_id: String,
    pub icon: String,
    pub id: String,
    pub label: String,
    #[serde(rename = "otherworldid")]
    pub otherworld_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PortalsConsequences {
    #[serde(rename = "deckeffects")]
    // TODO: This seems to be a single `{ "<key>": 1 }` pair in every case
    pub deckeffects: Option<BTreeMap<String, u32>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "topath")]
    pub topath: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Recipes {
    #[serde(default)]
    pub achievements: Vec<String>,
    #[serde(rename = "actionid")]
    #[serde(alias = "actionId")]
    pub action_id: Option<String>,
    pub alt: Option<Vec<RecipesAlt>>,
    #[serde(rename = "ambittable")]
    #[serde(deserialize_with = "parse_bool")]
    #[serde(default)]
    pub ambit_table: bool,
    #[serde(deserialize_with = "recipes_aspects")]
    #[serde(default)]
    pub aspects: Option<BTreeMap<String, i32>>,
    #[serde(rename = "audiooneshot")]
    pub audio_oneshot: Option<String>,
    #[serde(default = "bool_true")]
    pub blocks: bool,
    #[serde(rename = "burnimage")]
    pub burnimage: Option<String>,
    pub comments: Option<String>,
    #[serde(deserialize_with = "parse_bool")]
    #[serde(default)]
    pub craftable: bool,
    #[serde(rename = "deckeffects")]
    pub deck_effects: Option<BTreeMap<String, u32>>,
    #[serde(rename = "deleteverb")]
    pub deleteverb: Option<BTreeMap<String, u32>>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    pub description: Option<String>,
    pub effects: Option<BTreeMap<String, StringOrI32>>,
    pub ending: Option<String>,
    #[serde(rename = "extantreqs")]
    pub extant_reqs: Option<BTreeMap<String, i32>>,
    // TODO: parse value as 0, 1, 4, "queue", "set"
    pub fx: Option<BTreeMap<String, Value>>,
    #[serde(rename = "fxreqs")]
    pub fx_reqs: Option<BTreeMap<String, String>>,
    #[serde(rename = "greq")]
    pub g_req: Option<BTreeMap<String, i32>>,
    #[serde(rename = "haltverb")]
    pub haltverb: Option<BTreeMap<String, u32>>,
    #[serde(rename = "hintonly")]
    #[serde(default)]
    pub hint_only: bool,
    pub icon: Option<String>,
    pub id: String,
    #[serde(rename = "inductions")]
    pub inductions: Option<Vec<RecipesInductions>>,
    pub inherits: Option<String>,
    #[serde(rename = "internaldeck")]
    pub internal_deck: Option<RecipesInternalDeck>,
    #[serde(alias = "Label")]
    pub label: Option<String>,
    #[serde(rename = "lalt")]
    pub l_alt: Option<String>,
    pub linked: Option<StringMapOrArray<StringOrStruct<RecipesLinked>>>,
    #[serde(rename = "maxexecutions")]
    pub max_executions: Option<u32>,
    pub mutations: Option<StringMapOrArray<StringOrStruct<RecipesMutations>>>,
    // TODO: this could be just BTreeSet<String>,
    // because it really is just `{ "<string>": 1 }` for all instances
    #[serde(rename = "ngreq")]
    pub ng_req: Option<BTreeMap<String, i32>>,
    #[serde(default)]
    pub notable: bool,
    #[serde(rename = "portaleffect")]
    pub portal_effect: Option<String>,
    pub preface: Option<String>,
    pub preslots: Option<Vec<RecipesPreslots>>,
    pub purge: Option<BTreeMap<String, u32>>,
    #[serde(alias = "reqs")]
    pub requirements: Option<BTreeMap<String, StringOrI32>>,
    #[serde(rename = "run")]
    pub run: Option<String>,
    #[serde(rename = "signalEndingFlavour")]
    pub signal_ending_flavour: Option<String>,
    #[serde(rename = "signalimportantloop")]
    #[serde(default)]
    pub signal_important_loop: bool,
    #[serde(rename = "slots")]
    pub slots: Option<Vec<RecipesSlots>>,
    #[serde(rename = "startdescription")]
    #[serde(alias = "StartDescription")]
    pub start_description: Option<String>,
    #[serde(rename = "startlabel")]
    pub start_label: Option<String>,
    #[serde(rename = "tablereqs")]
    pub table_reqs: Option<BTreeMap<String, u32>>,
    #[serde(deserialize_with = "parse_opt_u32")]
    #[serde(default)]
    pub warmup: Option<u32>,
    #[serde(rename = "xpans")]
    pub xpans: Option<BTreeMap<String, u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesAlt {
    pub actionid: Option<String>,
    #[serde(default)]
    pub additional: bool,
    #[serde(rename = "challenges")]
    // TODO: Could be parsed as `BTreeMap<String, Base>` (where `Base` matches `"base"`)
    pub challenges: Option<BTreeMap<String, String>>,
    pub chance: Option<u32>,
    #[serde(rename = "craftable")]
    #[serde(default = "bool_true")]
    pub craftable: bool,
    #[serde(rename = "deckeffects")]
    pub deck_effects: Option<BTreeMap<String, u32>>,
    pub description: Option<String>,
    pub effects: Option<BTreeMap<String, StringOrI32>>,
    pub ending: Option<String>,
    pub expulsion: Option<RecipesLinkedExpulsion>,
    #[serde(rename = "extantreqs")]
    pub extant_reqs: Option<BTreeMap<String, i32>>,
    pub id: String,
    pub label: Option<String>,
    pub mutations: Option<StringMapOrArray<StringOrStruct<RecipesMutations>>>,
    pub requirements: Option<BTreeMap<String, i32>>,
    #[serde(rename = "signalEndingFlavour")]
    pub signal_ending_flavour: Option<String>,
    #[serde(rename = "startdescription")]
    pub start_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesInternalDeck {
    #[serde(rename = "defaultcard")]
    pub default_card: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    pub draws: u32,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "resetonexhaustion")]
    pub reset_on_exhaustion: bool,
    pub spec: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesLinked {
    #[serde(rename = "actionid")]
    pub actionid: Option<String>,
    #[serde(default)]
    pub additional: bool,
    // TODO: Could be parsed as `BTreeMap<String, Base>` (where `Base` matches `"base"`)
    pub challenges: Option<BTreeMap<String, String>>,
    #[serde(rename = "chance")]
    pub chance: Option<u32>,
    #[serde(rename = "effects")]
    pub effects: Option<BTreeMap<String, u32>>,
    pub expulsion: Option<RecipesLinkedExpulsion>,
    #[serde(rename = "extantreqs")]
    pub extant_reqs: Option<BTreeMap<String, i32>>,
    #[serde(rename = "id")]
    pub id: String,
    pub label: Option<String>,
    // TODO: consolidate that with `Linked`
    // Find out what it is actually doing for that
    pub linked: Option<Vec<BTreeMap<String, String>>>,
    pub mutations: Option<StringMapOrArray<StringOrStruct<RecipesMutations>>>,
    #[serde(rename = "outputpath")]
    // TODO: `^` carrot?!
    pub output_path: Option<String>,
    #[serde(rename = "purge")]
    pub purge: Option<BTreeMap<String, u32>>,
    pub requirements: Option<BTreeMap<String, i32>>,
    #[serde(rename = "shuffle")]
    #[serde(default)]
    pub shuffle: bool,
    #[serde(rename = "startdescription")]
    pub startdescription: Option<String>,
    #[serde(rename = "topath")]
    pub topath: Option<String>,
    #[serde(rename = "warmup")]
    pub warmup: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesInductions {
    pub chance: Option<u32>,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesLinkedExpulsion {
    // TODO: this could be just String, or BTreeSet<String>,
    // because it really is just `{ "<string>": 1 }` for all instances
    pub filter: BTreeMap<String, u32>,
    // TODO: Seems to just be 1 or 99
    pub limit: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesMutations {
    #[serde(deserialize_with = "parse_bool")]
    #[serde(default)]
    pub additive: bool,
    pub filter: String,
    pub level: StringOrI32,
    pub mutate: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesPreslots {
    #[serde(rename = "description")]
    pub description: Option<String>,
    // TODO: This again could maybe just be BTreeSet<String>
    pub essential: Option<BTreeMap<String, u32>>,
    pub forbidden: Option<BTreeMap<String, u32>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: String,
    pub required: Option<BTreeMap<String, u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesSlots {
    #[serde(rename = "actionid")]
    pub actionid: Option<String>,
    #[serde(rename = "consumes")]
    #[serde(default)]
    pub consumes: bool,
    pub description: Option<String>,
    #[serde(rename = "essential")]
    pub essential: Option<BTreeMap<String, i32>>,
    #[serde(rename = "forbidden")]
    pub forbidden: Option<BTreeMap<String, i32>>,
    #[serde(rename = "frompath")]
    pub frompath: Option<String>,
    #[serde(rename = "greedy")]
    #[serde(default)]
    pub greedy: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "required")]
    pub required: Option<BTreeMap<String, i32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub datatype: Option<String>,
    #[serde(rename = "defaultvalue")]
    #[serde(alias = "defaultValue")]
    pub default_value: Option<Value>,
    pub hint: Option<String>,
    pub id: String,
    #[serde(rename = "maxvalue")]
    pub max_value: Option<i32>,
    #[serde(rename = "minvalue")]
    pub min_value: Option<i32>,
    #[serde(rename = "PlatformDefaultValues")]
    pub platform_default_values: Option<BTreeMap<String, String>>,
    #[serde(rename = "tabid")]
    pub tab_id: Option<String>,
    pub ui: Option<String>,
    // TODO: This could be parsed as `Option<BTreeMap<i32, String>>`
    #[serde(rename = "valuelabels")]
    pub value_labels: Option<BTreeMap<String, String>>,
    // TODO: Same as [`Self::value_labels`]
    #[serde(rename = "valuenotifications")]
    pub valuenotifications: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Verbs {
    #[serde(rename = "ambits")]
    #[serde(default)]
    pub ambits: bool,
    #[serde(rename = "aspects")]
    pub aspects: Option<BTreeMap<String, u32>>,
    #[serde(rename = "audio")]
    pub audio: Option<String>,
    #[serde(rename = "category")]
    pub category: Option<String>,
    #[serde(rename = "comments")]
    pub comments: Option<String>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    pub description: Option<String>,
    #[serde(rename = "hints")]
    pub hints: Option<Vec<String>>,
    pub icon: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "maxnotes")]
    #[serde(alias = "maxNotes")]
    #[serde(alias = "MaxNotes")]
    pub max_notes: Option<u32>,
    #[serde(default)]
    pub multiple: bool,
    pub slot: Option<VerbsSlot>,
    pub slots: Option<Vec<VerbsSlot>>,
    #[serde(default)]
    #[serde(deserialize_with = "parse_bool")]
    pub spontaneous: bool,
    pub xtriggers: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerbsSlot {
    pub description: Option<String>,
    pub essential: Option<BTreeMap<String, u32>>,
    pub forbidden: Option<BTreeMap<String, u32>>,
    pub id: String,
    pub label: Option<String>,
    pub required: Option<BTreeMap<String, u32>>,
}
