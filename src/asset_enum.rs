use std::hash::Hash;
use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::utils::HashMap;

macro_rules! asset_enum_def {
    ($enum:ident, $list:ident, [$(($variant:ident, $path:expr)),+$(,)?] $(, derive($($der:ident),*))?) => {
        #[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
        $(#[derive($($der,)*)])?
        pub enum $enum {
            #[default]
            Undefined,
            $($variant,)*
        }

        impl crate::asset_enum::AssetEnum for $enum {
            fn get_list() -> &'static [(Self, &'static str)] {
                $list
            }
        }

        const $list: &[($enum, &str)] = &[
            $(($enum::$variant, $path),)*
        ];
    };
}

pub(crate) use asset_enum_def;

pub trait AssetEnum: Hash + Clone + Default + Sized + Send + Sync + PartialEq + Eq + 'static {
    fn get_list() -> &'static [(Self, &'static str)];
}

#[derive(Debug, Default, Resource)]
pub struct AssetDictionary<T: Hash, A: Asset>(pub HashMap<T, Handle<A>>);

impl<T: Hash + PartialEq + Eq, A: Asset> AssetDictionary<T, A> {
    pub fn get<'a>(&self, key: &T, assets: &'a Assets<A>) -> Option<&'a A> {
        assets.get(self.0.get(key)?)
    }
    pub fn get_handle(&self, key: &T) -> Option<Handle<A>> {
        self.0.get(key).cloned()
    }
}
    
#[derive(Default)]
pub struct AssetEnumPlugin<T: AssetEnum, A: Asset> {
    _data: PhantomData<(T, A)>
}

impl<T: AssetEnum, A: Asset + Default> Plugin for AssetEnumPlugin<T, A> {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetDictionary<T, A>>()
            .add_systems(Startup, load_asset_enum::<T, A>);
    }
}

pub fn load_asset_enum<T: AssetEnum, A: Asset>(
    mut dict: ResMut<AssetDictionary<T, A>>,
    asset_server: Res<AssetServer>
    ) {
    for (k, path) in T::get_list() {
        dict.0.insert(k.clone(), asset_server.load(*path));
    }
}
