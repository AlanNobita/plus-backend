use md5::{Digest, Md5};
use uuid::Uuid;

/// The UUID a vanilla offline-mode client computes for `username` (MD5 of
/// `OfflinePlayer:<name>`, same encoding Mojang's offline mode has always used).
///
/// The launcher's offline accounts share this scheme, so a private backend
/// can recognize them without asking any auth server. There is no way to
/// prove the caller owns the account — a private deployment is the only
/// place this should be trusted.
pub(crate) fn offline_uuid(username: &str) -> Uuid {
	let mut hasher = Md5::new();
	hasher.update(format!("OfflinePlayer:{username}").as_bytes());
	let digest = hasher.finalize();
	let mut bytes = [0u8; 16];
	bytes.copy_from_slice(&digest);
	bytes[6] = (bytes[6] & 0x0f) | 0x30;
	bytes[8] = (bytes[8] & 0x3f) | 0x80;
	Uuid::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
	use super::offline_uuid;

	#[test]
	fn vanilla_offline_uuids_are_stable() {
		// MD5("OfflinePlayer:TestPlayer") with the version/variant bits set,
		// matching the launcher's offline_uuid().
		assert_eq!(
			offline_uuid("TestPlayer"),
			uuid::Uuid::parse_str("bb77495a-a740-3169-a238-69654c8bd2c1").unwrap()
		);
	}
}
