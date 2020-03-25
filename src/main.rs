pub struct DropGuard<F>
where
    F: FnOnce(),
{
    func: Option<F>,
}

impl<F> Drop for DropGuard<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        if let Some(func) = self.func.take() {
            func()
        }
    }
}

// See https://docs.rs/finally-block/0.2.0/finally_block/
#[allow(dead_code)]
pub fn drop_guard<F>(func: F) -> DropGuard<F>
where
    F: FnOnce(),
{
    DropGuard { func: Some(func) }
}

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

fn main() -> std::io::Result<()> {
    let some_condition = true;
    let result: &str = bracket::<_, _, std::io::Error, _, _, _>(
        || match some_condition {
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
