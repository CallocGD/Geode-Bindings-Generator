// This is where we keep most of the parsing logic that involve things such as vtables...

use std::collections::HashMap;

use crate::re;

static ENUMS: [&'static str; 90] = [
    "SearchType",
    "GameObjectType",
    "PulseEffectType",
    "TouchTriggerType",
    "PlayerButton",
    "GhostType",
    "TableViewCellEditingStyle",
    "UserListType",
    "GJErrorCode",
    "AccountError",
    "GJSongError",
    "LikeItemType",
    "CommentError",
    "BackupAccountError",
    "GJMusicAction",
    "CellAction",
    "GJActionCommand",
    "DifficultyIconType",
    "GauntletType",
    "GJMPErrorCode",
    "GJTimedLevelType",
    "SongSelectType",
    "AudioTargetType",
    "FMODReverbPreset",
    "DemonDifficultyType",
    "PlayerCollisionDirection",
    "ChestSpriteState",
    "FormatterType",
    "AudioModType",
    "GJAreaActionType",
    "SFXTriggerState",
    "SongTriggerState",
    "GJGameEvent",
    "GJSmartDirection",
    "SmartBlockType",
    "TouchTriggerControl",
    "SmartPrefabResult",
    "AudioSortType",
    "spriteMode",
    "GJAssetType",
    "CommentKeyType",
    "LevelLeaderboardMode",
    "StatKey",
    "TextStyleType",
    "InputValueType",
    "GJInputStyle",
    "GJDifficultyName",
    "GJFeatureState",
    "GJKeyGroup",
    "GJKeyCommand",
    "SelectSettingType",
    "gjParticleValue",
    "ColorSelectType",
    "AudioGuidelinesType",
    "SmartBrowseFilter",
    "GJUITouchEvent",
    "ObjectScaleType",
    "SavedActiveObjectState",
    "SavedSpecialObjectState",
    "SavedObjectStateRef",
    "CommentType",
    "BoomListType",
    "CurrencySpriteType",
    "CurrencyRewardType",
    "MenuAnimationType",
    "ShopType",
    "ZLayer",
    "UpdateResponse",
    "UnlockType",
    "SpecialRewardItem",
    "EditCommand",
    "PlaybackMode",
    "SelectArtType",
    "UndoCommand",
    "EasingType",
    "GJDifficulty",
    "GJLevelType",
    "GJRewardType",
    "IconType",
    "GJChallengeType",
    "GJScoreType",
    "LevelLeaderboardType",
    "GJHttpType",
    "DialogChatPlacement",
    "DialogAnimationType",
    "ComparisonType",
    "MoveTargetType",
    "TouchToggleMode",
    "LeaderboardState",
    "Speed",
];

/// 1.8 or lower...
pub fn old_should_keep_symbol<'a>(sym: &'a str) -> bool {
    // println!("DEBUG {sym}");
    if !sym.contains("::") || re::TYPEINFO_VTABLE_ETC.is_match(&sym) {
        return false;
    }
    let mut msym = sym.to_string();
    let _ = msym.split_off(msym.find(':').unwrap_or(0));
    let class_name = msym;

    if class_name == "cocos2d" {
        return false;
    }

    if class_name == "fmt" {
        return false;
    }

    for e in ENUMS {
        if class_name == e {
            return false;
        }
    }

    // final check for regex...
    return !re::OLD_JNI_INTERALS_CHECK.is_match(&class_name);
}

// TODO...
// function shouldKeepSymbol(sym) {
//     let keep = sym && sym.includes('::') && !sym.match(/(typeinfo|vtable|thunk|guard variable)/);
//     if (!keep) return false;
//     let className = sym.split('::')[0];
//     keep = !className.match(/^(_JNIEnv|internal|tinyxml2|cocos2d|DS_Dictionary|ObjectDecoder|ObjectDecoderDelegate|pugi|__cxx|__gnu_cxx|std|fmt|llvm|tk|xml_|MD5|rtsha1)/);
//     keep = keep && className !== "FMOD" && !sym.startsWith('FMOD_') && className != "tk";
//     keep = keep && !enumClasses.includes(className);
//     keep = keep && className != "CCContentManager";
//     return keep;
// }

// 1.9 or newer...
// TODO New Bindings for later updates...
#[allow(dead_code)]
pub fn new_should_keep_symbol<'a>(sym: &'a str) -> bool {
    if !sym.contains("::") || re::TYPEINFO_VTABLE_ETC.is_match(&sym) {
        return false;
    }
    let mut msym = sym.to_string();
    let class_name = msym.split_off(msym.find(':').unwrap_or(0));

    if re::NEW_JNI_INTERNALS_CHECK.is_match(&class_name)
        || class_name == "FMOD"
        || msym.starts_with("FMOD_")
        || class_name == "tk"
        || class_name == "CCContentManager"
    {
        return false;
    }

    for e in ENUMS {
        if class_name == e {
            return true;
        }
    }

    return true;
}

// TODO: in the future reduce cloning...

pub fn should_comment_out_function<'a, 'b>(class_name: &'a str, name: &'b str) -> bool {
    let mut mut_cls_name = class_name.to_string();
    let base_class_name = mut_cls_name.split_off(mut_cls_name.find(':').unwrap_or(0));

    if name.contains("...") {
        return true;
    }

    for p in [
        format!("{base_class_name}()"),
        format!("{}({} const&)", base_class_name.clone(), class_name),
        format!("${}(${}&&)", base_class_name.clone(), class_name),
        format!("~${base_class_name}"),
    ] {
        if name.starts_with(&p) {
            return true;
        }
    }
    // Final check...
    return name.starts_with("fmt::");
}

pub fn is_static_func<'c, 'f>(class_name: &'c str, func_sig: &'f str) -> bool {
    func_sig.starts_with("create(")
        || class_name == "GameToolbox"
        || class_name == "sharedState()"
        || class_name == "sharedEngine()"
        || class_name == "sharedDecoder()"
        || class_name == "sharedEngine()"
}

// From rust documentation. I have nothing to hide here, I reformmated the function
// mainly Might do more modifications to it in the future...

pub fn old_read_lines(filename: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|x| re::clean_function_sig(x.to_string()))
        .filter(|x| old_should_keep_symbol(&x))
        .map(String::from)
        .collect()
}

// Different logic is being applied to the new version since it uses different techniques
// than my original python script for older versions of the game.
// pub fn new_read_lines<'f, P>(filename:&'f std::path::PathBuf) -> Vec<String> {
//     std::fs::read_to_string(filename).unwrap()
//         .lines().map(String::from).collect()
// }

pub fn read_vtables_json_file<'f>(
    filename: &'f std::path::PathBuf,
) -> HashMap<String, serde_json::Value> {
    serde_json::from_str(
        &std::fs::read_to_string(filename).expect("Trouble reading from vtables json file"),
    )
    .expect("Trouble parsing from vtables json file")
}
