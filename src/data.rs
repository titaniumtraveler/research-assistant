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
    achievements: Vec<Achievements>,
    #[serde(default)]
    cultures: Vec<Cultures>,
    #[serde(default)]
    decks: Vec<Decks>,
    #[serde(default)]
    dicta: Vec<Dicta>,
    #[serde(default)]
    elements: Vec<Elements>,
    #[serde(default)]
    endings: Vec<Endings>,
    #[serde(default)]
    legacies: Vec<Legacies>,
    #[serde(default)]
    levers: Vec<Levers>,
    #[serde(default)]
    portals: Vec<Portals>,
    #[serde(default)]
    recipes: Vec<Recipes>,
    #[serde(default)]
    settings: Vec<Settings>,
    #[serde(default)]
    verbs: Vec<Verbs>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Achievements {
    category: Option<String>,
    #[serde(rename = "descriptionunlocked")]
    description_unlocked: Option<String>,
    // TODO: convert `""` to `Option::None`
    #[serde(rename = "iconUnlocked")]
    icon_unlocked: String,
    id: String,
    #[serde(rename = "isCategory")]
    #[serde(default)]
    is_category: bool,
    #[serde(rename = "isHidden")]
    #[serde(default)]
    is_hidden: bool,
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
    comments: Option<String>,
    #[serde(rename = "drawmessages")]
    draw_messages: Option<BTreeMap<String, String>>,
    #[serde(rename = "defaultcard")]
    default_card: Option<String>,
    #[serde(rename = "desc")]
    #[serde(alias = "description")]
    desc: Option<String>,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "label")]
    label: Option<String>,
    #[serde(rename = "resetonexhaustion")]
    #[serde(default)]
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
    default_card_back: Option<String>,
    #[serde(rename = "DefaultGameSpeed")]
    // TODO parse f32 from String
    default_game_speed: String,
    #[serde(rename = "DefaultLongTravelDuration")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    default_long_travel_duration: Option<f32>,
    #[serde(rename = "DefaultQuickTravelDuration")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    default_quick_travel_duration: Option<f32>,
    #[serde(rename = "DefaultTravelDuration")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    default_travel_duration: Option<f32>,
    #[serde(rename = "DefaultWorldSpherePath")]
    default_world_sphere_path: String,
    #[serde(rename = "GameOverScene")]
    game_over_scene: String,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "LoadingScene")]
    loading_scene: Option<String>,
    #[serde(rename = "LogoScene")]
    logo_scene: String,
    #[serde(rename = "MaxSuitabilityPulseFrequency")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    max_suitability_pulse_frequency: Option<f32>,
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
    stored_manifestation: Option<String>,
    #[serde(rename = "StoredPhyicalManifestation")]
    stored_phyical_manifestation: Option<String>,
    #[serde(rename = "SuitabilityPulseSpeed")]
    #[serde(deserialize_with = "parse_opt_f32")]
    #[serde(default)]
    suitability_pulse_speed: Option<f32>,
    #[serde(rename = "WorldSphereType")]
    world_sphere_type: String,
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
    #[serde(rename = "burnTo")]
    burn_to: Option<String>,
    comments: Option<String>,
    commute: Option<Vec<String>>,
    #[serde(rename = "decayto")]
    #[serde(alias = "decayTo")]
    decay_to: Option<String>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    description: Option<String>,
    icon: Option<String>,
    #[serde(alias = "ID")]
    id: String,
    #[serde(rename = "induces")]
    induces: Option<Vec<ElementsInduces>>,
    imms: Option<Vec<ElementsImms>>,
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
    lever: Option<String>,
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
    #[serde(default)]
    // TODO: This *might* default to true
    reverse_ambit_tables_display: bool,
    #[serde(default)]
    slots: Vec<ElementsSlots>,
    sort: Option<String>,
    #[serde(default)]
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
    xtriggers: Option<BTreeMap<String, StringMapOrArray<StringOrStruct<ElementsXTriggers>>>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ElementsInduces {
    chance: u32,
    id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementsImms {
    effects: BTreeMap<String, Value>,
    reqs: BTreeMap<String, u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementsSlots {
    #[serde(rename = "actionid")]
    action_id: String,
    #[serde(rename = "consumes")]
    #[serde(default)]
    consumes: bool,
    description: Option<String>,
    #[serde(default)]
    essential: BTreeMap<String, u32>,
    #[serde(default)]
    forbidden: BTreeMap<String, i32>,
    id: String,
    #[serde(rename = "ifaspectspresent")]
    if_aspects_present: Option<BTreeMap<String, u32>>,
    label: Option<String>,
    #[serde(default)]
    required: BTreeMap<String, u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementsXTriggers {
    #[serde(default)]
    additive: bool,
    #[serde(default = "u32_100")]
    chance: u32,
    // TODO: parse into `""`, `"^"` or `String`
    id: String,
    // TODO: parse into `-1`, `1..=3` or `"^"`
    level: Option<Value>,
    // TODO: parse into `"mutate"`, `"spawn"` or `"transform"`
    morpheffect: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Endings {
    achievements: Option<Vec<String>>,
    anim: Option<String>,
    comments: Option<String>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    description: Option<String>,
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
    #[serde(rename = "$derives")]
    derives: Option<String>,
    #[serde(rename = "availableWithoutEndingMatch")]
    available_without_ending_match: bool,
    comments: Option<String>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    description: String,
    effects: Option<BTreeMap<String, u32>>,
    #[serde(rename = "excludesOnEnding")]
    excludes_on_ending: Option<Vec<String>>,
    family: Option<String>,
    #[serde(rename = "fromending")]
    from_ending: String,
    id: String,
    image: Option<String>,
    label: Option<String>,
    #[serde(rename = "newstart")]
    #[serde(default)]
    new_start: bool,
    #[serde(rename = "startdescription")]
    start_description: Option<String>,
    #[serde(rename = "startingverbid")]
    starting_verb_id: Option<String>,
    #[serde(rename = "startup")]
    startup: Option<Vec<LegaciesStartup>>,
    #[serde(rename = "statusbarelements")]
    statusbar_elements: Option<Vec<StringOrStruct<LegaciesStatusbarElements>>>,
    #[serde(rename = "tablecoverimage")]
    table_cover_image: Option<String>,
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
pub struct LegaciesStatusbarElements {
    #[serde(rename = "format")]
    format: Option<Vec<String>>,
    #[serde(rename = "ids")]
    ids: StringOrStringArray,
    #[serde(rename = "styles")]
    styles: StringOrStringArray,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Levers {
    #[serde(rename = "comments")]
    comments: Option<String>,
    #[serde(rename = "defaultValue")]
    default_value: String,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "onGameEnd")]
    on_game_end: bool,
    #[serde(rename = "redirects")]
    redirects: Option<BTreeMap<String, String>>,
    #[serde(rename = "requiredScore")]
    required_score: u32,
    #[serde(rename = "weights")]
    weights: Option<BTreeMap<String, i32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Portals {
    consequences: Option<Vec<PortalsConsequences>>,
    description: String,
    #[serde(rename = "egressid")]
    egress_id: String,
    icon: String,
    id: String,
    label: String,
    #[serde(rename = "otherworldid")]
    otherworld_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PortalsConsequences {
    #[serde(rename = "deckeffects")]
    // TODO: This seems to be a single `{ "<key>": 1 }` pair in every case
    deckeffects: Option<BTreeMap<String, u32>>,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "topath")]
    topath: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Recipes {
    #[serde(default)]
    achievements: Vec<String>,
    #[serde(rename = "actionid")]
    #[serde(alias = "actionId")]
    action_id: Option<String>,
    alt: Option<Vec<RecipesAlt>>,
    #[serde(rename = "ambittable")]
    #[serde(deserialize_with = "parse_bool")]
    #[serde(default)]
    ambit_table: bool,
    #[serde(deserialize_with = "recipes_aspects")]
    #[serde(default)]
    aspects: Option<BTreeMap<String, i32>>,
    #[serde(rename = "audiooneshot")]
    audio_oneshot: Option<String>,
    #[serde(default = "bool_true")]
    blocks: bool,
    #[serde(rename = "burnimage")]
    burnimage: Option<String>,
    comments: Option<String>,
    #[serde(deserialize_with = "parse_bool")]
    #[serde(default)]
    craftable: bool,
    #[serde(rename = "deckeffects")]
    deck_effects: Option<BTreeMap<String, u32>>,
    #[serde(rename = "deleteverb")]
    deleteverb: Option<BTreeMap<String, u32>>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    description: Option<String>,
    effects: Option<BTreeMap<String, StringOrI32>>,
    ending: Option<String>,
    #[serde(rename = "extantreqs")]
    extant_reqs: Option<BTreeMap<String, i32>>,
    // TODO: parse value as 0, 1, 4, "queue", "set"
    fx: Option<BTreeMap<String, Value>>,
    #[serde(rename = "fxreqs")]
    fx_reqs: Option<BTreeMap<String, String>>,
    #[serde(rename = "greq")]
    g_req: Option<BTreeMap<String, i32>>,
    #[serde(rename = "haltverb")]
    haltverb: Option<BTreeMap<String, u32>>,
    #[serde(rename = "hintonly")]
    #[serde(default)]
    hint_only: bool,
    icon: Option<String>,
    id: String,
    #[serde(rename = "inductions")]
    inductions: Option<Vec<RecipesInductions>>,
    inherits: Option<String>,
    #[serde(rename = "internaldeck")]
    internal_deck: Option<RecipesInternalDeck>,
    #[serde(alias = "Label")]
    label: Option<String>,
    #[serde(rename = "lalt")]
    l_alt: Option<String>,
    linked: Option<StringMapOrArray<StringOrStruct<RecipesLinked>>>,
    #[serde(rename = "maxexecutions")]
    max_executions: Option<u32>,
    mutations: Option<StringMapOrArray<StringOrStruct<RecipesMutations>>>,
    // TODO: this could be just BTreeSet<String>,
    // because it really is just `{ "<string>": 1 }` for all instances
    #[serde(rename = "ngreq")]
    ng_req: Option<BTreeMap<String, i32>>,
    #[serde(default)]
    notable: bool,
    #[serde(rename = "portaleffect")]
    portal_effect: Option<String>,
    preface: Option<String>,
    preslots: Option<Vec<RecipesPreslots>>,
    purge: Option<BTreeMap<String, u32>>,
    #[serde(alias = "reqs")]
    requirements: Option<BTreeMap<String, StringOrI32>>,
    #[serde(rename = "run")]
    run: Option<String>,
    #[serde(rename = "signalEndingFlavour")]
    signal_ending_flavour: Option<String>,
    #[serde(rename = "signalimportantloop")]
    #[serde(default)]
    signal_important_loop: bool,
    #[serde(rename = "slots")]
    slots: Option<Vec<RecipesSlots>>,
    #[serde(rename = "startdescription")]
    #[serde(alias = "StartDescription")]
    start_description: Option<String>,
    #[serde(rename = "startlabel")]
    start_label: Option<String>,
    #[serde(rename = "tablereqs")]
    table_reqs: Option<BTreeMap<String, u32>>,
    #[serde(deserialize_with = "parse_opt_u32")]
    #[serde(default)]
    warmup: Option<u32>,
    #[serde(rename = "xpans")]
    xpans: Option<BTreeMap<String, u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesAlt {
    actionid: Option<String>,
    #[serde(default)]
    additional: bool,
    #[serde(rename = "challenges")]
    // TODO: Could be parsed as `BTreeMap<String, Base>` (where `Base` matches `"base"`)
    challenges: Option<BTreeMap<String, String>>,
    chance: Option<u32>,
    #[serde(rename = "craftable")]
    #[serde(default = "bool_true")]
    craftable: bool,
    #[serde(rename = "deckeffects")]
    deck_effects: Option<BTreeMap<String, u32>>,
    description: Option<String>,
    effects: Option<BTreeMap<String, StringOrI32>>,
    ending: Option<String>,
    expulsion: Option<RecipesLinkedExpulsion>,
    #[serde(rename = "extantreqs")]
    extant_reqs: Option<BTreeMap<String, i32>>,
    id: String,
    label: Option<String>,
    mutations: Option<StringMapOrArray<StringOrStruct<RecipesMutations>>>,
    requirements: Option<BTreeMap<String, i32>>,
    #[serde(rename = "signalEndingFlavour")]
    signal_ending_flavour: Option<String>,
    #[serde(rename = "startdescription")]
    start_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesInternalDeck {
    #[serde(rename = "defaultcard")]
    default_card: Option<String>,
    #[serde(rename = "description")]
    description: Option<String>,
    draws: u32,
    #[serde(rename = "label")]
    label: Option<String>,
    #[serde(rename = "resetonexhaustion")]
    reset_on_exhaustion: bool,
    spec: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesLinked {
    #[serde(rename = "actionid")]
    actionid: Option<String>,
    #[serde(default)]
    additional: bool,
    // TODO: Could be parsed as `BTreeMap<String, Base>` (where `Base` matches `"base"`)
    challenges: Option<BTreeMap<String, String>>,
    #[serde(rename = "chance")]
    chance: Option<u32>,
    #[serde(rename = "effects")]
    effects: Option<BTreeMap<String, u32>>,
    expulsion: Option<RecipesLinkedExpulsion>,
    #[serde(rename = "extantreqs")]
    extant_reqs: Option<BTreeMap<String, i32>>,
    #[serde(rename = "id")]
    id: String,
    label: Option<String>,
    // TODO: consolidate that with `Linked`
    // Find out what it is actually doing for that
    linked: Option<Vec<BTreeMap<String, String>>>,
    mutations: Option<StringMapOrArray<StringOrStruct<RecipesMutations>>>,
    #[serde(rename = "outputpath")]
    // TODO: `^` carrot?!
    output_path: Option<String>,
    #[serde(rename = "purge")]
    purge: Option<BTreeMap<String, u32>>,
    requirements: Option<BTreeMap<String, i32>>,
    #[serde(rename = "shuffle")]
    #[serde(default)]
    shuffle: bool,
    #[serde(rename = "startdescription")]
    startdescription: Option<String>,
    #[serde(rename = "topath")]
    topath: Option<String>,
    #[serde(rename = "warmup")]
    warmup: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesInductions {
    chance: Option<u32>,
    id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesLinkedExpulsion {
    // TODO: this could be just String, or BTreeSet<String>,
    // because it really is just `{ "<string>": 1 }` for all instances
    filter: BTreeMap<String, u32>,
    // TODO: Seems to just be 1 or 99
    limit: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesMutations {
    #[serde(deserialize_with = "parse_bool")]
    #[serde(default)]
    additive: bool,
    filter: String,
    level: StringOrI32,
    mutate: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesPreslots {
    #[serde(rename = "description")]
    description: Option<String>,
    // TODO: This again could maybe just be BTreeSet<String>
    essential: Option<BTreeMap<String, u32>>,
    forbidden: Option<BTreeMap<String, u32>>,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "label")]
    label: String,
    required: Option<BTreeMap<String, u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipesSlots {
    #[serde(rename = "actionid")]
    actionid: Option<String>,
    #[serde(rename = "consumes")]
    #[serde(default)]
    consumes: bool,
    description: Option<String>,
    #[serde(rename = "essential")]
    essential: Option<BTreeMap<String, i32>>,
    #[serde(rename = "forbidden")]
    forbidden: Option<BTreeMap<String, i32>>,
    #[serde(rename = "frompath")]
    frompath: Option<String>,
    #[serde(rename = "greedy")]
    #[serde(default)]
    greedy: bool,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "label")]
    label: Option<String>,
    #[serde(rename = "required")]
    required: Option<BTreeMap<String, i32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    datatype: Option<String>,
    #[serde(rename = "defaultvalue")]
    #[serde(alias = "defaultValue")]
    default_value: Option<Value>,
    hint: Option<String>,
    id: String,
    #[serde(rename = "maxvalue")]
    max_value: Option<i32>,
    #[serde(rename = "minvalue")]
    min_value: Option<i32>,
    #[serde(rename = "PlatformDefaultValues")]
    platform_default_values: Option<BTreeMap<String, String>>,
    #[serde(rename = "tabid")]
    tab_id: Option<String>,
    ui: Option<String>,
    // TODO: This could be parsed as `Option<BTreeMap<i32, String>>`
    #[serde(rename = "valuelabels")]
    value_labels: Option<BTreeMap<String, String>>,
    // TODO: Same as [`Self::value_labels`]
    #[serde(rename = "valuenotifications")]
    valuenotifications: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Verbs {
    #[serde(rename = "ambits")]
    #[serde(default)]
    ambits: bool,
    #[serde(rename = "aspects")]
    aspects: Option<BTreeMap<String, u32>>,
    #[serde(rename = "audio")]
    audio: Option<String>,
    #[serde(rename = "category")]
    category: Option<String>,
    #[serde(rename = "comments")]
    comments: Option<String>,
    #[serde(alias = "desc")]
    #[serde(alias = "Desc")]
    description: Option<String>,
    #[serde(rename = "hints")]
    hints: Option<Vec<String>>,
    icon: Option<String>,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "label")]
    label: Option<String>,
    #[serde(rename = "maxnotes")]
    #[serde(alias = "maxNotes")]
    #[serde(alias = "MaxNotes")]
    max_notes: Option<u32>,
    #[serde(default)]
    multiple: bool,
    slot: Option<VerbsSlot>,
    slots: Option<Vec<VerbsSlot>>,
    #[serde(default)]
    #[serde(deserialize_with = "parse_bool")]
    spontaneous: bool,
    xtriggers: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerbsSlot {
    description: Option<String>,
    essential: Option<BTreeMap<String, u32>>,
    forbidden: Option<BTreeMap<String, u32>>,
    id: String,
    label: Option<String>,
    required: Option<BTreeMap<String, u32>>,
}
