/// See https://wiki.haskell.org/Bracket_pattern
/// # Arguments
///
/// * `R` - type of resource
/// * `T` - type of return value from computation
/// * `E` - type of error
/// * `acquire` - possibly failing function that acquires resource
/// * `release` - function that releases resource
/// * `consume` - possibly failing function that consumes resource and yields return value of type `T`
#[allow(dead_code)]
pub fn bracket<R, T, E, F, G, H>(acquire: F, release: G, consume: H) -> std::result::Result<T, E>
where
    F: FnOnce() -> std::result::Result<R, E>,
    G: FnOnce(R) -> (),
    H: FnOnce(&R) -> std::result::Result<T, E>,
{
    let resource = acquire()?;
    let result = consume(&resource);
    release(resource);
    result
}

struct Resource {}

impl Resource {
    fn new() -> Self {
        println!("Resource acquired");
        Self {}
    }

    fn release(&self) {
        println!("Resource released");
    }
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
fn demo_resource_leak(acquire_resource: bool) -> std::io::Result<()> {
    let resource = match acquire_resource {
        true => Some(Resource::new()),
        false => None,
    };

    println!("Resource used");
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Leak the resource!",
    ));

    match resource {
        Some(r) => r.release(),
        None => {}
    };

    Ok(())
}

#[allow(unreachable_code)]
fn demo_drop(acquire_resource: bool) -> std::io::Result<()> {
    struct ResourceHolder {
        resource: Resource,
    }

    impl Drop for ResourceHolder {
        fn drop(&mut self) {
            self.resource.release()
        }
    }

    let resource = match acquire_resource {
        true => Some(Resource::new()),
        false => None,
    };
    let _holder = resource.map(|r| ResourceHolder { resource: r });

    println!("Resource used");
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Resource does not leak!",
    ));

    Ok(())
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
fn demo_explicit_drop(acquire_resource: bool) -> std::io::Result<()> {
    struct ResourceHolder {
        resource: Resource,
    }

    impl Drop for ResourceHolder {
        fn drop(&mut self) {
            self.resource.release()
        }
    }

    let resource = match acquire_resource {
        true => Some(Resource::new()),
        false => None,
    };
    let holder = resource.map(|r| ResourceHolder { resource: r });

    println!("Resource used");
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Resource does not leak!",
    ));

    drop(holder);

    Ok(())
}

fn demo_bracket(acquire_resource: bool) -> std::io::Result<()> {
    let result: &str = bracket::<_, _, std::io::Error, _, _, _>(
        || match acquire_resource {
            true => Ok(Some(Resource::new())),
            false => Ok(None),
        },
        |resource| match resource {
            Some(r) => r.release(),
            None => {}
        },
        |_| {
            println!("Resource used");
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Resource does not leak!",
            ))
        },
    )?;
    println!("result={}", result);
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("*** demo_resource_leak(true) ***");
    let _ = demo_resource_leak(true);
    println!("*** demo_resource_leak(false) ***");
    let _ = demo_resource_leak(false);
    println!("*** demo_drop(true) ***");
    let _ = demo_drop(true);
    println!("*** demo_drop(false) ***");
    let _ = demo_drop(false);
    println!("*** demo_bracket(true) ***");
    let _ = demo_bracket(true);
    println!("*** demo_bracket(false) ***");
    let _ = demo_bracket(false);

    println!("*** demo_explicit_drop(true) ***");
    let _ = demo_explicit_drop(true);
    println!("*** demo_explicit_drop(false) ***");
    let _ = demo_explicit_drop(false);
    Ok(())
}
