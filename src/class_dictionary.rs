pub struct ClassEntry {
    pub code: &'static str,
    pub chinese_name: &'static str,
}

pub const CLASS_DICTIONARY: &[ClassEntry] = &[
    ClassEntry { code: "Hero_Class_Infantry", chinese_name: "步兵" },
    ClassEntry { code: "Hero_Class_Archers",  chinese_name: "弓箭手" },
    ClassEntry { code: "Hero_Class_Pikemen",  chinese_name: "长矛手" },
];
