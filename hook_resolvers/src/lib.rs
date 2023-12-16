use patternsleuth::resolvers::futures::future::join_all;
use patternsleuth::resolvers::unreal::*;
use patternsleuth::resolvers::*;
use patternsleuth::scanner::Pattern;

#[derive(Debug)]
pub struct GetServerName(pub usize);
impl_resolver_singleton!(GetServerName, |ctx| async {
    let patterns = [
        "48 89 5C 24 10 48 89 6C 24 18 48 89 74 24 20 57 41 56 41 57 48 83 EC 30 45 33 FF 4C 8B F2 48 8B D9 44 89 7C 24 50 41 8B FF"
    ];

    let res = join_all(patterns.iter().map(|p| ctx.scan(Pattern::new(p).unwrap()))).await;

    // matches two but the first is the one we need
    // could potentially get number of xrefs if this becomes problematic
    Ok(Self(
        res.into_iter()
            .flatten()
            .next()
            .context("expected at least one")?,
    ))
});

#[derive(Debug)]
pub struct Disable(pub usize);
impl_resolver_singleton!(Disable, |ctx| async {
    let patterns = ["4C 8B B4 24 48 01 00 00 0F 84"];

    let res = join_all(patterns.iter().map(|p| ctx.scan(Pattern::new(p).unwrap()))).await;

    Ok(Self(ensure_one(res.into_iter().flatten())?))
});

#[derive(Debug)]
pub struct Resize16(pub usize);
impl_resolver_singleton!(Resize16, |ctx| async {
    let patterns =
        ["48 89 5C 24 ?? 57 48 83 EC 20 48 63 DA 48 8B F9 85 D2 74 ?? 48 8B CB 33 D2 48 03 C9"];

    let res = join_all(patterns.iter().map(|p| ctx.scan(Pattern::new(p).unwrap()))).await;

    Ok(Self(ensure_one(res.into_iter().flatten())?))
});

#[derive(Debug)]
pub struct FMemoryFree(pub usize);
impl_resolver_singleton!(FMemoryFree, |ctx| async {
    let patterns = ["48 85 C9 74 ?? 53 48 83 EC 20 48 8B D9 48 8B 0D"];

    let res = join_all(patterns.iter().map(|p| ctx.scan(Pattern::new(p).unwrap()))).await;

    Ok(Self(ensure_one(res.into_iter().flatten())?))
});

impl_try_collector! {
    #[derive(Debug)]
    pub struct ServerNameResolution {
        pub fmemory_free: FMemoryFree,
        pub resize16: Resize16,
        pub get_server_name: GetServerName,
    }
}

impl_try_collector! {
    #[derive(Debug)]
    pub struct SaveGameResolution {
        pub fmemory_free: FMemoryFree,
        pub save_game_to_memory: UGameplayStaticsSaveGameToMemory,
        pub save_game_to_slot: UGameplayStaticsSaveGameToSlot,
        pub load_game_from_memory: UGameplayStaticsLoadGameFromMemory,
        pub load_game_from_slot: UGameplayStaticsLoadGameFromSlot,
        pub does_save_game_exist: UGameplayStaticsDoesSaveGameExist,
    }
}

impl_collector! {
    #[derive(Debug)]
    pub struct HookResolution {
        pub fmemory_free: FMemoryFree,
        pub disable: Disable,
        pub server_name: ServerNameResolution,
        pub save_game: SaveGameResolution,
    }
}
