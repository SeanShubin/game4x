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

/// The kinds `releases/first-release.md` declares.
///
/// **The list is here and every other kind-shaped decision is not.** What a kind costs, what
/// it crosses and what it is bounded by are data the game loads, and `S-21`'s second half is
/// where they stop being Rust. This enum is what a trait's `kind` value is drawn from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Citizen,
    Garrison,
    Extractor,
    /// **`P-260`: one kind with a `resource` trait, not three that differ in one word.**
    /// `spec/invariants.md` -> templating gives variety without complexity. A store holds
    /// ten of the resource it was built for, and that ten is a fact about the kind rather
    /// than about any one store - `spec/logistics.md`.
    Store,
    Yard,
    Ark,
    Pioneer,
    Food,
    Metal,
    Energy,
    Labor,
    Territory,
    Orbit,
    /// **`P-322`: what a territory's ground offers of one resource, and how richly.**
    ///
    /// It is a kind because `density` had nowhere else to be written. A description is a flat
    /// map from a trait name to one value and a territory has a density per resource, so
    /// `density` could not be a trait of a territory *and* be written down - `C-46`. As a
    /// thing it can: a territory contains `{deposit resource:food density:4} -> 1`.
    ///
    /// **It is not `Node` coming back.** `S-48` deleted a deposit *per unit of capacity* -
    /// `capacity` copies of one fact, since no territory-resource pair has two densities.
    /// This is one per resource, which is the shape `Territory::deposits` already had.
    Deposit,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Citizen => "citizen",
            Kind::Garrison => "garrison",
            Kind::Extractor => "extractor",
            Kind::Store => "store",
            Kind::Yard => "yard",
            Kind::Ark => "ark",
            Kind::Pioneer => "pioneer",
            Kind::Food => "food",
            Kind::Metal => "metal",
            Kind::Energy => "energy",
            Kind::Labor => "labor",
            Kind::Territory => "territory",
            Kind::Orbit => "orbit",
            Kind::Deposit => "deposit",
        }
    }

    /// Every kind, so that a reader can name one that is nowhere.
    pub const ALL: [Kind; 14] = [
        Kind::Citizen,
        Kind::Garrison,
        Kind::Extractor,
        Kind::Store,
        Kind::Yard,
        Kind::Ark,
        Kind::Pioneer,
        Kind::Food,
        Kind::Metal,
        Kind::Energy,
        Kind::Labor,
        Kind::Territory,
        Kind::Orbit,
        Kind::Deposit,
    ];

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
/// **A trait is a slot, not a field per kind.** A citizen simply does not carry the ones that
/// mean nothing to it. Adding a kind adds no variant here.
///
/// # Only what something reads
///
/// **`Q-45`, and the returning half of it.** These five were once here unread - the rules
/// decided by kind, and `work` reached `garrison.manned` as a named field while
/// `Trait::Manned` sat unused in the map beside it. They were deleted, because *going to be
/// read* is not something a compiler or a test can tell from *dead*, and an unread
/// representation cannot diverge detectably.
///
/// **They are back in the commit that makes a rule read them**, which is what deleting them
/// was for: a garrison and an extractor are things now, and these are what distinguish
/// them. Nothing here is written down against a future.
///
/// The quality lens named why that is worse than ordinary duplication, and the argument is
/// the one that decided this: **every other two-source case in this repository had both
/// sides read, so a divergence eventually showed.** An unread representation cannot diverge
/// detectably. Those five could have said anything at all and no test, no drawing and no
/// command would have differed.
///
/// So they are gone. They were me writing down where this is going as though it were state,
/// and *going to be read* is not a property the compiler or a test can tell from *dead*.
/// Each comes back in the commit that makes a rule read it.
///
/// **`Ready` stays because it is read.** It is what `is_ready`, `spend_readiness` and
/// `refresh` are, it is the release's `ready` trait, and `P-235` wants it able to become a
/// quantity - which it is.
///
/// The migration is finished: `garrison` and `extractors` were the last two typed structs
/// with named fields, and a territory now holds only things.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trait {
    /// Which resource an extractor is built for.
    ///
    /// `P-234` collapsed three extractor kinds back into one with this trait, which is what
    /// the command language always said - `build extractor 1 food` treats the resource as a
    /// parameter.
    Resource,
    /// Force of its own.
    Force,
    /// What a citizen working here produces in force.
    Multiplier,
    /// Citizens working here this turn.
    Manned,
    /// How much one extractor working this deposit yields.
    ///
    /// **`P-322` moved it here from the territory**, and it is read by
    /// `containment::describe` - a trait arrives in the commit that makes a rule read it,
    /// which is what the note above this enum is about.
    Density,
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
