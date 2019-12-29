//! Commands

/// get current WPA/EAPOL/EAP status
pub const STATUS: &str = "STATUS";

/// get MIB variables (dot1x, dot11)
pub const MIB: &str = "MIB";

/// IEEE 802.1X EAPOL state machine logoff
pub const LOGOFF: &str = "LOGOFF";

/// IEEE 802.1X EAPOL state machine logon
pub const LOGON: &str = "LOGON";

/// show PMKSA cache
pub const PMKSA: &str = "PMKSA";

/// force reassociation
pub const REASSOCIATE: &str = "REASSOCIATE";

/// force wpa_supplicant to re-read its configuration file
pub const RECONFIGURE: &str = "RECONFIGURE";

/// list configured networks
pub const LIST_NETWORKS: &str = "LIST_NETWORKS";

/// terminate wpa_supplicant
pub const TERMINATE: &str = "TERMINATE";
