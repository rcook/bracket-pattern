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
            Ok("hello world")
        },
    )?;
    println!("result={}", result);
    Ok(())
}

fn main() -> std::io::Result<()> {
    demo_bracket(true)?;
    demo_bracket(false)?;
    Ok(())
}
