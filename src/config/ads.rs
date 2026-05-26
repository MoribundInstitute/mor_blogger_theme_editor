use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AdsMode {
    #[default]
    None,
    Always,
    OptIn,
    OptOut,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AdBannerPosition {
    #[default]
    BottomSlideUp,
    TopPushDown,
    ModalOverlay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AdSlotPlacement {
    #[default]
    Sidebar,
    BetweenPosts,
    Both,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AdSlot {
    pub slot_id: String,
    pub placement: AdSlotPlacement,
}

impl Default for AdSlot {
    fn default() -> Self {
        Self {
            slot_id: String::new(),
            placement: AdSlotPlacement::Sidebar,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AdsConfig {
    pub mode: AdsMode,
    pub publisher_id: String,
    pub slots: Vec<AdSlot>,
    pub banner_position: AdBannerPosition,
}

impl Default for AdsConfig {
    fn default() -> Self {
        Self {
            mode: AdsMode::None,
            publisher_id: String::new(),
            slots: Vec::new(),
            banner_position: AdBannerPosition::BottomSlideUp,
        }
    }
}

impl AdsConfig {
    pub fn requires_publisher_data(&self) -> bool {
        matches!(self.mode, AdsMode::OptIn | AdsMode::OptOut)
    }

    pub fn warnings(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.requires_publisher_data() {
            if self.publisher_id.trim().is_empty() {
                out.push(
                    "Selected ads mode requires a publisher ID (e.g. ca-pub-1234567890123456). \
                     Without it, ads will not render."
                        .to_string(),
                );
            }
            if self.slots.is_empty() {
                out.push(
                    "Selected ads mode requires at least one ad slot. Add one in the Ads panel."
                        .to_string(),
                );
            } else {
                let empty_count = self
                    .slots
                    .iter()
                    .filter(|s| s.slot_id.trim().is_empty())
                    .count();
                if empty_count > 0 {
                    out.push(format!(
                        "{} ad slot(s) have empty slot IDs. Those slots will not render.",
                        empty_count
                    ));
                }
            }
        }
        out
    }
}
