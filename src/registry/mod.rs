pub mod index;
pub mod local;
pub mod remote;
pub mod traits;

pub use index::{RegistryEntry, RegistryIndex};
pub use local::LocalRegistry;
pub use remote::RemoteRegistry;
pub use traits::RegistryProvider;
