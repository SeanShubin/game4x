//! What a thing is: a kind and its own traits.
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
//! A thing holding nothing is a leaf, and there is no separate leaf type - that would be a case
//! in the one place the rule forbids one, and it would be wrong on this game's own facts: a
//! territory has a biome *and* contains citizens, an extractor has a resource *and* contains its
//! catch. Under *containers have no value of their own* neither can be said.
//!
//! **Containment is not a field on this type, and `C-66` is why.** [`Thing`] carried a list of
//! contained things that nothing in the repository ever wrote. The tree the data file states is
//! built in [`crate::containment`] from `Territory::held` and `Game::units`, and its `Entry` is
//! what carries a description and its contents at once - so the paragraph above is a fact about
//! `Entry`, and this type is the flat record a description is made from. **An unread
//! representation cannot diverge detectably**, which is the rule this file applied to five
//! traits before it applied it here.
//!
//! # Parts and contents are one list at different depths
//!
//! `docs/notes/what-a-thing-is.md`, Sean's answer: a tank is a part of a pioneer and the energy
//! is in the tank. Nothing distinguishes a part from cargo, because the tree already does, by
//! depth. **In this release the tank is not a thing at all** - `fuel` is a trait of the unit,
//! which `containment.rs` records against the release's own words.

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
    /// **`P-334`: two places that share an edge, held by the thing that holds them.**
    ///
    /// It was a trait - `adjacency`, of *a thing that holds places* - and a description is a
    /// flat map while an adjacency is a pair, which is the problem `density` had and the same
    /// answer. `spec/logistics.md` puts it on the container rather than on either place: *a
    /// thing says which of the things in it are next to which. That is a fact about the
    /// container rather than about its contents.*
    Adjacency,
    /// **`P-351`: every thing is in it, and it is the one thing that is in nothing.**
    ///
    /// `spec/logistics.md` needed a thing that is in nothing for containment to be a tree, and
    /// the Kinds table did not declare one - so `containment::tree` wrote the word `game` by
    /// hand and `C-46` reported it as a word the code writes anyway. **This is that word
    /// answered**: the release declares it now, so the root of the tree is a kind like any
    /// other rather than a string this file happens to spell the same way.
    Game,
    /// **A citizen's capacity to raise one more, spent by raising one and renewed each turn.**
    ///
    /// The saturating rewrite's own kind. `grow` consumed *the lesser of the surplus food and
    /// the citizens here*, which is a quantity read from the state; `P-373` says such a rule
    /// is written as a smaller one that fires as many times as it can, and `bear`, `breed` and
    /// `renew` are that rule. Fertility is what they pass between them.
    ///
    /// **It is transient**, which the release says of exactly this and `labor`: neither has a
    /// source and nothing holds either, so both are always in disorder and neither survives
    /// the turn's end. `discard` is what takes it - `P-380`, and without that row a territory
    /// that starved to nobody banked fertility and repopulated from stock the moment food
    /// arrived, which is `C-83`.
    Fertility,
    /// **`P-399`: what a thing spends to act, drawn from time and refilled each turn.**
    ///
    /// **A kind, where it was a yes-or-no trait.** *Where things are* bounds it at one per
    /// thing per action, and that is the token model's whole limit: two recipes naming the
    /// same action draw on the same tokens, so one token is what makes a thing choose.
    ///
    /// **Stored as [`Trait::Ready`] and [`Trait::Spent`] on the thing that holds it, and
    /// written into a data file as a thing it contains.** Those two traits are already a count
    /// per action - `Ready` is the one a citizen spends on labor and an extractor on work,
    /// `Spent` the one a citizen spends on bearing - so the release's four actions map onto
    /// what the model already holds, and only the writing changed.
    Readiness,
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
            Kind::Adjacency => "adjacency",
            Kind::Game => "game",
            Kind::Fertility => "fertility",
            Kind::Readiness => "readiness",
        }
    }

    /// Every kind, so that a reader can name one that is nowhere.
    pub const ALL: [Kind; 18] = [
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
        Kind::Adjacency,
        Kind::Game,
        Kind::Fertility,
        Kind::Readiness,
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
/// **`Manned` came back and has gone again, by Sean's decision on `C-46` - `S-72`.** It was
/// declared by no row of the release's *Traits* table, so the choice was a row or a deletion
/// and he chose the deletion. **It had also never worked**: `Territory::garrison()` returns a
/// copy, so `work`'s `garrison.manned += count` incremented a temporary and every garrison
/// read `manned 0`. The rule this file states caught the field twice and the arithmetic not
/// once, which is the limit of the rule rather than a failure of it.
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
    /// Which place an adjacency runs from. `P-334`.
    From,
    /// Which place an adjacency runs to.
    ///
    /// **The lower id is `from`**, so a symmetric fact is written once - thirty entries for a
    /// tiny planet rather than sixty - and the same state is the same bytes.
    To,
    /// How many extractors this deposit has room for.
    ///
    /// **`P-331` put it beside `density`.** It read *a territory, per kind*, which a
    /// description could not hold - a territory has one per kind and a description is a flat
    /// map. On the deposit there is one of each per deposit, and `C-53`'s half-closed round
    /// trip closes: the release's `6 x 2` is both numbers and the file now carries both.
    TotalCapacity,
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
    /// Whether a citizen has already borne this turn. Absent means fertile.
    ///
    /// **This is what bounds the increase at the number of citizens** - the job `grow`'s
    /// expression used to do with *the lesser of the surplus food and the citizens here*.
    /// `bear` takes a fertile citizen and leaves a spent one, so a citizen cannot bear twice
    /// in one ending; `renew` clears it, once per turn, which is the release's *everything
    /// becomes ready again* applied to bearing rather than to acting.
    ///
    /// **`P-399` made this a readiness `for bearing`**, which is what it always was: a
    /// citizen holds one token for bearing and one for labor, and `bear` spends the first
    /// where `create labor` spends the second. The storage is unchanged; what a data file
    /// calls it is not.
    ///
    /// **That phrase is the release's and no longer the specification's.** `P-390` replaced it
    /// in `spec/turn.md` with time refilling each thing's tokens, and the release has not
    /// followed yet - so the quotation above is accurate today and is quoting the document
    /// that is behind. It goes when the release catches up, along with `renew` itself.
    ///
    /// **Absent means fertile, following [`Trait::Ready`]'s precedent**, so a citizen made
    /// this turn needs no trait to be able to bear and the default state writes nothing into
    /// the data file.
    Spent,
}

/// A thing: its kind and its own traits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thing {
    pub kind: Kind,
    pub traits: BTreeMap<Trait, u32>,
}

impl Thing {
    /// A thing of this kind with nothing distinguishing it.
    pub fn of(kind: Kind) -> Self {
        Thing {
            kind,
            traits: BTreeMap::new(),
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
