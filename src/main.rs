fn main() {
    // ***** STEP: 5 create world like this
    let mut world = World::new();
    // Icarus's health is *not* looking good.
    world.new_entity(Some(Health(-10)), Some(Name("Icarus"))); 
    // Prometheus is very healthy.
    world.new_entity(Some(Health(100)), Some(Name("Prometheus"))); 
    // Note that Zeus does not have a `Health` component.
    world.new_entity(None, Some(Name("Zeus"))); 
    // *****

    // **** STEP: 6
    let zip = world
        .health_components
        .iter()
        .zip(world.name_components.iter());

    // * filter_map creates an Iterator that skips any values that return None otherwise it will return the contents of the Some().
    // * as_ref remaps our &Option<Health> value be Option<&Health>.
    // * And the ? at the end of health.as_ref()? says "return None if this value is None, otherwise return the inner value."
    let with_health_and_name =
        zip.filter_map(|(health, name): (&Option<Health>, &Option<Name>)| {
            Some((health.as_ref()?, name.as_ref()?))
    });
    // *****

    // ***** STEP: 7
    // The end result is an Iterator that iterates only the components of entities 
    // that have both components, and then returns references to those components.
    for (health, name) in with_health_and_name {
        if health.0 < 0 {
            println!("{} has perished!", name.0);
        } else {
            println!("{} is still healthy", name.0);
        }
    }
    // *****
}


// ***** STEP: 1
//  Health and Name are Components 
//  this holds data and do nothing else
struct Health(i32);
struct Name(&'static str);
// *****

// ***** STEP: 2
// World struct holds all parts of ECS 
struct World {
    health_components: Vec<Option<Health>>,
    name_components: Vec<Option<Name>>,
}
// *****

impl World {
    // ***** STEP: 3
    fn new() -> Self {
        Self {
            health_components: vec![],
            name_components: vec![],
        }
    }
    // *****

    // **** STEP: 4
    fn new_entity(&mut self, health: Option<Health>, name: Option<Name>) {
        self.health_components.push(health);
        self.name_components.push(name);
        // with this system health and name will always have same length
        // entites can simply be looked by index in Components, which will
        // return either None or the Components itself.
    }
    // *****
}
