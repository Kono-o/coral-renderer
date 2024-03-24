use std::env;

pub const OS_NAME: &str = env::consts::OS;

pub const CRATE_NAME: &str = "carrot-on-a-stick";
pub const CRATE_PACKS_PATH: &str = "./packs/";

pub const RESOURCE_DIRS: [&str;2] = [
//linux
".minecraft/resourcepacks/",
//windows
"AppData/Roaming/.minecraft/resourcepacks/"
];

pub const TEXTURES_DIRS: [&str;2] = [
//java
"assets/minecraft/textures/block/",
//bedrock
"textures/blocks/"
];

pub const TEX_SIZE: u32 = 16;
pub const TILE_SIZE: u32 = TEX_SIZE + 2;
pub const ATLAS_SIZE_POT: u32 = 1024;
pub const ATLAS_SIZE: u32 = (ATLAS_SIZE_POT/TILE_SIZE) * TILE_SIZE;

pub const TEX_LIST: [&str; 990] = [
    "acacia_door_bottom",
    "acacia_door_top",
    "acacia_leaves",
    "acacia_log",
    "acacia_log_top",
    "acacia_planks",
    "acacia_sapling",
    "acacia_trapdoor",
    "activator_rail",
    "activator_rail_on",
    "allium",
    "amethyst_block",
    "amethyst_cluster",
    "ancient_debris_side",
    "ancient_debris_top",
    "andesite",
    "anvil",
    "anvil_top",
    "attached_melon_stem",
    "attached_pumpkin_stem",
    "azalea_leaves",
    "azalea_plant",
    "azalea_side",
    "azalea_top",
    "azure_bluet",
    "bamboo_block",
    "bamboo_block_top",
    "bamboo_door_bottom",
    "bamboo_door_top",
    "bamboo_fence",
    "bamboo_fence_gate",
    "bamboo_fence_gate_particle",
    "bamboo_fence_particle",
    "bamboo_large_leaves",
    "bamboo_mosaic",
    "bamboo_planks",
    "bamboo_singleleaf",
    "bamboo_small_leaves",
    "bamboo_stage0",
    "bamboo_stalk",
    "bamboo_trapdoor",
    "barrel_bottom",
    "barrel_side",
    "barrel_top",
    "barrel_top_open",
    "basalt_side",
    "basalt_top",
    "beacon",
    "bedrock",
    "beehive_end",
    "beehive_front",
    "beehive_front_honey",
    "beehive_side",
    "beetroots_stage0",
    "beetroots_stage1",
    "beetroots_stage2",
    "beetroots_stage3",
    "bee_nest_bottom",
    "bee_nest_front",
    "bee_nest_front_honey",
    "bee_nest_side",
    "bee_nest_top",
    "bell_bottom",
    "bell_side",
    "bell_top",
    "big_dripleaf_side",
    "big_dripleaf_stem",
    "big_dripleaf_tip",
    "big_dripleaf_top",
    "birch_door_bottom",
    "birch_door_top",
    "birch_leaves",
    "birch_log",
    "birch_log_top",
    "birch_planks",
    "birch_sapling",
    "birch_trapdoor",
    "blackstone",
    "blackstone_top",
    "black_candle",
    "black_candle_lit",
    "black_concrete",
    "black_concrete_powder",
    "black_glazed_terracotta",
    "black_shulker_box",
    "black_stained_glass",
    "black_stained_glass_pane_top",
    "black_terracotta",
    "black_wool",
    "blast_furnace_front",
    "blast_furnace_front_on",
    "blast_furnace_side",
    "blast_furnace_top",
    "blue_candle",
    "blue_candle_lit",
    "blue_concrete",
    "blue_concrete_powder",
    "blue_glazed_terracotta",
    "blue_ice",
    "blue_orchid",
    "blue_shulker_box",
    "blue_stained_glass",
    "blue_stained_glass_pane_top",
    "blue_terracotta",
    "blue_wool",
    "bone_block_side",
    "bone_block_top",
    "bookshelf",
    "brain_coral",
    "brain_coral_block",
    "brain_coral_fan",
    "brewing_stand",
    "brewing_stand_base",
    "bricks",
    "brown_candle",
    "brown_candle_lit",
    "brown_concrete",
    "brown_concrete_powder",
    "brown_glazed_terracotta",
    "brown_mushroom",
    "brown_mushroom_block",
    "brown_shulker_box",
    "brown_stained_glass",
    "brown_stained_glass_pane_top",
    "brown_terracotta",
    "brown_wool",
    "bubble_coral",
    "bubble_coral_block",
    "bubble_coral_fan",
    "budding_amethyst",
    "cactus_bottom",
    "cactus_side",
    "cactus_top",
    "cake_bottom",
    "cake_inner",
    "cake_side",
    "cake_top",
    "calcite",
    "calibrated_sculk_sensor_amethyst",
    "calibrated_sculk_sensor_input_side",
    "calibrated_sculk_sensor_top",
    "campfire_fire",
    "campfire_log",
    "campfire_log_lit",
    "candle",
    "candle_lit",
    "carrots_stage0",
    "carrots_stage1",
    "carrots_stage2",
    "carrots_stage3",
    "cartography_table_side1",
    "cartography_table_side2",
    "cartography_table_side3",
    "cartography_table_top",
    "carved_pumpkin",
    "cauldron_bottom",
    "cauldron_inner",
    "cauldron_side",
    "cauldron_top",
    "cave_vines",
    "cave_vines_lit",
    "cave_vines_plant",
    "cave_vines_plant_lit",
    "chain",
    "chain_command_block_back",
    "chain_command_block_conditional",
    "chain_command_block_front",
    "chain_command_block_side",
    "cherry_door_bottom",
    "cherry_door_top",
    "cherry_leaves",
    "cherry_log",
    "cherry_log_top",
    "cherry_planks",
    "cherry_sapling",
    "cherry_trapdoor",
    "chipped_anvil_top",
    "chiseled_bookshelf_empty",
    "chiseled_bookshelf_occupied",
    "chiseled_bookshelf_side",
    "chiseled_bookshelf_top",
    "chiseled_copper",
    "chiseled_deepslate",
    "chiseled_nether_bricks",
    "chiseled_polished_blackstone",
    "chiseled_quartz_block",
    "chiseled_quartz_block_top",
    "chiseled_red_sandstone",
    "chiseled_sandstone",
    "chiseled_stone_bricks",
    "chiseled_tuff",
    "chiseled_tuff_bricks",
    "chiseled_tuff_bricks_top",
    "chiseled_tuff_top",
    "chorus_flower",
    "chorus_flower_dead",
    "chorus_plant",
    "clay",
    "coal_block",
    "coal_ore",
    "coarse_dirt",
    "cobbled_deepslate",
    "cobblestone",
    "cobweb",
    "cocoa_stage0",
    "cocoa_stage1",
    "cocoa_stage2",
    "command_block_back",
    "command_block_conditional",
    "command_block_front",
    "command_block_side",
    "comparator",
    "comparator_on",
    "composter_bottom",
    "composter_compost",
    "composter_ready",
    "composter_side",
    "composter_top",
    "conduit",
    "copper_block",
    "copper_bulb",
    "copper_bulb_lit",
    "copper_bulb_lit_powered",
    "copper_bulb_powered",
    "copper_door_bottom",
    "copper_door_top",
    "copper_grate",
    "copper_ore",
    "copper_trapdoor",
    "cornflower",
    "cracked_deepslate_bricks",
    "cracked_deepslate_tiles",
    "cracked_nether_bricks",
    "cracked_polished_blackstone_bricks",
    "cracked_stone_bricks",
    "crafter_bottom",
    "crafter_east",
    "crafter_east_crafting",
    "crafter_east_triggered",
    "crafter_north",
    "crafter_north_crafting",
    "crafter_south",
    "crafter_south_triggered",
    "crafter_top",
    "crafter_top_crafting",
    "crafter_top_triggered",
    "crafter_west",
    "crafter_west_crafting",
    "crafter_west_triggered",
    "crafting_table_front",
    "crafting_table_side",
    "crafting_table_top",
    "crimson_door_bottom",
    "crimson_door_top",
    "crimson_fungus",
    "crimson_nylium",
    "crimson_nylium_side",
    "crimson_planks",
    "crimson_roots",
    "crimson_roots_pot",
    "crimson_stem",
    "crimson_stem_top",
    "crimson_trapdoor",
    "crying_obsidian",
    "cut_copper",
    "cut_red_sandstone",
    "cut_sandstone",
    "cyan_candle",
    "cyan_candle_lit",
    "cyan_concrete",
    "cyan_concrete_powder",
    "cyan_glazed_terracotta",
    "cyan_shulker_box",
    "cyan_stained_glass",
    "cyan_stained_glass_pane_top",
    "cyan_terracotta",
    "cyan_wool",
    "damaged_anvil_top",
    "dandelion",
    "dark_oak_door_bottom",
    "dark_oak_door_top",
    "dark_oak_leaves",
    "dark_oak_log",
    "dark_oak_log_top",
    "dark_oak_planks",
    "dark_oak_sapling",
    "dark_oak_trapdoor",
    "dark_prismarine",
    "daylight_detector_inverted_top",
    "daylight_detector_side",
    "daylight_detector_top",
    "dead_brain_coral",
    "dead_brain_coral_block",
    "dead_brain_coral_fan",
    "dead_bubble_coral",
    "dead_bubble_coral_block",
    "dead_bubble_coral_fan",
    "dead_bush",
    "dead_fire_coral",
    "dead_fire_coral_block",
    "dead_fire_coral_fan",
    "dead_horn_coral",
    "dead_horn_coral_block",
    "dead_horn_coral_fan",
    "dead_tube_coral",
    "dead_tube_coral_block",
    "dead_tube_coral_fan",
    "debug",
    "debug2",
    "deepslate",
    "deepslate_bricks",
    "deepslate_coal_ore",
    "deepslate_copper_ore",
    "deepslate_diamond_ore",
    "deepslate_emerald_ore",
    "deepslate_gold_ore",
    "deepslate_iron_ore",
    "deepslate_lapis_ore",
    "deepslate_redstone_ore",
    "deepslate_tiles",
    "deepslate_top",
    "destroy_stage_0",
    "destroy_stage_1",
    "destroy_stage_2",
    "destroy_stage_3",
    "destroy_stage_4",
    "destroy_stage_5",
    "destroy_stage_6",
    "destroy_stage_7",
    "destroy_stage_8",
    "destroy_stage_9",
    "detector_rail",
    "detector_rail_on",
    "diamond_block",
    "diamond_ore",
    "diorite",
    "dirt",
    "dirt_path_side",
    "dirt_path_top",
    "dispenser_front",
    "dispenser_front_vertical",
    "dragon_egg",
    "dried_kelp_bottom",
    "dried_kelp_side",
    "dried_kelp_top",
    "dripstone_block",
    "dropper_front",
    "dropper_front_vertical",
    "emerald_block",
    "emerald_ore",
    "enchanting_table_bottom",
    "enchanting_table_side",
    "enchanting_table_top",
    "end_portal_frame_eye",
    "end_portal_frame_side",
    "end_portal_frame_top",
    "end_rod",
    "end_stone",
    "end_stone_bricks",
    "exposed_chiseled_copper",
    "exposed_copper",
    "exposed_copper_bulb",
    "exposed_copper_bulb_lit",
    "exposed_copper_bulb_lit_powered",
    "exposed_copper_bulb_powered",
    "exposed_copper_door_bottom",
    "exposed_copper_door_top",
    "exposed_copper_grate",
    "exposed_copper_trapdoor",
    "exposed_cut_copper",
    "farmland",
    "farmland_moist",
    "fern",
    "fire_0",
    "fire_1",
    "fire_coral",
    "fire_coral_block",
    "fire_coral_fan",
    "fletching_table_front",
    "fletching_table_side",
    "fletching_table_top",
    "flowering_azalea_leaves",
    "flowering_azalea_side",
    "flowering_azalea_top",
    "flower_pot",
    "frogspawn",
    "frosted_ice_0",
    "frosted_ice_1",
    "frosted_ice_2",
    "frosted_ice_3",
    "furnace_front",
    "furnace_front_on",
    "furnace_side",
    "furnace_top",
    "gilded_blackstone",
    "glass",
    "glass_pane_top",
    "glowstone",
    "glow_item_frame",
    "glow_lichen",
    "gold_block",
    "gold_ore",
    "granite",
    "grass_block_side",
    "grass_block_side_overlay",
    "grass_block_snow",
    "grass_block_top",
    "gravel",
    "gray_candle",
    "gray_candle_lit",
    "gray_concrete",
    "gray_concrete_powder",
    "gray_glazed_terracotta",
    "gray_shulker_box",
    "gray_stained_glass",
    "gray_stained_glass_pane_top",
    "gray_terracotta",
    "gray_wool",
    "green_candle",
    "green_candle_lit",
    "green_concrete",
    "green_concrete_powder",
    "green_glazed_terracotta",
    "green_shulker_box",
    "green_stained_glass",
    "green_stained_glass_pane_top",
    "green_terracotta",
    "green_wool",
    "grindstone_pivot",
    "grindstone_round",
    "grindstone_side",
    "hanging_roots",
    "hay_block_side",
    "hay_block_top",
    "honeycomb_block",
    "honey_block_bottom",
    "honey_block_side",
    "honey_block_top",
    "hopper_inside",
    "hopper_outside",
    "hopper_top",
    "horn_coral",
    "horn_coral_block",
    "horn_coral_fan",
    "ice",
    "iron_bars",
    "iron_block",
    "iron_door_bottom",
    "iron_door_top",
    "iron_ore",
    "iron_trapdoor",
    "item_frame",
    "jack_o_lantern",
    "jigsaw_bottom",
    "jigsaw_lock",
    "jigsaw_side",
    "jigsaw_top",
    "jukebox_side",
    "jukebox_top",
    "jungle_door_bottom",
    "jungle_door_top",
    "jungle_leaves",
    "jungle_log",
    "jungle_log_top",
    "jungle_planks",
    "jungle_sapling",
    "jungle_trapdoor",
    "kelp",
    "kelp_plant",
    "ladder",
    "lantern",
    "lapis_block",
    "lapis_ore",
    "large_amethyst_bud",
    "large_fern_bottom",
    "large_fern_top",
    "lava_flow",
    "lava_still",
    "lectern_base",
    "lectern_front",
    "lectern_sides",
    "lectern_top",
    "lever",
    "lightning_rod",
    "lightning_rod_on",
    "light_blue_candle",
    "light_blue_candle_lit",
    "light_blue_concrete",
    "light_blue_concrete_powder",
    "light_blue_glazed_terracotta",
    "light_blue_shulker_box",
    "light_blue_stained_glass",
    "light_blue_stained_glass_pane_top",
    "light_blue_terracotta",
    "light_blue_wool",
    "light_gray_candle",
    "light_gray_candle_lit",
    "light_gray_concrete",
    "light_gray_concrete_powder",
    "light_gray_glazed_terracotta",
    "light_gray_shulker_box",
    "light_gray_stained_glass",
    "light_gray_stained_glass_pane_top",
    "light_gray_terracotta",
    "light_gray_wool",
    "lilac_bottom",
    "lilac_top",
    "lily_of_the_valley",
    "lily_pad",
    "lime_candle",
    "lime_candle_lit",
    "lime_concrete",
    "lime_concrete_powder",
    "lime_glazed_terracotta",
    "lime_shulker_box",
    "lime_stained_glass",
    "lime_stained_glass_pane_top",
    "lime_terracotta",
    "lime_wool",
    "lodestone_side",
    "lodestone_top",
    "loom_bottom",
    "loom_front",
    "loom_side",
    "loom_top",
    "magenta_candle",
    "magenta_candle_lit",
    "magenta_concrete",
    "magenta_concrete_powder",
    "magenta_glazed_terracotta",
    "magenta_shulker_box",
    "magenta_stained_glass",
    "magenta_stained_glass_pane_top",
    "magenta_terracotta",
    "magenta_wool",
    "magma",
    "mangrove_door_bottom",
    "mangrove_door_top",
    "mangrove_leaves",
    "mangrove_log",
    "mangrove_log_top",
    "mangrove_planks",
    "mangrove_propagule",
    "mangrove_propagule_hanging",
    "mangrove_roots_side",
    "mangrove_roots_top",
    "mangrove_trapdoor",
    "medium_amethyst_bud",
    "melon_side",
    "melon_stem",
    "melon_top",
    "mossy_cobblestone",
    "mossy_stone_bricks",
    "moss_block",
    "mud",
    "muddy_mangrove_roots_side",
    "muddy_mangrove_roots_top",
    "mud_bricks",
    "mushroom_block_inside",
    "mushroom_stem",
    "mycelium_side",
    "mycelium_top",
    "netherite_block",
    "netherrack",
    "nether_bricks",
    "nether_gold_ore",
    "nether_portal",
    "nether_quartz_ore",
    "nether_sprouts",
    "nether_wart_block",
    "nether_wart_stage0",
    "nether_wart_stage1",
    "nether_wart_stage2",
    "note_block",
    "oak_door_bottom",
    "oak_door_top",
    "oak_leaves",
    "oak_log",
    "oak_log_top",
    "oak_planks",
    "oak_sapling",
    "oak_trapdoor",
    "observer_back",
    "observer_back_on",
    "observer_front",
    "observer_side",
    "observer_top",
    "obsidian",
    "ochre_froglight_side",
    "ochre_froglight_top",
    "orange_candle",
    "orange_candle_lit",
    "orange_concrete",
    "orange_concrete_powder",
    "orange_glazed_terracotta",
    "orange_shulker_box",
    "orange_stained_glass",
    "orange_stained_glass_pane_top",
    "orange_terracotta",
    "orange_tulip",
    "orange_wool",
    "oxeye_daisy",
    "oxidized_chiseled_copper",
    "oxidized_copper",
    "oxidized_copper_bulb",
    "oxidized_copper_bulb_lit",
    "oxidized_copper_bulb_lit_powered",
    "oxidized_copper_bulb_powered",
    "oxidized_copper_door_bottom",
    "oxidized_copper_door_top",
    "oxidized_copper_grate",
    "oxidized_copper_trapdoor",
    "oxidized_cut_copper",
    "packed_ice",
    "packed_mud",
    "pearlescent_froglight_side",
    "pearlescent_froglight_top",
    "peony_bottom",
    "peony_top",
    "pink_candle",
    "pink_candle_lit",
    "pink_concrete",
    "pink_concrete_powder",
    "pink_glazed_terracotta",
    "pink_petals",
    "pink_petals_stem",
    "pink_shulker_box",
    "pink_stained_glass",
    "pink_stained_glass_pane_top",
    "pink_terracotta",
    "pink_tulip",
    "pink_wool",
    "piston_bottom",
    "piston_inner",
    "piston_side",
    "piston_top",
    "piston_top_sticky",
    "pitcher_crop_bottom",
    "pitcher_crop_bottom_stage_1",
    "pitcher_crop_bottom_stage_2",
    "pitcher_crop_bottom_stage_3",
    "pitcher_crop_bottom_stage_4",
    "pitcher_crop_side",
    "pitcher_crop_top",
    "pitcher_crop_top_stage_3",
    "pitcher_crop_top_stage_4",
    "podzol_side",
    "podzol_top",
    "pointed_dripstone_down_base",
    "pointed_dripstone_down_frustum",
    "pointed_dripstone_down_middle",
    "pointed_dripstone_down_tip",
    "pointed_dripstone_down_tip_merge",
    "pointed_dripstone_up_base",
    "pointed_dripstone_up_frustum",
    "pointed_dripstone_up_middle",
    "pointed_dripstone_up_tip",
    "pointed_dripstone_up_tip_merge",
    "polished_andesite",
    "polished_basalt_side",
    "polished_basalt_top",
    "polished_blackstone",
    "polished_blackstone_bricks",
    "polished_deepslate",
    "polished_diorite",
    "polished_granite",
    "polished_tuff",
    "poppy",
    "potatoes_stage0",
    "potatoes_stage1",
    "potatoes_stage2",
    "potatoes_stage3",
    "potted_azalea_bush_plant",
    "potted_azalea_bush_side",
    "potted_azalea_bush_top",
    "potted_flowering_azalea_bush_plant",
    "potted_flowering_azalea_bush_side",
    "potted_flowering_azalea_bush_top",
    "powder_snow",
    "powered_rail",
    "powered_rail_on",
    "prismarine",
    "prismarine_bricks",
    "pumpkin_side",
    "pumpkin_stem",
    "pumpkin_top",
    "purple_candle",
    "purple_candle_lit",
    "purple_concrete",
    "purple_concrete_powder",
    "purple_glazed_terracotta",
    "purple_shulker_box",
    "purple_stained_glass",
    "purple_stained_glass_pane_top",
    "purple_terracotta",
    "purple_wool",
    "purpur_block",
    "purpur_pillar",
    "purpur_pillar_top",
    "quartz_block_bottom",
    "quartz_block_side",
    "quartz_block_top",
    "quartz_bricks",
    "quartz_pillar",
    "quartz_pillar_top",
    "rail",
    "rail_corner",
    "raw_copper_block",
    "raw_gold_block",
    "raw_iron_block",
    "redstone_block",
    "redstone_dust_dot",
    "redstone_dust_line0",
    "redstone_dust_line1",
    "redstone_dust_overlay",
    "redstone_lamp",
    "redstone_lamp_on",
    "redstone_ore",
    "redstone_torch",
    "redstone_torch_off",
    "red_candle",
    "red_candle_lit",
    "red_concrete",
    "red_concrete_powder",
    "red_glazed_terracotta",
    "red_mushroom",
    "red_mushroom_block",
    "red_nether_bricks",
    "red_sand",
    "red_sandstone",
    "red_sandstone_bottom",
    "red_sandstone_top",
    "red_shulker_box",
    "red_stained_glass",
    "red_stained_glass_pane_top",
    "red_terracotta",
    "red_tulip",
    "red_wool",
    "reinforced_deepslate_bottom",
    "reinforced_deepslate_side",
    "reinforced_deepslate_top",
    "repeater",
    "repeater_on",
    "repeating_command_block_back",
    "repeating_command_block_conditional",
    "repeating_command_block_front",
    "repeating_command_block_side",
    "respawn_anchor_bottom",
    "respawn_anchor_side0",
    "respawn_anchor_side1",
    "respawn_anchor_side2",
    "respawn_anchor_side3",
    "respawn_anchor_side4",
    "respawn_anchor_top",
    "respawn_anchor_top_off",
    "rooted_dirt",
    "rose_bush_bottom",
    "rose_bush_top",
    "sand",
    "sandstone",
    "sandstone_bottom",
    "sandstone_top",
    "scaffolding_bottom",
    "scaffolding_side",
    "scaffolding_top",
    "sculk",
    "sculk_catalyst_bottom",
    "sculk_catalyst_side",
    "sculk_catalyst_side_bloom",
    "sculk_catalyst_top",
    "sculk_catalyst_top_bloom",
    "sculk_sensor_bottom",
    "sculk_sensor_side",
    "sculk_sensor_tendril_active",
    "sculk_sensor_tendril_inactive",
    "sculk_sensor_top",
    "sculk_shrieker_bottom",
    "sculk_shrieker_can_summon_inner_top",
    "sculk_shrieker_inner_top",
    "sculk_shrieker_side",
    "sculk_shrieker_top",
    "sculk_vein",
    "seagrass",
    "sea_lantern",
    "sea_pickle",
    "short_grass",
    "shroomlight",
    "shulker_box",
    "slime_block",
    "small_amethyst_bud",
    "small_dripleaf_side",
    "small_dripleaf_stem_bottom",
    "small_dripleaf_stem_top",
    "small_dripleaf_top",
    "smithing_table_bottom",
    "smithing_table_front",
    "smithing_table_side",
    "smithing_table_top",
    "smoker_bottom",
    "smoker_front",
    "smoker_front_on",
    "smoker_side",
    "smoker_top",
    "smooth_basalt",
    "smooth_stone",
    "smooth_stone_slab_side",
    "sniffer_egg_not_cracked_bottom",
    "sniffer_egg_not_cracked_east",
    "sniffer_egg_not_cracked_north",
    "sniffer_egg_not_cracked_south",
    "sniffer_egg_not_cracked_top",
    "sniffer_egg_not_cracked_west",
    "sniffer_egg_slightly_cracked_bottom",
    "sniffer_egg_slightly_cracked_east",
    "sniffer_egg_slightly_cracked_north",
    "sniffer_egg_slightly_cracked_south",
    "sniffer_egg_slightly_cracked_top",
    "sniffer_egg_slightly_cracked_west",
    "sniffer_egg_very_cracked_bottom",
    "sniffer_egg_very_cracked_east",
    "sniffer_egg_very_cracked_north",
    "sniffer_egg_very_cracked_south",
    "sniffer_egg_very_cracked_top",
    "sniffer_egg_very_cracked_west",
    "snow",
    "soul_campfire_fire",
    "soul_campfire_log_lit",
    "soul_fire_0",
    "soul_fire_1",
    "soul_lantern",
    "soul_sand",
    "soul_soil",
    "soul_torch",
    "spawner",
    "sponge",
    "spore_blossom",
    "spore_blossom_base",
    "spruce_door_bottom",
    "spruce_door_top",
    "spruce_leaves",
    "spruce_log",
    "spruce_log_top",
    "spruce_planks",
    "spruce_sapling",
    "spruce_trapdoor",
    "stone",
    "stonecutter_bottom",
    "stonecutter_saw",
    "stonecutter_side",
    "stonecutter_top",
    "stone_bricks",
    "stripped_acacia_log",
    "stripped_acacia_log_top",
    "stripped_bamboo_block",
    "stripped_bamboo_block_top",
    "stripped_birch_log",
    "stripped_birch_log_top",
    "stripped_cherry_log",
    "stripped_cherry_log_top",
    "stripped_crimson_stem",
    "stripped_crimson_stem_top",
    "stripped_dark_oak_log",
    "stripped_dark_oak_log_top",
    "stripped_jungle_log",
    "stripped_jungle_log_top",
    "stripped_mangrove_log",
    "stripped_mangrove_log_top",
    "stripped_oak_log",
    "stripped_oak_log_top",
    "stripped_spruce_log",
    "stripped_spruce_log_top",
    "stripped_warped_stem",
    "stripped_warped_stem_top",
    "structure_block",
    "structure_block_corner",
    "structure_block_data",
    "structure_block_load",
    "structure_block_save",
    "sugar_cane",
    "sunflower_back",
    "sunflower_bottom",
    "sunflower_front",
    "sunflower_top",
    "suspicious_gravel_0",
    "suspicious_gravel_1",
    "suspicious_gravel_2",
    "suspicious_gravel_3",
    "suspicious_sand_0",
    "suspicious_sand_1",
    "suspicious_sand_2",
    "suspicious_sand_3",
    "sweet_berry_bush_stage0",
    "sweet_berry_bush_stage1",
    "sweet_berry_bush_stage2",
    "sweet_berry_bush_stage3",
    "tall_grass_bottom",
    "tall_grass_top",
    "tall_seagrass_bottom",
    "tall_seagrass_top",
    "target_side",
    "target_top",
    "terracotta",
    "tinted_glass",
    "tnt_bottom",
    "tnt_side",
    "tnt_top",
    "torch",
    "torchflower",
    "torchflower_crop_stage0",
    "torchflower_crop_stage1",
    "trial_spawner_bottom",
    "trial_spawner_side_active",
    "trial_spawner_side_inactive",
    "trial_spawner_top_active",
    "trial_spawner_top_ejecting_reward",
    "trial_spawner_top_inactive",
    "tripwire",
    "tripwire_hook",
    "tube_coral",
    "tube_coral_block",
    "tube_coral_fan",
    "tuff",
    "tuff_bricks",
    "turtle_egg",
    "turtle_egg_slightly_cracked",
    "turtle_egg_very_cracked",
    "twisting_vines",
    "twisting_vines_plant",
    "verdant_froglight_side",
    "verdant_froglight_top",
    "vine",
    "warped_door_bottom",
    "warped_door_top",
    "warped_fungus",
    "warped_nylium",
    "warped_nylium_side",
    "warped_planks",
    "warped_roots",
    "warped_roots_pot",
    "warped_stem",
    "warped_stem_top",
    "warped_trapdoor",
    "warped_wart_block",
    "water_flow",
    "water_overlay",
    "water_still",
    "weathered_chiseled_copper",
    "weathered_copper",
    "weathered_copper_bulb",
    "weathered_copper_bulb_lit",
    "weathered_copper_bulb_lit_powered",
    "weathered_copper_bulb_powered",
    "weathered_copper_door_bottom",
    "weathered_copper_door_top",
    "weathered_copper_grate",
    "weathered_copper_trapdoor",
    "weathered_cut_copper",
    "weeping_vines",
    "weeping_vines_plant",
    "wet_sponge",
    "wheat_stage0",
    "wheat_stage1",
    "wheat_stage2",
    "wheat_stage3",
    "wheat_stage4",
    "wheat_stage5",
    "wheat_stage6",
    "wheat_stage7",
    "white_candle",
    "white_candle_lit",
    "white_concrete",
    "white_concrete_powder",
    "white_glazed_terracotta",
    "white_shulker_box",
    "white_stained_glass",
    "white_stained_glass_pane_top",
    "white_terracotta",
    "white_tulip",
    "white_wool",
    "wither_rose",
    "yellow_candle",
    "yellow_candle_lit",
    "yellow_concrete",
    "yellow_concrete_powder",
    "yellow_glazed_terracotta",
    "yellow_shulker_box",
    "yellow_stained_glass",
    "yellow_stained_glass_pane_top",
    "yellow_terracotta",
    "yellow_wool"
];
