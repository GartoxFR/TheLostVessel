use serde::Deserialize;

use crate::asset_enum::asset_enum_def;

asset_enum_def!(Portrait, PORTRAITS, [
    (AI, "texture/ai.png"),
    (Player, "texture/player_icon.png"),
], derive(Deserialize));
