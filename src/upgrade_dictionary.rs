pub struct UpgradeEntry {
    pub code: &'static str,
    pub chinese_name: &'static str,
    pub initial_level: i32,
}

pub const ITEM_DICTIONARY_FUSION: &[UpgradeEntry] = &[
    UpgradeEntry { code: "Hero_Item_Charge",       chinese_name: "盾冲",     initial_level: 1 },
    UpgradeEntry { code: "Hero_Item_DeathZone",    chinese_name: "死亡区域", initial_level: 1 },
];

pub const TRAIT_DICTIONARY: &[UpgradeEntry] = &[
    UpgradeEntry { code: "Hero_Trait_BluntWeapons",      chinese_name: "重型武器",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_CheaperItems",      chinese_name: "收藏�?,       initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_CheaperSkills",     chinese_name: "技艺熟�?,     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_ExtraArmor",        chinese_name: "钢铁之躯",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_ExtraUnit",         chinese_name: "深受爱戴",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_ExtraUses",         chinese_name: "重型负载",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Fast",              chinese_name: "加速行�?,     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_FastReplenish",     chinese_name: "振奋人心的演�?, initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Giant",             chinese_name: "山岳",         initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_NoFlee",            chinese_name: "无畏",         initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_SharpWeapons",      chinese_name: "尖锐武器",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_ShortCooldown",     chinese_name: "精力充沛",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Sturdy",            chinese_name: "脚踏实地",     initial_level: 0 },
];

pub const TRAIT_DICTIONARY_OLDGREY_FLAG: &[UpgradeEntry] = &[
    UpgradeEntry { code: "Hero_Trait_AxeThrower",        chinese_name: "掷斧�?,       initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_CheaperClass",      chinese_name: "迅捷精�?,     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Regenerative",      chinese_name: "追猎",         initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Thorns",            chinese_name: "荆棘",         initial_level: 0 },
];

pub const TRAIT_DICTIONARY_FUSION: &[UpgradeEntry] = &[
    UpgradeEntry { code: "Hero_Trait_AxeThrower",        chinese_name: "投斧大队",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_StunCommander",     chinese_name: "眩晕大师",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Demolitionist",     chinese_name: "投弹专家",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_SlowAura",          chinese_name: "减速光�?,     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_AreaCommander",     chinese_name: "力场行�?,     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_LoneWolf",          chinese_name: "孤狼",         initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_ApocalypseKnight",  chinese_name: "天启三骑�?,   initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Ranger",            chinese_name: "游骑�?,       initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_ShieldOfEmpire",    chinese_name: "帝国之盾",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Ashtar",            chinese_name: "阿斯塔特",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Thorns",            chinese_name: "荆棘",         initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_CheaperClass",      chinese_name: "快速精�?,     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Regenerative",      chinese_name: "医疗训练",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Jumper",            chinese_name: "跳劈大队",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Creeper",           chinese_name: "短人部队",     initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Flyer",             chinese_name: "神鹰",         initial_level: 0 },
    UpgradeEntry { code: "Hero_Trait_Titan",             chinese_name: "泰坦",         initial_level: 0 },
];

pub const UPGRADE_DICTIONARY: &[UpgradeEntry] = &[
    UpgradeEntry { code: "Hero_Upgrade_Bomb",              chinese_name: "炸弹",       initial_level: 0 },
    UpgradeEntry { code: "Hero_Upgrade_Cornucopia",        chinese_name: "雅贝�?,     initial_level: 0 },
    UpgradeEntry { code: "Hero_Upgrade_Mine",              chinese_name: "地雷",       initial_level: 0 },
    UpgradeEntry { code: "Hero_Upgrade_PhilosophersStone", chinese_name: "贤者之�?,   initial_level: 0 },
    UpgradeEntry { code: "Hero_Upgrade_Size",              chinese_name: "指挥之戒",   initial_level: 0 },
    UpgradeEntry { code: "Hero_Upgrade_WarHorn",           chinese_name: "战争号角",       initial_level: 0 },
    UpgradeEntry { code: "Hero_Upgrade_Warhammer",         chinese_name: "战锤",       initial_level: 0 },
];

pub const SKILL_DICTIONARY: &[UpgradeEntry] = &[
    UpgradeEntry { code: "Hero_Skill_Charge",       chinese_name: "冲锋",     initial_level: 1 },
    UpgradeEntry { code: "Hero_Skill_ArrowRain",    chinese_name: "箭雨",     initial_level: 1 },
    UpgradeEntry { code: "Hero_Skill_ShieldWall",   chinese_name: "盾墙",     initial_level: 1 },
    UpgradeEntry { code: "Hero_Skill_Rally",        chinese_name: "集结",     initial_level: 1 },
    UpgradeEntry { code: "Hero_Skill_Berserker",    chinese_name: "狂战�?,   initial_level: 1 },
    UpgradeEntry { code: "Hero_Skill_Volley",       chinese_name: "齐射",     initial_level: 1 },
];

pub struct ClassEntry {
    pub code: &'static str,
    pub chinese_name: &'static str,
    pub initial_level: i32,
}

pub const CLASS_DICTIONARY: &[ClassEntry] = &[
    ClassEntry { code: "Hero_Class_Infantry", chinese_name: "步兵",   initial_level: 1 },
    ClassEntry { code: "Hero_Class_Archers",  chinese_name: "弓箭�?, initial_level: 1 },
    ClassEntry { code: "Hero_Class_Pikemen",  chinese_name: "长矛�?, initial_level: 1 },
];

