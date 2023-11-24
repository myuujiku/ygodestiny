use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// A generic card pack containing groups for making rarities and different weights
/// possible and layouts to allow for more varied pack contents.
#[derive(Default, Serialize, Deserialize)]
#[serde(bound = "ID: Serialize, for<'a> ID: Deserialize<'a>")]
pub struct Pack<ID>
where
    ID: Copy + Serialize,
    for<'a> ID: Deserialize<'a>,
{
    pub name: String,
    pub groups: HashMap<u128, PackGroup<ID>>,
    pub main_layout: PackLayout,
    pub extra_layouts: HashMap<String, PackLayout>,
}

/// Group of card IDs with an optional associated rarity.
#[derive(Serialize, Deserialize)]
#[serde(bound = "ID: Serialize, for<'a> ID: Deserialize<'a>")]
pub struct PackGroup<ID>
where
    ID: Copy + Serialize,
    for<'a> ID: Deserialize<'a>,
{
    pub cards: Vec<ID>,
    pub default_rarity: Option<String>,
}

/// The pack's structure composed of a weighted list of possible [`PackVariant`]s that
/// can be generated. For example there could be a 1 in 10 chance to get a pack with
/// 4 rare cards instead of 5 cards that may not contain any rares.
#[derive(Default, Serialize, Deserialize)]
pub struct PackLayout {
    pub variants: Vec<PackVariant>,
    /// Weighting method for choosing a [`PackVariant`].
    pub weighting: Weighting,
}

impl PackLayout {
    /// Selects a [`PackVariant`] based on its weight and returns its result
    /// of [`PackVariant::generate_groups`].
    pub fn generate_groups(&self) -> anyhow::Result<Vec<&PackGroupRef>> {
        self.variants
            .choose_weighted(&mut rand::thread_rng(), |variant| {
                self.weighting.calculate(variant.weight)
            })?
            .generate_groups()
    }
}

/// [`PackSlot`] container with weight used in a [`PackVariant`].
#[derive(Default, Serialize, Deserialize)]
pub struct PackVariant {
    pub weight: u32,
    pub slots: Vec<PackSlot>,
}

impl PackVariant {
    /// Creates a new `PackSlot` with default values.
    pub fn new() -> Self {
        Self {
            weight: 1,
            ..Default::default()
        }
    }

    /// Calls [`PackSlot::generate_groups`] on each slot in `Self`.
    pub fn generate_groups(&self) -> anyhow::Result<Vec<&PackGroupRef>> {
        let mut groups = Vec::with_capacity(
            self.slots
                .iter()
                .fold(0, |res, slot| res + slot.duplications),
        );

        for slot in &self.slots {
            groups.append(&mut slot.generate_groups()?);
        }

        Ok(groups)
    }
}

/// Corresponds to one or more slots in a [`PackVariant`], which generates one card
/// during drafting.
#[derive(Default, Serialize, Deserialize)]
pub struct PackSlot {
    /// Weighting method used to determine which [`PackGroupRef`] to choose with
    /// [`generate_groups`](Self::generate_groups).
    pub weighting: Weighting,
    /// List of [`PackGroupRef`]s that can be generated.
    pub groups: Vec<PackGroupRef>,
    /// Number of copies of this slot during drafting.
    pub duplications: usize,
}

impl PackSlot {
    /// Returns `self.duplications + 1` borrowed [`PackGroupRef`]s based on their
    /// weightings if no error occurs.
    pub fn generate_groups(&self) -> anyhow::Result<Vec<&PackGroupRef>> {
        Ok(self
            .groups
            .choose_multiple_weighted(&mut rand::thread_rng(), self.duplications + 1, |x| {
                self.weighting.calculate(x.weight)
            })?
            .collect())
    }

    /// Returns `self.duplications + 1` borrowed [`PackGroupRef`]s without applying
    /// weighting.
    pub fn generate_groups_no_weighting(&self) -> Vec<&PackGroupRef> {
        self.groups
            .choose_multiple(&mut rand::thread_rng(), self.duplications + 1)
            .collect()
    }
}

/// Reference to a [`PackGroup`]. Does not contain an actual rust reference
/// to the [`PackGroup`] but its uuid in the [`Pack`].
#[derive(Serialize, Deserialize)]
pub struct PackGroupRef {
    /// uuid of the [`PackGroup`]
    pub group_uuid: u128,
    /// weight in the [`PackSlot`]
    pub weight: u32,
    /// if set overrides the rarity defined in the [`PackGroup`]
    pub override_rarity: Option<String>,
}

impl PackGroupRef {
    /// Returns a random [`GeneratedCard`] if the uuid is valid and a
    /// [`RefError`] otherwise.
    pub fn generate<ID>(&self, pack: &Pack<ID>) -> Result<GeneratedCard<ID>, RefError>
    where
        ID: Copy + Serialize,
        for<'de> ID: Deserialize<'de>,
    {
        match pack.groups.get(&self.group_uuid) {
            Some(group) => Ok({
                match group.cards.choose(&mut rand::thread_rng()) {
                    Some(id) => GeneratedCard {
                        id: *id,
                        rarity: if self.override_rarity.is_some() {
                            self.override_rarity.clone()
                        } else {
                            group.default_rarity.clone()
                        },
                    },
                    None => Err(RefError::new(self.group_uuid, RefErrorKind::Empty))?,
                }
            }),
            None => Err(RefError::new(self.group_uuid, RefErrorKind::NotFound)),
        }
    }
}

/// A card generated by a `PackGroupRef`.
pub struct GeneratedCard<ID: Copy> {
    pub id: ID,
    pub rarity: Option<String>,
}

/// Method of weighted randomness.
#[derive(Default, Serialize, Deserialize)]
pub enum Weighting {
    /// Items with a higher weight are more likely to be choosen.
    #[default]
    Accumulate,
    /// Applies the multiplicative inverse to each weight, making items
    /// with a higher weight *less* likely to be choosen.
    Divide,
}

impl Weighting {
    /// Takes a `u32` `weight` and returns an `f64` based on the variant of `self`.
    pub fn calculate(&self, weight: u32) -> f64 {
        match self {
            Weighting::Accumulate => weight.into(),
            Weighting::Divide => 1.0 / weight as f64,
        }
    }
}

/// An error caused by a [`PackGroupRef`].
#[derive(Debug)]
pub struct RefError {
    uuid: u128,
    kind: RefErrorKind,
}

impl RefError {
    /// Creates a new `RefError` from a uuid and a [`RefErrorKind`].
    pub fn new(uuid: u128, kind: RefErrorKind) -> Self {
        Self { uuid, kind }
    }
}

impl Error for RefError {}

impl fmt::Display for RefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            RefErrorKind::NotFound => write!(f, "Referenced group `{}` does not exist.", self.uuid),
            RefErrorKind::Empty => write!(f, "Referenced group `{}` is empty", self.uuid),
        }
    }
}

/// Type of the [`RefError`].
#[derive(Debug)]
pub enum RefErrorKind {
    /// The referenced [`PackGroup`] does not exist.
    NotFound,
    /// The [`PackGroup`] does not contain any cards.
    Empty,
}
