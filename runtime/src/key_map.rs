use srml_support::{StorageMap, dispatch::Result};

pub trait Trait: system::Trait {}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn set_mapping(_origin, key: u32, value: u32) -> Result {
			<Value<T>>::insert(key, value);
			Ok(())
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as RuntimeExampleStorage {
		Value: map u32 => u32;
	}
}