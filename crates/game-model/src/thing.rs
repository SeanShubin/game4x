//! What a thing is: a kind, its own traits, and the things it contains.
//!
//! `spec/invariants.md`, promoted 2026-08-31:
//!
//! > A game's state is things, in places, and how many of each. A thing is a set of traits,
//! > and one of them names its kind
//!
//! > Nothing in the state is special to a kind. Adding a kind adds no field and no case, and
//! > whatever reads the state reads it the same way whatever kind it holds
//!
//! **The second sentence is the one that costs something**, and it is what the old shape
//! could not do. A territory had `citizens`, `yards`, `stores`, `garrison` and `extractors`
//! as five separate fields, so `P-192` adding two kinds and `P-206` adding three added
//! nothing the model could use - **adding a kind added a field**, which is exactly what the
//! rule forbids.
//!
//! # A leaf is an observation, not a type
//!
//! [`Thing`] has children and a value, and a thing with no children is a leaf. There is no
//! separate leaf type, because that would be a case in the one place the rule forbids one -
//! and because it would be wrong on this game's own facts: a territory has a biome *and*
//! contains citizens, an extractor has a resource *and* contains its catch. Under
//! *containers have no value of their own* neither can be said.
//!
//! # Parts and contents are one list at different depths
//!
//! `docs/notes/what-a-thing-is.md`, Sean's answer: a tank is a part of a pioneer and the
//! energy is in the tank. Nothing here distinguishes a part from cargo, because the tree
//! already does, by depth.

use std::collections::BTreeMap;

/// The fourteen kinds `releases/first-release.md` declares.
///
/// **The list is here and every other kind-shaped decision is not.** What a kind costs, what
/// it crosses and what it is bounded by are data the game loads, and `S-21`'s second half is
/// where they stop being Rust. This enum is what a trait's `kind` value is drawn from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Citizen,
    Garrison,
    FoodExtractor,
    MetalExtractor,
    EnergyExtractor,
    Yard,
    Ark,
    Pioneer,
    Food,
    Metal,
    Energy,
    Labor,
    Territory,
    Orbit,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Citizen => "citizen",
            Kind::Garrison => "garrison",
            Kind::FoodExtractor => "food extractor",
            Kind::MetalExtractor => "metal extractor",
            Kind::EnergyExtractor => "energy extractor",
            Kind::Yard => "yard",
            Kind::Ark => "ark",
            Kind::Pioneer => "pioneer",
            Kind::Food => "food",
            Kind::Metal => "metal",
            Kind::Energy => "energy",
            Kind::Labor => "labor",
            Kind::Territory => "territory",
            Kind::Orbit => "orbit",
        }
    }

    /// Every kind, so that a reader can name one that is nowhere.
    pub const ALL: [Kind; 14] = [
        Kind::Citizen,
        Kind::Garrison,
        Kind::FoodExtractor,
        Kind::MetalExtractor,
        Kind::EnergyExtractor,
        Kind::Yard,
        Kind::Ark,
        Kind::Pioneer,
        Kind::Food,
        Kind::Metal,
        Kind::Energy,
        Kind::Labor,
        Kind::Territory,
        Kind::Orbit,
    ];

    /// The extractor kind that produces this resource.
    ///
    /// `P-206` split one extractor into three, and this is the only place that split needs
    /// to be known - a resource names its extractor rather than an extractor carrying a
    /// resource field.
    pub fn extractor_for(resource: crate::Resource) -> Kind {
        match resource {
            crate::Resource::Food => Kind::FoodExtractor,
            crate::Resource::Metal => Kind::MetalExtractor,
            crate::Resource::Energy => Kind::EnergyExtractor,
        }
    }

    /// The kind a unit of this resource is.
    ///
    /// A resource is a kind, which is why a store is things rather than a number.
    pub fn from_resource(resource: crate::Resource) -> Kind {
        match resource {
            crate::Resource::Food => Kind::Food,
            crate::Resource::Metal => Kind::Metal,
            crate::Resource::Energy => Kind::Energy,
        }
    }

    /// The resource this kind is, if it is one.
    pub fn resource(self) -> Option<crate::Resource> {
        match self {
            Kind::Food => Some(crate::Resource::Food),
            Kind::Metal => Some(crate::Resource::Metal),
            Kind::Energy => Some(crate::Resource::Energy),
            _ => None,
        }
    }
}

/// What a thing can be distinguished by, beyond its kind.
///
/// **A trait is a slot, not a field per kind.** `Works` is which node an extractor works and
/// means nothing to a citizen; a citizen simply does not carry it. Adding a kind adds no
/// variant here, and adding a trait is what the release's *Traits* table is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trait {
    /// Ready, as a number. Absent means ready, and zero means not.
    ///
    /// **`P-233` renamed the trait and `P-235` says why it is a number.** The release used
    /// to call this `readiness` with values *ready, exhausted*; it is now `ready`, yes or
    /// no. This held `Exhausted` as a presence flag, which was doubly wrong afterwards - the
    /// negative of the trait the release names, and a shape that cannot hold anything but
    /// two states.
    ///
    /// `docs/vision.md` → Directions: *a trait may become a quantity, and nothing should
    /// depend on one having only two values.* A citizen with two actions a turn is that
    /// direction, and it needs a number here rather than a rename later. So this is a count,
    /// read through [`Thing::is_ready`], and every caller asks that rather than the value.
    Ready,
    /// Which of a territory's nodes an extractor works.
    Works,
    /// Force of its own.
    Force,
    /// What a citizen working here produces in force.
    Multiplier,
    /// Citizens working here this turn.
    Manned,
}

/// A thing: its kind, its own traits, and what it contains.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thing {
    pub kind: Kind,
    pub traits: BTreeMap<Trait, u32>,
    pub children: Vec<Thing>,
}

impl Thing {
    /// A thing of this kind with nothing distinguishing it and nothing in it.
    pub fn of(kind: Kind) -> Self {
        Thing {
            kind,
            traits: BTreeMap::new(),
            children: Vec::new(),
        }
    }

    pub fn with(mut self, name: Trait, value: u32) -> Self {
        self.traits.insert(name, value);
        self
    }

    pub fn trait_of(&self, name: Trait) -> Option<u32> {
        self.traits.get(&name).copied()
    }

    pub fn is(&self, name: Trait) -> bool {
        self.traits.contains_key(&name)
    }

    /// Whether this thing can still act.
    ///
    /// **Absent means ready**, so a thing made this turn needs no trait to be usable, and
    /// **nothing outside this method reads the number**. That is what keeps `P-235`'s
    /// direction open: a citizen with two actions is a different value here and no change
    /// anywhere else.
    pub fn is_ready(&self) -> bool {
        self.trait_of(Trait::Ready).unwrap_or(1) > 0
    }

    /// Spend one of whatever readiness this thing has.
    pub fn spend_readiness(&mut self) {
        let left = self.trait_of(Trait::Ready).unwrap_or(1);
        self.set(Trait::Ready, left.saturating_sub(1));
    }

    /// What `refresh` does: everything can act again.
    pub fn refresh(&mut self) {
        self.clear(Trait::Ready);
    }

    pub fn set(&mut self, name: Trait, value: u32) {
        self.traits.insert(name, value);
    }

    pub fn clear(&mut self, name: Trait) {
        self.traits.remove(&name);
    }
}
