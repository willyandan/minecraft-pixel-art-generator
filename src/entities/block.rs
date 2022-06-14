use super::{errors::invalid_json::InvalidJson, item::Item, rgb::Rgb};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub item: Item,
    pub color: Rgb,
}

impl Block {
    pub fn initialize() -> Result<Vec<Block>, InvalidJson> {
        let blocks: Result<Vec<Block>, serde_json::Error> = serde_json::from_str(
            "
            [
            {
                \"item\": {
                    \"id\": 1,
                    \"name\": \"stone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 125,
                    \"green\": 125,
                    \"blue\": 125
                }
            },
            {
                \"item\": {
                    \"id\": 2,
                    \"name\": \"granite\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 149,
                    \"green\": 103,
                    \"blue\": 85
                }
            },
            {
                \"item\": {
                    \"id\": 3,
                    \"name\": \"polished_granite\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 154,
                    \"green\": 106,
                    \"blue\": 89
                }
            },
            {
                \"item\": {
                    \"id\": 4,
                    \"name\": \"diorite\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 188,
                    \"green\": 188,
                    \"blue\": 188
                }
            },
            {
                \"item\": {
                    \"id\": 5,
                    \"name\": \"polished_diorite\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 192,
                    \"green\": 193,
                    \"blue\": 194
                }
            },
            {
                \"item\": {
                    \"id\": 6,
                    \"name\": \"andesite\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 136,
                    \"green\": 136,
                    \"blue\": 136
                }
            },
            {
                \"item\": {
                    \"id\": 7,
                    \"name\": \"polished_andesite\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 132,
                    \"green\": 134,
                    \"blue\": 133
                }
            },
            {
                \"item\": {
                    \"id\": 8,
                    \"name\": \"grass_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 126,
                    \"green\": 107,
                    \"blue\": 65
                }
            },
            {
                \"item\": {
                    \"id\": 9,
                    \"name\": \"dirt\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 134,
                    \"green\": 96,
                    \"blue\": 67
                }
            },
            {
                \"item\": {
                    \"id\": 10,
                    \"name\": \"coarse_dirt\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 119,
                    \"green\": 85,
                    \"blue\": 59
                }
            },
            {
                \"item\": {
                    \"id\": 11,
                    \"name\": \"podzol\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 122,
                    \"green\": 87,
                    \"blue\": 57
                }
            },
            {
                \"item\": {
                    \"id\": 12,
                    \"name\": \"cobblestone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 127,
                    \"green\": 127,
                    \"blue\": 127
                }
            },
            {
                \"item\": {
                    \"id\": 13,
                    \"name\": \"oak_planks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 162,
                    \"green\": 130,
                    \"blue\": 78
                }
            },
            {
                \"item\": {
                    \"id\": 14,
                    \"name\": \"spruce_planks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 114,
                    \"green\": 84,
                    \"blue\": 48
                }
            },
            {
                \"item\": {
                    \"id\": 15,
                    \"name\": \"birch_planks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 192,
                    \"green\": 175,
                    \"blue\": 121
                }
            },
            {
                \"item\": {
                    \"id\": 16,
                    \"name\": \"jungle_planks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 160,
                    \"green\": 115,
                    \"blue\": 80
                }
            },
            {
                \"item\": {
                    \"id\": 17,
                    \"name\": \"acacia_planks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 168,
                    \"green\": 90,
                    \"blue\": 50
                }
            },
            {
                \"item\": {
                    \"id\": 18,
                    \"name\": \"dark_oak_planks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 66,
                    \"green\": 43,
                    \"blue\": 20
                }
            },
            {
                \"item\": {
                    \"id\": 25,
                    \"name\": \"bedrock\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 85,
                    \"green\": 85,
                    \"blue\": 85
                }
            },
            {
                \"item\": {
                    \"id\": 28,
                    \"name\": \"sand\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 219,
                    \"green\": 207,
                    \"blue\": 163
                }
            },
            {
                \"item\": {
                    \"id\": 29,
                    \"name\": \"red_sand\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 190,
                    \"green\": 102,
                    \"blue\": 33
                }
            },
            {
                \"item\": {
                    \"id\": 30,
                    \"name\": \"gravel\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 131,
                    \"green\": 127,
                    \"blue\": 126
                }
            },
            {
                \"item\": {
                    \"id\": 31,
                    \"name\": \"gold_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 145,
                    \"green\": 133,
                    \"blue\": 106
                }
            },
            {
                \"item\": {
                    \"id\": 32,
                    \"name\": \"deepslate_gold_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 115,
                    \"green\": 102,
                    \"blue\": 78
                }
            },
            {
                \"item\": {
                    \"id\": 33,
                    \"name\": \"iron_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 136,
                    \"green\": 129,
                    \"blue\": 122
                }
            },
            {
                \"item\": {
                    \"id\": 34,
                    \"name\": \"deepslate_iron_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 106,
                    \"green\": 99,
                    \"blue\": 94
                }
            },
            {
                \"item\": {
                    \"id\": 35,
                    \"name\": \"coal_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 105,
                    \"green\": 105,
                    \"blue\": 105
                }
            },
            {
                \"item\": {
                    \"id\": 36,
                    \"name\": \"deepslate_coal_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 74,
                    \"green\": 74,
                    \"blue\": 76
                }
            },
            {
                \"item\": {
                    \"id\": 37,
                    \"name\": \"nether_gold_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 115,
                    \"green\": 54,
                    \"blue\": 42
                }
            },
            {
                \"item\": {
                    \"id\": 38,
                    \"name\": \"oak_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 109,
                    \"green\": 85,
                    \"blue\": 50
                }
            },
            {
                \"item\": {
                    \"id\": 39,
                    \"name\": \"spruce_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 58,
                    \"green\": 37,
                    \"blue\": 16
                }
            },
            {
                \"item\": {
                    \"id\": 40,
                    \"name\": \"birch_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 216,
                    \"green\": 215,
                    \"blue\": 210
                }
            },
            {
                \"item\": {
                    \"id\": 41,
                    \"name\": \"jungle_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 85,
                    \"green\": 67,
                    \"blue\": 25
                }
            },
            {
                \"item\": {
                    \"id\": 42,
                    \"name\": \"acacia_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 103,
                    \"green\": 96,
                    \"blue\": 86
                }
            },
            {
                \"item\": {
                    \"id\": 43,
                    \"name\": \"dark_oak_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 60,
                    \"green\": 46,
                    \"blue\": 26
                }
            },
            {
                \"item\": {
                    \"id\": 44,
                    \"name\": \"stripped_spruce_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 115,
                    \"green\": 89,
                    \"blue\": 52
                }
            },
            {
                \"item\": {
                    \"id\": 45,
                    \"name\": \"stripped_birch_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 196,
                    \"green\": 176,
                    \"blue\": 118
                }
            },
            {
                \"item\": {
                    \"id\": 46,
                    \"name\": \"stripped_jungle_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 171,
                    \"green\": 132,
                    \"blue\": 84
                }
            },
            {
                \"item\": {
                    \"id\": 47,
                    \"name\": \"stripped_acacia_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 174,
                    \"green\": 92,
                    \"blue\": 59
                }
            },
            {
                \"item\": {
                    \"id\": 48,
                    \"name\": \"stripped_dark_oak_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 72,
                    \"green\": 56,
                    \"blue\": 36
                }
            },
            {
                \"item\": {
                    \"id\": 49,
                    \"name\": \"stripped_oak_log\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 177,
                    \"green\": 144,
                    \"blue\": 86
                }
            },
            {
                \"item\": {
                    \"id\": 70,
                    \"name\": \"sponge\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 195,
                    \"green\": 192,
                    \"blue\": 74
                }
            },
            {
                \"item\": {
                    \"id\": 71,
                    \"name\": \"wet_sponge\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 171,
                    \"green\": 181,
                    \"blue\": 70
                }
            },
            {
                \"item\": {
                    \"id\": 73,
                    \"name\": \"lapis_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 107,
                    \"green\": 117,
                    \"blue\": 141
                }
            },
            {
                \"item\": {
                    \"id\": 74,
                    \"name\": \"deepslate_lapis_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 79,
                    \"green\": 90,
                    \"blue\": 115
                }
            },
            {
                \"item\": {
                    \"id\": 75,
                    \"name\": \"lapis_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 30,
                    \"green\": 67,
                    \"blue\": 140
                }
            },
            {
                \"item\": {
                    \"id\": 77,
                    \"name\": \"sandstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 216,
                    \"green\": 203,
                    \"blue\": 155
                }
            },
            {
                \"item\": {
                    \"id\": 78,
                    \"name\": \"chiseled_sandstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 216,
                    \"green\": 202,
                    \"blue\": 155
                }
            },
            {
                \"item\": {
                    \"id\": 79,
                    \"name\": \"cut_sandstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 217,
                    \"green\": 206,
                    \"blue\": 159
                }
            },
            {
                \"item\": {
                    \"id\": 80,
                    \"name\": \"note_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 88,
                    \"green\": 58,
                    \"blue\": 40
                }
            },
            {
                \"item\": {
                    \"id\": 106,
                    \"name\": \"piston\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 109,
                    \"green\": 104,
                    \"blue\": 96
                }
            },
            {
                \"item\": {
                    \"id\": 108,
                    \"name\": \"white_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 233,
                    \"green\": 236,
                    \"blue\": 236
                }
            },
            {
                \"item\": {
                    \"id\": 109,
                    \"name\": \"orange_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 240,
                    \"green\": 118,
                    \"blue\": 19
                }
            },
            {
                \"item\": {
                    \"id\": 110,
                    \"name\": \"magenta_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 189,
                    \"green\": 68,
                    \"blue\": 179
                }
            },
            {
                \"item\": {
                    \"id\": 111,
                    \"name\": \"light_blue_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 58,
                    \"green\": 175,
                    \"blue\": 217
                }
            },
            {
                \"item\": {
                    \"id\": 112,
                    \"name\": \"yellow_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 248,
                    \"green\": 197,
                    \"blue\": 39
                }
            },
            {
                \"item\": {
                    \"id\": 113,
                    \"name\": \"lime_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 112,
                    \"green\": 185,
                    \"blue\": 25
                }
            },
            {
                \"item\": {
                    \"id\": 114,
                    \"name\": \"pink_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 237,
                    \"green\": 141,
                    \"blue\": 172
                }
            },
            {
                \"item\": {
                    \"id\": 115,
                    \"name\": \"gray_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 62,
                    \"green\": 68,
                    \"blue\": 71
                }
            },
            {
                \"item\": {
                    \"id\": 116,
                    \"name\": \"light_gray_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 142,
                    \"green\": 142,
                    \"blue\": 134
                }
            },
            {
                \"item\": {
                    \"id\": 117,
                    \"name\": \"cyan_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 21,
                    \"green\": 137,
                    \"blue\": 145
                }
            },
            {
                \"item\": {
                    \"id\": 118,
                    \"name\": \"purple_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 121,
                    \"green\": 42,
                    \"blue\": 172
                }
            },
            {
                \"item\": {
                    \"id\": 119,
                    \"name\": \"blue_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 53,
                    \"green\": 57,
                    \"blue\": 157
                }
            },
            {
                \"item\": {
                    \"id\": 120,
                    \"name\": \"brown_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 114,
                    \"green\": 71,
                    \"blue\": 40
                }
            },
            {
                \"item\": {
                    \"id\": 121,
                    \"name\": \"green_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 84,
                    \"green\": 109,
                    \"blue\": 27
                }
            },
            {
                \"item\": {
                    \"id\": 122,
                    \"name\": \"red_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 160,
                    \"green\": 39,
                    \"blue\": 34
                }
            },
            {
                \"item\": {
                    \"id\": 123,
                    \"name\": \"black_wool\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 20,
                    \"green\": 21,
                    \"blue\": 25
                }
            },
            {
                \"item\": {
                    \"id\": 140,
                    \"name\": \"gold_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 246,
                    \"green\": 208,
                    \"blue\": 61
                }
            },
            {
                \"item\": {
                    \"id\": 141,
                    \"name\": \"iron_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 220,
                    \"green\": 220,
                    \"blue\": 220
                }
            },
            {
                \"item\": {
                    \"id\": 142,
                    \"name\": \"bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 150,
                    \"green\": 97,
                    \"blue\": 83
                }
            },
            {
                \"item\": {
                    \"id\": 143,
                    \"name\": \"tnt\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 182,
                    \"green\": 88,
                    \"blue\": 84
                }
            },
            {
                \"item\": {
                    \"id\": 144,
                    \"name\": \"bookshelf\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 117,
                    \"green\": 94,
                    \"blue\": 59
                }
            },
            {
                \"item\": {
                    \"id\": 145,
                    \"name\": \"mossy_cobblestone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 110,
                    \"green\": 118,
                    \"blue\": 94
                }
            },
            {
                \"item\": {
                    \"id\": 146,
                    \"name\": \"obsidian\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 15,
                    \"green\": 10,
                    \"blue\": 24
                }
            },
            {
                \"item\": {
                    \"id\": 155,
                    \"name\": \"diamond_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 121,
                    \"green\": 141,
                    \"blue\": 140
                }
            },
            {
                \"item\": {
                    \"id\": 156,
                    \"name\": \"deepslate_diamond_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 83,
                    \"green\": 106,
                    \"blue\": 106
                }
            },
            {
                \"item\": {
                    \"id\": 157,
                    \"name\": \"diamond_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 98,
                    \"green\": 237,
                    \"blue\": 228
                }
            },
            {
                \"item\": {
                    \"id\": 158,
                    \"name\": \"crafting_table\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 128,
                    \"green\": 102,
                    \"blue\": 63
                }
            },
            {
                \"item\": {
                    \"id\": 160,
                    \"name\": \"farmland\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 143,
                    \"green\": 102,
                    \"blue\": 70
                }
            },
            {
                \"item\": {
                    \"id\": 161,
                    \"name\": \"furnace\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 120,
                    \"green\": 120,
                    \"blue\": 120
                }
            },
            {
                \"item\": {
                    \"id\": 187,
                    \"name\": \"redstone_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 140,
                    \"green\": 109,
                    \"blue\": 109
                }
            },
            {
                \"item\": {
                    \"id\": 188,
                    \"name\": \"deepslate_redstone_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 104,
                    \"green\": 73,
                    \"blue\": 74
                }
            },
            {
                \"item\": {
                    \"id\": 196,
                    \"name\": \"clay\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 160,
                    \"green\": 166,
                    \"blue\": 179
                }
            },
            {
                \"item\": {
                    \"id\": 198,
                    \"name\": \"jukebox\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 88,
                    \"green\": 58,
                    \"blue\": 40
                }
            },
            {
                \"item\": {
                    \"id\": 200,
                    \"name\": \"pumpkin\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 195,
                    \"green\": 114,
                    \"blue\": 24
                }
            },
            {
                \"item\": {
                    \"id\": 201,
                    \"name\": \"netherrack\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 97,
                    \"green\": 38,
                    \"blue\": 38
                }
            },
            {
                \"item\": {
                    \"id\": 202,
                    \"name\": \"soul_sand\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 81,
                    \"green\": 62,
                    \"blue\": 50
                }
            },
            {
                \"item\": {
                    \"id\": 203,
                    \"name\": \"soul_soil\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 75,
                    \"green\": 57,
                    \"blue\": 46
                }
            },
            {
                \"item\": {
                    \"id\": 204,
                    \"name\": \"basalt\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 73,
                    \"green\": 72,
                    \"blue\": 77
                }
            },
            {
                \"item\": {
                    \"id\": 205,
                    \"name\": \"polished_basalt\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 88,
                    \"green\": 88,
                    \"blue\": 91
                }
            },
            {
                \"item\": {
                    \"id\": 208,
                    \"name\": \"glowstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 171,
                    \"green\": 131,
                    \"blue\": 84
                }
            },
            {
                \"item\": {
                    \"id\": 210,
                    \"name\": \"carved_pumpkin\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 150,
                    \"green\": 84,
                    \"blue\": 17
                }
            },
            {
                \"item\": {
                    \"id\": 211,
                    \"name\": \"jack_o_lantern\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 214,
                    \"green\": 152,
                    \"blue\": 52
                }
            },
            {
                \"item\": {
                    \"id\": 236,
                    \"name\": \"stone_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 122,
                    \"green\": 121,
                    \"blue\": 122
                }
            },
            {
                \"item\": {
                    \"id\": 237,
                    \"name\": \"mossy_stone_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 115,
                    \"green\": 121,
                    \"blue\": 105
                }
            },
            {
                \"item\": {
                    \"id\": 238,
                    \"name\": \"cracked_stone_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 118,
                    \"green\": 117,
                    \"blue\": 118
                }
            },
            {
                \"item\": {
                    \"id\": 239,
                    \"name\": \"chiseled_stone_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 119,
                    \"green\": 118,
                    \"blue\": 119
                }
            },
            {
                \"item\": {
                    \"id\": 246,
                    \"name\": \"brown_mushroom_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 149,
                    \"green\": 111,
                    \"blue\": 81
                }
            },
            {
                \"item\": {
                    \"id\": 247,
                    \"name\": \"red_mushroom_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 200,
                    \"green\": 46,
                    \"blue\": 45
                }
            },
            {
                \"item\": {
                    \"id\": 248,
                    \"name\": \"mushroom_stem\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 203,
                    \"green\": 196,
                    \"blue\": 185
                }
            },
            {
                \"item\": {
                    \"id\": 252,
                    \"name\": \"melon\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 114,
                    \"green\": 146,
                    \"blue\": 30
                }
            },
            {
                \"item\": {
                    \"id\": 262,
                    \"name\": \"mycelium\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 113,
                    \"green\": 87,
                    \"blue\": 71
                }
            },
            {
                \"item\": {
                    \"id\": 264,
                    \"name\": \"nether_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 44,
                    \"green\": 21,
                    \"blue\": 26
                }
            },
            {
                \"item\": {
                    \"id\": 276,
                    \"name\": \"end_stone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 219,
                    \"green\": 222,
                    \"blue\": 158
                }
            },
            {
                \"item\": {
                    \"id\": 278,
                    \"name\": \"redstone_lamp\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 95,
                    \"green\": 54,
                    \"blue\": 30
                }
            },
            {
                \"item\": {
                    \"id\": 281,
                    \"name\": \"emerald_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 108,
                    \"green\": 136,
                    \"blue\": 115
                }
            },
            {
                \"item\": {
                    \"id\": 282,
                    \"name\": \"deepslate_emerald_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 78,
                    \"green\": 104,
                    \"blue\": 87
                }
            },
            {
                \"item\": {
                    \"id\": 286,
                    \"name\": \"emerald_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 42,
                    \"green\": 203,
                    \"blue\": 87
                }
            },
            {
                \"item\": {
                    \"id\": 290,
                    \"name\": \"command_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 177,
                    \"green\": 133,
                    \"blue\": 107
                }
            },
            {
                \"item\": {
                    \"id\": 339,
                    \"name\": \"anvil\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 68,
                    \"green\": 68,
                    \"blue\": 68
                }
            },
            {
                \"item\": {
                    \"id\": 346,
                    \"name\": \"daylight_detector\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 66,
                    \"green\": 55,
                    \"blue\": 35
                }
            },
            {
                \"item\": {
                    \"id\": 347,
                    \"name\": \"redstone_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 175,
                    \"green\": 24,
                    \"blue\": 5
                }
            },
            {
                \"item\": {
                    \"id\": 348,
                    \"name\": \"nether_quartz_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 117,
                    \"green\": 65,
                    \"blue\": 62
                }
            },
            {
                \"item\": {
                    \"id\": 350,
                    \"name\": \"quartz_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 235,
                    \"green\": 229,
                    \"blue\": 222
                }
            },
            {
                \"item\": {
                    \"id\": 351,
                    \"name\": \"chiseled_quartz_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 231,
                    \"green\": 226,
                    \"blue\": 218
                }
            },
            {
                \"item\": {
                    \"id\": 352,
                    \"name\": \"quartz_pillar\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 235,
                    \"green\": 230,
                    \"blue\": 224
                }
            },
            {
                \"item\": {
                    \"id\": 356,
                    \"name\": \"white_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 209,
                    \"green\": 178,
                    \"blue\": 161
                }
            },
            {
                \"item\": {
                    \"id\": 357,
                    \"name\": \"orange_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 161,
                    \"green\": 83,
                    \"blue\": 37
                }
            },
            {
                \"item\": {
                    \"id\": 358,
                    \"name\": \"magenta_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 149,
                    \"green\": 88,
                    \"blue\": 108
                }
            },
            {
                \"item\": {
                    \"id\": 359,
                    \"name\": \"light_blue_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 113,
                    \"green\": 108,
                    \"blue\": 137
                }
            },
            {
                \"item\": {
                    \"id\": 360,
                    \"name\": \"yellow_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 186,
                    \"green\": 133,
                    \"blue\": 35
                }
            },
            {
                \"item\": {
                    \"id\": 361,
                    \"name\": \"lime_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 103,
                    \"green\": 117,
                    \"blue\": 52
                }
            },
            {
                \"item\": {
                    \"id\": 362,
                    \"name\": \"pink_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 161,
                    \"green\": 78,
                    \"blue\": 78
                }
            },
            {
                \"item\": {
                    \"id\": 363,
                    \"name\": \"gray_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 57,
                    \"green\": 42,
                    \"blue\": 35
                }
            },
            {
                \"item\": {
                    \"id\": 364,
                    \"name\": \"light_gray_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 135,
                    \"green\": 106,
                    \"blue\": 97
                }
            },
            {
                \"item\": {
                    \"id\": 365,
                    \"name\": \"cyan_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 86,
                    \"green\": 91,
                    \"blue\": 91
                }
            },
            {
                \"item\": {
                    \"id\": 366,
                    \"name\": \"purple_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 118,
                    \"green\": 70,
                    \"blue\": 86
                }
            },
            {
                \"item\": {
                    \"id\": 367,
                    \"name\": \"blue_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 74,
                    \"green\": 59,
                    \"blue\": 91
                }
            },
            {
                \"item\": {
                    \"id\": 368,
                    \"name\": \"brown_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 77,
                    \"green\": 51,
                    \"blue\": 35
                }
            },
            {
                \"item\": {
                    \"id\": 369,
                    \"name\": \"green_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 76,
                    \"green\": 83,
                    \"blue\": 42
                }
            },
            {
                \"item\": {
                    \"id\": 370,
                    \"name\": \"red_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 143,
                    \"green\": 61,
                    \"blue\": 46
                }
            },
            {
                \"item\": {
                    \"id\": 371,
                    \"name\": \"black_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 37,
                    \"green\": 22,
                    \"blue\": 16
                }
            },
            {
                \"item\": {
                    \"id\": 394,
                    \"name\": \"prismarine\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 99,
                    \"green\": 156,
                    \"blue\": 151
                }
            },
            {
                \"item\": {
                    \"id\": 395,
                    \"name\": \"prismarine_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 99,
                    \"green\": 171,
                    \"blue\": 158
                }
            },
            {
                \"item\": {
                    \"id\": 396,
                    \"name\": \"dark_prismarine\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 51,
                    \"green\": 91,
                    \"blue\": 75
                }
            },
            {
                \"item\": {
                    \"id\": 403,
                    \"name\": \"sea_lantern\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 172,
                    \"green\": 199,
                    \"blue\": 190
                }
            },
            {
                \"item\": {
                    \"id\": 404,
                    \"name\": \"hay_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 166,
                    \"green\": 136,
                    \"blue\": 38
                }
            },
            {
                \"item\": {
                    \"id\": 421,
                    \"name\": \"terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 152,
                    \"green\": 94,
                    \"blue\": 67
                }
            },
            {
                \"item\": {
                    \"id\": 422,
                    \"name\": \"coal_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 16,
                    \"green\": 15,
                    \"blue\": 15
                }
            },
            {
                \"item\": {
                    \"id\": 423,
                    \"name\": \"packed_ice\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 141,
                    \"green\": 180,
                    \"blue\": 250
                }
            },
            {
                \"item\": {
                    \"id\": 462,
                    \"name\": \"red_sandstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 186,
                    \"green\": 99,
                    \"blue\": 29
                }
            },
            {
                \"item\": {
                    \"id\": 463,
                    \"name\": \"chiseled_red_sandstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 183,
                    \"green\": 96,
                    \"blue\": 27
                }
            },
            {
                \"item\": {
                    \"id\": 464,
                    \"name\": \"cut_red_sandstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 189,
                    \"green\": 101,
                    \"blue\": 31
                }
            },
            {
                \"item\": {
                    \"id\": 485,
                    \"name\": \"smooth_stone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 158,
                    \"green\": 158,
                    \"blue\": 158
                }
            },
            {
                \"item\": {
                    \"id\": 507,
                    \"name\": \"purpur_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 169,
                    \"green\": 125,
                    \"blue\": 169
                }
            },
            {
                \"item\": {
                    \"id\": 508,
                    \"name\": \"purpur_pillar\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 171,
                    \"green\": 129,
                    \"blue\": 171
                }
            },
            {
                \"item\": {
                    \"id\": 510,
                    \"name\": \"end_stone_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 218,
                    \"green\": 224,
                    \"blue\": 162
                }
            },
            {
                \"item\": {
                    \"id\": 514,
                    \"name\": \"repeating_command_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 128,
                    \"green\": 110,
                    \"blue\": 170
                }
            },
            {
                \"item\": {
                    \"id\": 515,
                    \"name\": \"chain_command_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 131,
                    \"green\": 161,
                    \"blue\": 147
                }
            },
            {
                \"item\": {
                    \"id\": 518,
                    \"name\": \"nether_wart_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 114,
                    \"green\": 2,
                    \"blue\": 2
                }
            },
            {
                \"item\": {
                    \"id\": 519,
                    \"name\": \"red_nether_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 69,
                    \"green\": 7,
                    \"blue\": 9
                }
            },
            {
                \"item\": {
                    \"id\": 520,
                    \"name\": \"bone_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 229,
                    \"green\": 225,
                    \"blue\": 207
                }
            },
            {
                \"item\": {
                    \"id\": 522,
                    \"name\": \"observer\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 70,
                    \"green\": 68,
                    \"blue\": 68
                }
            },
            {
                \"item\": {
                    \"id\": 540,
                    \"name\": \"white_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 188,
                    \"green\": 212,
                    \"blue\": 202
                }
            },
            {
                \"item\": {
                    \"id\": 541,
                    \"name\": \"orange_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 154,
                    \"green\": 147,
                    \"blue\": 91
                }
            },
            {
                \"item\": {
                    \"id\": 542,
                    \"name\": \"magenta_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 208,
                    \"green\": 100,
                    \"blue\": 191
                }
            },
            {
                \"item\": {
                    \"id\": 543,
                    \"name\": \"light_blue_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 94,
                    \"green\": 164,
                    \"blue\": 208
                }
            },
            {
                \"item\": {
                    \"id\": 544,
                    \"name\": \"yellow_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 234,
                    \"green\": 192,
                    \"blue\": 88
                }
            },
            {
                \"item\": {
                    \"id\": 545,
                    \"name\": \"lime_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 162,
                    \"green\": 197,
                    \"blue\": 55
                }
            },
            {
                \"item\": {
                    \"id\": 546,
                    \"name\": \"pink_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 235,
                    \"green\": 154,
                    \"blue\": 181
                }
            },
            {
                \"item\": {
                    \"id\": 547,
                    \"name\": \"gray_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 83,
                    \"green\": 90,
                    \"blue\": 93
                }
            },
            {
                \"item\": {
                    \"id\": 548,
                    \"name\": \"light_gray_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 144,
                    \"green\": 166,
                    \"blue\": 167
                }
            },
            {
                \"item\": {
                    \"id\": 549,
                    \"name\": \"cyan_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 52,
                    \"green\": 118,
                    \"blue\": 125
                }
            },
            {
                \"item\": {
                    \"id\": 550,
                    \"name\": \"purple_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 109,
                    \"green\": 48,
                    \"blue\": 152
                }
            },
            {
                \"item\": {
                    \"id\": 551,
                    \"name\": \"blue_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 47,
                    \"green\": 64,
                    \"blue\": 139
                }
            },
            {
                \"item\": {
                    \"id\": 552,
                    \"name\": \"brown_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 119,
                    \"green\": 106,
                    \"blue\": 85
                }
            },
            {
                \"item\": {
                    \"id\": 553,
                    \"name\": \"green_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 117,
                    \"green\": 142,
                    \"blue\": 67
                }
            },
            {
                \"item\": {
                    \"id\": 554,
                    \"name\": \"red_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 181,
                    \"green\": 59,
                    \"blue\": 53
                }
            },
            {
                \"item\": {
                    \"id\": 555,
                    \"name\": \"black_glazed_terracotta\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 67,
                    \"green\": 30,
                    \"blue\": 32
                }
            },
            {
                \"item\": {
                    \"id\": 556,
                    \"name\": \"white_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 207,
                    \"green\": 213,
                    \"blue\": 214
                }
            },
            {
                \"item\": {
                    \"id\": 557,
                    \"name\": \"orange_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 224,
                    \"green\": 97,
                    \"blue\": 0
                }
            },
            {
                \"item\": {
                    \"id\": 558,
                    \"name\": \"magenta_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 169,
                    \"green\": 48,
                    \"blue\": 159
                }
            },
            {
                \"item\": {
                    \"id\": 559,
                    \"name\": \"light_blue_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 35,
                    \"green\": 137,
                    \"blue\": 198
                }
            },
            {
                \"item\": {
                    \"id\": 560,
                    \"name\": \"yellow_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 240,
                    \"green\": 175,
                    \"blue\": 21
                }
            },
            {
                \"item\": {
                    \"id\": 561,
                    \"name\": \"lime_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 94,
                    \"green\": 168,
                    \"blue\": 24
                }
            },
            {
                \"item\": {
                    \"id\": 562,
                    \"name\": \"pink_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 213,
                    \"green\": 101,
                    \"blue\": 142
                }
            },
            {
                \"item\": {
                    \"id\": 563,
                    \"name\": \"gray_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 54,
                    \"green\": 57,
                    \"blue\": 61
                }
            },
            {
                \"item\": {
                    \"id\": 564,
                    \"name\": \"light_gray_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 125,
                    \"green\": 125,
                    \"blue\": 115
                }
            },
            {
                \"item\": {
                    \"id\": 565,
                    \"name\": \"cyan_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 21,
                    \"green\": 119,
                    \"blue\": 136
                }
            },
            {
                \"item\": {
                    \"id\": 566,
                    \"name\": \"purple_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 100,
                    \"green\": 31,
                    \"blue\": 156
                }
            },
            {
                \"item\": {
                    \"id\": 567,
                    \"name\": \"blue_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 44,
                    \"green\": 46,
                    \"blue\": 143
                }
            },
            {
                \"item\": {
                    \"id\": 568,
                    \"name\": \"brown_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 96,
                    \"green\": 59,
                    \"blue\": 31
                }
            },
            {
                \"item\": {
                    \"id\": 569,
                    \"name\": \"green_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 73,
                    \"green\": 91,
                    \"blue\": 36
                }
            },
            {
                \"item\": {
                    \"id\": 570,
                    \"name\": \"red_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 142,
                    \"green\": 32,
                    \"blue\": 32
                }
            },
            {
                \"item\": {
                    \"id\": 571,
                    \"name\": \"black_concrete\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 8,
                    \"green\": 10,
                    \"blue\": 15
                }
            },
            {
                \"item\": {
                    \"id\": 572,
                    \"name\": \"white_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 225,
                    \"green\": 227,
                    \"blue\": 227
                }
            },
            {
                \"item\": {
                    \"id\": 573,
                    \"name\": \"orange_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 227,
                    \"green\": 131,
                    \"blue\": 31
                }
            },
            {
                \"item\": {
                    \"id\": 574,
                    \"name\": \"magenta_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 192,
                    \"green\": 83,
                    \"blue\": 184
                }
            },
            {
                \"item\": {
                    \"id\": 575,
                    \"name\": \"light_blue_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 74,
                    \"green\": 180,
                    \"blue\": 213
                }
            },
            {
                \"item\": {
                    \"id\": 576,
                    \"name\": \"yellow_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 232,
                    \"green\": 199,
                    \"blue\": 54
                }
            },
            {
                \"item\": {
                    \"id\": 577,
                    \"name\": \"lime_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 125,
                    \"green\": 189,
                    \"blue\": 41
                }
            },
            {
                \"item\": {
                    \"id\": 578,
                    \"name\": \"pink_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 228,
                    \"green\": 153,
                    \"blue\": 181
                }
            },
            {
                \"item\": {
                    \"id\": 579,
                    \"name\": \"gray_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 76,
                    \"green\": 81,
                    \"blue\": 84
                }
            },
            {
                \"item\": {
                    \"id\": 580,
                    \"name\": \"light_gray_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 154,
                    \"green\": 154,
                    \"blue\": 148
                }
            },
            {
                \"item\": {
                    \"id\": 581,
                    \"name\": \"cyan_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 36,
                    \"green\": 147,
                    \"blue\": 157
                }
            },
            {
                \"item\": {
                    \"id\": 582,
                    \"name\": \"purple_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 131,
                    \"green\": 55,
                    \"blue\": 177
                }
            },
            {
                \"item\": {
                    \"id\": 583,
                    \"name\": \"blue_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 70,
                    \"green\": 73,
                    \"blue\": 166
                }
            },
            {
                \"item\": {
                    \"id\": 584,
                    \"name\": \"brown_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 125,
                    \"green\": 84,
                    \"blue\": 53
                }
            },
            {
                \"item\": {
                    \"id\": 585,
                    \"name\": \"green_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 97,
                    \"green\": 119,
                    \"blue\": 44
                }
            },
            {
                \"item\": {
                    \"id\": 586,
                    \"name\": \"red_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 168,
                    \"green\": 54,
                    \"blue\": 50
                }
            },
            {
                \"item\": {
                    \"id\": 587,
                    \"name\": \"black_concrete_powder\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 25,
                    \"green\": 26,
                    \"blue\": 31
                }
            },
            {
                \"item\": {
                    \"id\": 592,
                    \"name\": \"dead_tube_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 130,
                    \"green\": 123,
                    \"blue\": 119
                }
            },
            {
                \"item\": {
                    \"id\": 593,
                    \"name\": \"dead_brain_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 124,
                    \"green\": 117,
                    \"blue\": 114
                }
            },
            {
                \"item\": {
                    \"id\": 594,
                    \"name\": \"dead_bubble_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 131,
                    \"green\": 123,
                    \"blue\": 119
                }
            },
            {
                \"item\": {
                    \"id\": 595,
                    \"name\": \"dead_fire_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 131,
                    \"green\": 123,
                    \"blue\": 119
                }
            },
            {
                \"item\": {
                    \"id\": 596,
                    \"name\": \"dead_horn_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 133,
                    \"green\": 126,
                    \"blue\": 122
                }
            },
            {
                \"item\": {
                    \"id\": 597,
                    \"name\": \"tube_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 49,
                    \"green\": 87,
                    \"blue\": 206
                }
            },
            {
                \"item\": {
                    \"id\": 598,
                    \"name\": \"brain_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 207,
                    \"green\": 91,
                    \"blue\": 159
                }
            },
            {
                \"item\": {
                    \"id\": 599,
                    \"name\": \"bubble_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 165,
                    \"green\": 26,
                    \"blue\": 162
                }
            },
            {
                \"item\": {
                    \"id\": 600,
                    \"name\": \"fire_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 163,
                    \"green\": 35,
                    \"blue\": 46
                }
            },
            {
                \"item\": {
                    \"id\": 601,
                    \"name\": \"horn_coral_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 216,
                    \"green\": 199,
                    \"blue\": 66
                }
            },
            {
                \"item\": {
                    \"id\": 633,
                    \"name\": \"blue_ice\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 116,
                    \"green\": 167,
                    \"blue\": 253
                }
            },
            {
                \"item\": {
                    \"id\": 681,
                    \"name\": \"loom\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 145,
                    \"green\": 101,
                    \"blue\": 72
                }
            },
            {
                \"item\": {
                    \"id\": 682,
                    \"name\": \"barrel\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 107,
                    \"green\": 81,
                    \"blue\": 50
                }
            },
            {
                \"item\": {
                    \"id\": 683,
                    \"name\": \"smoker\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 102,
                    \"green\": 91,
                    \"blue\": 75
                }
            },
            {
                \"item\": {
                    \"id\": 684,
                    \"name\": \"blast_furnace\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 107,
                    \"green\": 107,
                    \"blue\": 107
                }
            },
            {
                \"item\": {
                    \"id\": 685,
                    \"name\": \"cartography_table\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 70,
                    \"green\": 50,
                    \"blue\": 34
                }
            },
            {
                \"item\": {
                    \"id\": 686,
                    \"name\": \"fletching_table\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 191,
                    \"green\": 167,
                    \"blue\": 129
                }
            },
            {
                \"item\": {
                    \"id\": 689,
                    \"name\": \"smithing_table\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 55,
                    \"green\": 35,
                    \"blue\": 35
                }
            },
            {
                \"item\": {
                    \"id\": 697,
                    \"name\": \"warped_stem\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 58,
                    \"green\": 58,
                    \"blue\": 77
                }
            },
            {
                \"item\": {
                    \"id\": 698,
                    \"name\": \"stripped_warped_stem\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 57,
                    \"green\": 150,
                    \"blue\": 147
                }
            },
            {
                \"item\": {
                    \"id\": 701,
                    \"name\": \"warped_nylium\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 43,
                    \"green\": 114,
                    \"blue\": 101
                }
            },
            {
                \"item\": {
                    \"id\": 703,
                    \"name\": \"warped_wart_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 22,
                    \"green\": 119,
                    \"blue\": 121
                }
            },
            {
                \"item\": {
                    \"id\": 706,
                    \"name\": \"crimson_stem\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 92,
                    \"green\": 25,
                    \"blue\": 29
                }
            },
            {
                \"item\": {
                    \"id\": 707,
                    \"name\": \"stripped_crimson_stem\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 137,
                    \"green\": 57,
                    \"blue\": 90
                }
            },
            {
                \"item\": {
                    \"id\": 710,
                    \"name\": \"crimson_nylium\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 130,
                    \"green\": 31,
                    \"blue\": 31
                }
            },
            {
                \"item\": {
                    \"id\": 712,
                    \"name\": \"shroomlight\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 240,
                    \"green\": 146,
                    \"blue\": 70
                }
            },
            {
                \"item\": {
                    \"id\": 718,
                    \"name\": \"crimson_planks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 101,
                    \"green\": 48,
                    \"blue\": 70
                }
            },
            {
                \"item\": {
                    \"id\": 719,
                    \"name\": \"warped_planks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 43,
                    \"green\": 104,
                    \"blue\": 99
                }
            },
            {
                \"item\": {
                    \"id\": 740,
                    \"name\": \"structure_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 88,
                    \"green\": 74,
                    \"blue\": 90
                }
            },
            {
                \"item\": {
                    \"id\": 741,
                    \"name\": \"jigsaw\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 62,
                    \"green\": 53,
                    \"blue\": 63
                }
            },
            {
                \"item\": {
                    \"id\": 742,
                    \"name\": \"composter\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 112,
                    \"green\": 70,
                    \"blue\": 31
                }
            },
            {
                \"item\": {
                    \"id\": 743,
                    \"name\": \"target\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 229,
                    \"green\": 176,
                    \"blue\": 168
                }
            },
            {
                \"item\": {
                    \"id\": 744,
                    \"name\": \"bee_nest\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 196,
                    \"green\": 150,
                    \"blue\": 77
                }
            },
            {
                \"item\": {
                    \"id\": 745,
                    \"name\": \"beehive\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 157,
                    \"green\": 126,
                    \"blue\": 75
                }
            },
            {
                \"item\": {
                    \"id\": 747,
                    \"name\": \"honeycomb_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 229,
                    \"green\": 148,
                    \"blue\": 29
                }
            },
            {
                \"item\": {
                    \"id\": 748,
                    \"name\": \"netherite_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 66,
                    \"green\": 61,
                    \"blue\": 63
                }
            },
            {
                \"item\": {
                    \"id\": 749,
                    \"name\": \"ancient_debris\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 95,
                    \"green\": 63,
                    \"blue\": 55
                }
            },
            {
                \"item\": {
                    \"id\": 750,
                    \"name\": \"crying_obsidian\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 32,
                    \"green\": 10,
                    \"blue\": 60
                }
            },
            {
                \"item\": {
                    \"id\": 751,
                    \"name\": \"respawn_anchor\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 42,
                    \"green\": 26,
                    \"blue\": 64
                }
            },
            {
                \"item\": {
                    \"id\": 756,
                    \"name\": \"lodestone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 119,
                    \"green\": 119,
                    \"blue\": 123
                }
            },
            {
                \"item\": {
                    \"id\": 757,
                    \"name\": \"blackstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 42,
                    \"green\": 35,
                    \"blue\": 40
                }
            },
            {
                \"item\": {
                    \"id\": 761,
                    \"name\": \"polished_blackstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 53,
                    \"green\": 48,
                    \"blue\": 56
                }
            },
            {
                \"item\": {
                    \"id\": 762,
                    \"name\": \"polished_blackstone_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 48,
                    \"green\": 42,
                    \"blue\": 49
                }
            },
            {
                \"item\": {
                    \"id\": 763,
                    \"name\": \"cracked_polished_blackstone_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 44,
                    \"green\": 37,
                    \"blue\": 43
                }
            },
            {
                \"item\": {
                    \"id\": 764,
                    \"name\": \"chiseled_polished_blackstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 53,
                    \"green\": 48,
                    \"blue\": 56
                }
            },
            {
                \"item\": {
                    \"id\": 768,
                    \"name\": \"gilded_blackstone\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 55,
                    \"green\": 42,
                    \"blue\": 38
                }
            },
            {
                \"item\": {
                    \"id\": 774,
                    \"name\": \"chiseled_nether_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 47,
                    \"green\": 23,
                    \"blue\": 28
                }
            },
            {
                \"item\": {
                    \"id\": 775,
                    \"name\": \"cracked_nether_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 40,
                    \"green\": 20,
                    \"blue\": 23
                }
            },
            {
                \"item\": {
                    \"id\": 776,
                    \"name\": \"quartz_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 234,
                    \"green\": 229,
                    \"blue\": 221
                }
            },
            {
                \"item\": {
                    \"id\": 811,
                    \"name\": \"amethyst_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 133,
                    \"green\": 97,
                    \"blue\": 191
                }
            },
            {
                \"item\": {
                    \"id\": 812,
                    \"name\": \"budding_amethyst\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 132,
                    \"green\": 96,
                    \"blue\": 186
                }
            },
            {
                \"item\": {
                    \"id\": 817,
                    \"name\": \"tuff\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 108,
                    \"green\": 109,
                    \"blue\": 102
                }
            },
            {
                \"item\": {
                    \"id\": 818,
                    \"name\": \"calcite\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 223,
                    \"green\": 224,
                    \"blue\": 220
                }
            },
            {
                \"item\": {
                    \"id\": 820,
                    \"name\": \"powder_snow\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 248,
                    \"green\": 253,
                    \"blue\": 253
                }
            },
            {
                \"item\": {
                    \"id\": 822,
                    \"name\": \"oxidized_copper\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 82,
                    \"green\": 162,
                    \"blue\": 132
                }
            },
            {
                \"item\": {
                    \"id\": 823,
                    \"name\": \"weathered_copper\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 108,
                    \"green\": 153,
                    \"blue\": 110
                }
            },
            {
                \"item\": {
                    \"id\": 824,
                    \"name\": \"exposed_copper\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 161,
                    \"green\": 125,
                    \"blue\": 103
                }
            },
            {
                \"item\": {
                    \"id\": 825,
                    \"name\": \"copper_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 192,
                    \"green\": 107,
                    \"blue\": 79
                }
            },
            {
                \"item\": {
                    \"id\": 826,
                    \"name\": \"copper_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 124,
                    \"green\": 125,
                    \"blue\": 120
                }
            },
            {
                \"item\": {
                    \"id\": 827,
                    \"name\": \"deepslate_copper_ore\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 92,
                    \"green\": 93,
                    \"blue\": 89
                }
            },
            {
                \"item\": {
                    \"id\": 828,
                    \"name\": \"oxidized_cut_copper\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 79,
                    \"green\": 153,
                    \"blue\": 126
                }
            },
            {
                \"item\": {
                    \"id\": 829,
                    \"name\": \"weathered_cut_copper\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 109,
                    \"green\": 145,
                    \"blue\": 107
                }
            },
            {
                \"item\": {
                    \"id\": 830,
                    \"name\": \"exposed_cut_copper\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 154,
                    \"green\": 121,
                    \"blue\": 101
                }
            },
            {
                \"item\": {
                    \"id\": 831,
                    \"name\": \"cut_copper\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 191,
                    \"green\": 106,
                    \"blue\": 80
                }
            },
            {
                \"item\": {
                    \"id\": 858,
                    \"name\": \"dripstone_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 134,
                    \"green\": 107,
                    \"blue\": 92
                }
            },
            {
                \"item\": {
                    \"id\": 865,
                    \"name\": \"moss_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 89,
                    \"green\": 109,
                    \"blue\": 45
                }
            },
            {
                \"item\": {
                    \"id\": 870,
                    \"name\": \"rooted_dirt\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 144,
                    \"green\": 103,
                    \"blue\": 76
                }
            },
            {
                \"item\": {
                    \"id\": 871,
                    \"name\": \"deepslate\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 80,
                    \"green\": 80,
                    \"blue\": 82
                }
            },
            {
                \"item\": {
                    \"id\": 872,
                    \"name\": \"cobbled_deepslate\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 77,
                    \"green\": 77,
                    \"blue\": 80
                }
            },
            {
                \"item\": {
                    \"id\": 876,
                    \"name\": \"polished_deepslate\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 72,
                    \"green\": 72,
                    \"blue\": 73
                }
            },
            {
                \"item\": {
                    \"id\": 880,
                    \"name\": \"deepslate_tiles\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 54,
                    \"green\": 54,
                    \"blue\": 55
                }
            },
            {
                \"item\": {
                    \"id\": 884,
                    \"name\": \"deepslate_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 70,
                    \"green\": 70,
                    \"blue\": 71
                }
            },
            {
                \"item\": {
                    \"id\": 888,
                    \"name\": \"chiseled_deepslate\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 54,
                    \"green\": 54,
                    \"blue\": 54
                }
            },
            {
                \"item\": {
                    \"id\": 889,
                    \"name\": \"cracked_deepslate_bricks\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 64,
                    \"green\": 64,
                    \"blue\": 65
                }
            },
            {
                \"item\": {
                    \"id\": 890,
                    \"name\": \"cracked_deepslate_tiles\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 52,
                    \"green\": 52,
                    \"blue\": 52
                }
            },
            {
                \"item\": {
                    \"id\": 892,
                    \"name\": \"smooth_basalt\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 72,
                    \"green\": 72,
                    \"blue\": 78
                }
            },
            {
                \"item\": {
                    \"id\": 893,
                    \"name\": \"raw_iron_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 166,
                    \"green\": 135,
                    \"blue\": 107
                }
            },
            {
                \"item\": {
                    \"id\": 894,
                    \"name\": \"raw_copper_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 154,
                    \"green\": 105,
                    \"blue\": 79
                }
            },
            {
                \"item\": {
                    \"id\": 895,
                    \"name\": \"raw_gold_block\",
                    \"transparent\": false
                },
                \"color\": {
                    \"red\": 221,
                    \"green\": 169,
                    \"blue\": 46
                }
            }
        ]",
        );
        match blocks {
            Ok(blocks) => Ok(blocks),
            Err(_) => Err(InvalidJson),
        }
    }
}
