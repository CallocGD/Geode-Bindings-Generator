"""This is a modification of generate.mjs for older versions of geometry dash that did not come out on windows at the time

We do not have 2 files since there was no Windows release up until geometry dash 1.9.

so I had to translate generate.mjs to a langauge that
I knew better so that this code could be easily maintable.
Otherwise languages such as rust would've been considered
for use a long time ago.

"""

import os
import re
from itertools import chain
from typing import Union
try:
    # Try and see if we have the best json library in python
    from orjson import loads
except ModuleNotFoundError:
    # Workaround
    from json import loads

import click


def fix_gd_set(s: re.Match[str]):
    return f"gd::set<{re.match(r'(?<=std::set<)(.*?)(?=,)', s.group()).group(0)}"


def fix_gd_vector(s: re.Match[str]):
    # Seems to be a bug with just simply using re.match
    m = re.search(r"(?<=std::vector<)(.*?)(?=,)", s.group())
    return f"gd::vector<{m.group(1)}>"


def fix_gd_map(s: re.Match[str]):
    m = re.match(r"(?<=std::map<)(.*?),(.*?)(?=,)", s.group())
    return f"gd::map<{m.group(1)},{m.group(2)}>"

def fix_gd_unordered_map_1(s: re.Match[str]):
    m = re.match(r"(?<=std::unordered_map<)(.*?)(?=,)/)", s.group())
    return f"gd::unordered_map<{m.group(1)}, std::pair<double, double>>"


def fix_gd_unordered_map_2(s: re.Match[str]):
    m = re.match(r"(?<=std::unordered_map<)(.*?),(.*?)(?=,)", s.group())
    return f"gd::unordered_map<{m.group(1)},{m.group(2)}>"


REPLACEMENTS = {
    re.compile(
        r"^(public\: |private\: |protected\: |enum |class |struct |__thiscall |__cdecl )"
    ): "",
    re.compile(r" &"): "&",
    re.compile(r" \*"): "*",
    re.compile(r"\(void\)"): "()",
    re.compile(r"\)const "): ") const",
    re.compile(r"\,(?!\s)"): ", ",
    re.compile(
        r"std::basic_string\<char, std::char_traits<char>, std::allocator<char> ?>"
    ): "gd::string",
    re.compile(r"std::string"): "gd::string",
    re.compile(
        r"std::set<(.*?), std::less<(.*?)>, std::allocator<(.*?)> ?>"
    ): fix_gd_set,
    re.compile(r"std::vector<(.*?), std::allocator<(.*?)> ?>"): fix_gd_vector,
    re.compile(
        r"std::_Tree_const_iterator<std::_Tree_val<std::_Tree_simple_types<cocos2d::CCObject\*> ?> ?>"
    ): "cocos2d::CCSetIterator",
    re.compile(
        r"std::map<(.*?), (.*?), std::less<(.*?)>, std::allocator<std::pair<(.*?), (.*?)> ?> ?>"
    ): fix_gd_map,
    re.compile(
        r"std::unordered_map<(.*?), std::pair<double, double>, .*?> ?> ?> ?>"
    ): fix_gd_unordered_map_1,
    re.compile(r"std::unordered_map<(.*?), (.*?), .*?> ?> ?>"): fix_gd_unordered_map_2,
    re.compile(r"unsigned long long"): "uint64_t",
    re.compile(
        r"void \(cocos2d::CCObject::\*\)\(cocos2d::CCObject\*\)"
    ): "cocos2d::SEL_MenuHandler",
    re.compile(r"void \(cocos2d::CCObject::\*\)\(\)"): "cocos2d::SEL_CallFunc",
    re.compile(
        r"void \(cocos2d::CCObject::\*\)\(cocos2d::CCNode\*\)"
    ): "cocos2d::SEL_CallFuncN",
    re.compile(
        r"void \(cocos2d::CCObject::\*\)\(cocos2d::CCNode\*, void\*\)"
    ): "cocos2d::SEL_CallFuncND",
    re.compile(
        r"void \(cocos2d::CCObject::\*\)\(cocos2d::CCObject\*\)"
    ): "cocos2d::SEL_CallFuncO",
    re.compile(
        r"void \(cocos2d::CCObject::\*\)\(cocos2d::CCEvent\*\)"
    ): "cocos2d::SEL_EventHandler",
    re.compile(
        r"int \(cocos2d::CCObject::\*\)\(cocos2d::CCObject\*\)"
    ): "cocos2d::SEL_Compare",
    re.compile(
        r"void \(cocos2d::CCObject::\*\)\(cocos2d::extension::CCHttpClient\*, cocos2d::extension::CCHttpResponse\*\)"
    ): "cocos2d::extension::SEL_HttpResponse",
    re.compile(r"void \(cocos2d::CCObject::\*\)\(float\)"): "cocos2d::SEL_SCHEDULE",
    re.compile(r"cocos2d::_ccColor3B"): "cocos2d::ccColor3B",
    re.compile(r"cocos2d::_ccColor4B"): "cocos2d::ccColor4B",
    re.compile(r"cocos2d::_ccColor4F"): "cocos2d::ccColor4F",
    re.compile(r"cocos2d::_ccVertex2F"): "cocos2d::_ccVertex2F",
    re.compile(r"cocos2d::_ccVertex3F"): "cocos2d::_ccVertex3F",
    re.compile(r"cocos2d::_ccHSVValue"): "cocos2d::ccHSVValue",
}


def cleanFunctionSig(sig: str):
    for r, sub in REPLACEMENTS.items():
        sig = re.sub(r, sub, sig)
    return sig


# function shouldKeepSymbol(sym) {
# let keep = sym && sym.includes('::') && !sym.match(/(typeinfo|vtable|thunk|guard variable)/);
# if (!keep) return false;
# let className = sym.split('::')[0];
# keep = !className.match(/^(_JNIEnv|internal|tinyxml2|cocos2d|DS_Dictionary|ObjectDecoder|ObjectDecoderDelegate|pugi|__cxx|__gnu_cxx|std|fmt|llvm|tk|xml_|MD5|rtsha1)/);
# keep = keep && className !== "FMOD" && !sym.startsWith('FMOD_') && className != "tk";
# keep = keep && !enumClasses.includes(className);
# keep = keep && className != "CCContentManager";
# retu


def read_symbols(symbols_file: str):
    with open(symbols_file, "r") as r:
        for line in r:
            name = line.strip()
            if (
                "(" in name
                and ("__gun_cxx::" not in name)
                and not name.startswith("std::")
            ):
                yield cleanFunctionSig(name)


# Some of the internals we should be keeping such as
# fmt because we
# currently don't know what version of fmt robtop
# is using which is very annoying to have to try and
# figure out.

# DS_Dictionary Should be kept because It has not been properly reverse engineered
# properly by anybody that I know of yet...

enums = [
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
]


def shouldKeepSymbol(sym: str):
    # keep = "::" in sym and
    if ("::" not in sym) or (re.search(r"(typeinfo|vtable|thunk|guard variable)", sym)):
        return False
    # We don't have a cocos2d file...
    className = sym.split("::")[0]
    # There's many differences with gd 1.0 vs robtop's modern/modified cocos2d, so we may want to tweak these objects a little bit...
    if className == "cocos2d":
        return True

    # I might write in fmt into the bindings by hand or in a sperate file idk just yet how to approch this problem...
    elif className == "fmt":
        return False

    if className in enums:
        return False

    return not re.match(
        r"^(_JNIEnv|internal|tinyxml2|cocos2d|ObjectDecoder|ObjectDecoderDelegate|pugi|__cxx|__gnu_cxx|std|llvm|tk|xml_|MD5)",
        sym,
    )


def make_virutuals_table():
    # rb is a safety mechanism for unknown utf-8 characters
    with open("virtuals.json", "rb") as r:
        virutalsTable: dict[str, list[list[str]]] = loads(r.read())

    # python doesn't allow a dictionary to be edited during
    # it's own iteration so we need to copy the virtuals
    # Table so that we will be allowed to edit the virutals during iteration
    for name in virutalsTable.copy().keys():
        virutalsTable[name] = list(
            map(lambda x: list(map(cleanFunctionSig, x)), virutalsTable[name])
        )

    return virutalsTable


FILTER_FUNCTIONS = re.compile(r"((?:cocos2d::)?(\w+::)*\w+)::([\w~]+\(.*\))")


def shouldCommentOutFunction(className: str, name: str):
    baseClassName = className.split("::", -1)[0] if "::" in className else className

    return ("..." in name) or name.startswith(
        (
            f"{baseClassName}()",
            f"{baseClassName}({className} const&)",
            f"${baseClassName}(${className}&&)",
            f"~${baseClassName}",
            "fmt::",
        )
    )


def load_android_symbols(symbols_file: str):
    def filter_empty(e: Union[re.Match, None]):
        return e is not None

    classes: dict[str, dict[str, list[str]]] = {"GeometryDash.bro": {}}

    for s in filter(
        lambda x: not x.group(1).endswith("::"),
        filter(
            filter_empty,
            map(
                FILTER_FUNCTIONS.match,
                filter(shouldKeepSymbol, read_symbols(symbols_file)),
            ),
        ),
    ):
        groups = s.groups()
        if groups[0] == "cocos2d":
            continue
        # print()

        func = groups[-1]
        className = groups[0]
        if className not in classes["GeometryDash.bro"].keys():
            classes["GeometryDash.bro"][className] = []
        # We already cleaned out the function before so we can move on
        if func not in classes["GeometryDash.bro"][className]:
            classes["GeometryDash.bro"][className].append(func)

    virtualsTable = make_virutuals_table()

    for name in virtualsTable.copy().keys():
        if not (shouldKeepSymbol(f"{name}::init()")):
            continue
        tables = virtualsTable[name]
        pureVirts = list(
            set(filter(lambda x: x.startswith("pure_virtual_"), chain(*tables)))
        )
        if pureVirts:
            if not classes["GeometryDash.bro"].get(name):
                classes["GeometryDash.bro"][name] = []
            classes["GeometryDash.bro"][name].extend(pureVirts)

    return classes, virtualsTable


SIGS = []


def is_static_function(className: str, funcSig: str):
    """
    ```js
    function isStaticFunc(className, funcSig) {
        return funcSig.startsWith('create(')
        || className == 'GameToolbox'
        || funcSig == 'sharedState()'
        || funcSig == 'sharedEngine()'
        || funcSig == 'sharedDecoder()'
        || funcSig == 'sharedFontCache()'
        || funcSig == 'sharedSpriteFrameCache()'
    }
    ```
    """
    return (
        funcSig.startswith("create(")
        or (className == "GameToolBox")
        or funcSig
        in [
            "sharedState()",
            "sharedEngine()",
            "sharedDecoder()",
            "sharedFontCache()",
            "sharedSpriteFrameCache()",
        ]
    )

# virtual bool ccTouchBegan(CCTouch *pTouch, CCEvent *pEvent) {CC_UNUSED_PARAM(pTouch); CC_UNUSED_PARAM(pEvent); return false;};
#     // optional

#     virtual void ccTouchMoved(CCTouch *pTouch, CCEvent *pEvent) {CC_UNUSED_PARAM(pTouch); CC_UNUSED_PARAM(pEvent);}
#     virtual void ccTouchEnded(CCTouch *pTouch, CCEvent *pEvent) {CC_UNUSED_PARAM(pTouch); CC_UNUSED_PARAM(pEvent);}
#     virtual void ccTouchCancelled(CCTouch *pTouch, CCEvent *pEvent) {CC_UNUSED_PARAM(pTouch); CC_UNUSED_PARAM(pEvent);}

#     // optional
#      virtual void ccTouchesBegan(CCSet *pTouches, CCEvent *pEvent) {CC_UNUSED_PARAM(pTouches); CC_UNUSED_PARAM(pEvent);}
#      virtual void ccTouchesMoved(CCSet *pTouches, CCEvent *pEvent) {CC_UNUSED_PARAM(pTouches); CC_UNUSED_PARAM(pEvent);}
#      virtual void ccTouchesEnded(CCSet *pTouches, CCEvent *pEvent) {CC_UNUSED_PARAM(pTouches); CC_UNUSED_PARAM(pEvent);}
#      virtual void ccTouchesCancelled(CCSet *pTouches, CCEvent *pEvent) {CC_UNUSED_PARAM(pTouches); CC_UNUSED_PARAM(pEvent);}


def bestEffortGuess(className: str, name: str, classDict:dict[str, list[str]]):
    if name.startswith((f"{className}(", "~")):
        return name
    

    elif name.startswith(("ccTouchBegan(","ccTouchMoved(", "ccTouchEnded(", "ccTouchCancelled(")) and ("(cocos2d::CCTouch*, cocos2d::CCEvent*)") in name:
        return f"void {name[0:name.find('(')]}(cocos2d::CCTouch* pTouch, cocos2d::CCEvent* pEvent)"

    elif name.startswith(("ccTouchesBegan(","ccTouchesMoved(", "ccTouchesEnded(", "ccTouchesCancelled(")) and ("(cocos2d::CCTouch*, cocos2d::CCEvent*)") in name:
        return f"void {name[0:name.find('(')]}(cocos2d::CCSet* pTouches, cocos2d::CCEvent* pEvent)"



    elif name.startswith("create("):
        return f"{className}* {name}"

    elif name.startswith("init("):
        return f"bool {name}"

    elif re.match(r"^on[\w]+\(cocos2d::CCObject\*\)", name):
        return f"void {name[0:name.find('(')]}(cocos2d::CCObject* sender)"

    # HACK: TOO Risky for Reverse Enginneering so for now I will ignore this shit as
    # we don't even have informaton for libcocos or what robtop may have decided to fuck
    # up or around with in 1.0 or later up until 1.8/1.9 ...
    # if (isVirtual(className, name) && cocosVirtuals[name]) {
    #     return `${cocosVirtuals[name]} ${name}`
    # }

    if re.match(r"^set[A-Z]", name):
        # Extract variable and function name...
        
        if v := re.search(r"set([A-Z]\w+)\(([^\,\)]+)\)", name):
            variable_name = v.group(1)

            return f"void {name[0:name.find('(')]}({v.group(2)} {variable_name[0].lower() + variable_name[1:]})"
        else:
            # Workaround
            return f"void {name}"
        
    elif re.match(r"^is[A-Z]", name):
        return f"bool {name}"
    
    elif getStatement := re.match(r"^get([A-Z]\w+)", name):
        # Try a few logical Looks ups on name
        statement = getStatement.group(1)
        funcs = classDict[className]
        setFunc = ("set" + statement)
        for f in funcs:
            if f.startswith(setFunc + "("):
                # Set function should not be using more than one variable hence the comma check in my regex...
                v = re.search(r"set[A-Z]\w+\(([^\,\)]+)\)", f)
                if v and v.group(1):
                    return f"{v.group(1)} {name}"


    return f"TodoReturn {name}"


def vtableIndexForFunc(vtable: dict[str, list[list[str]]], name: str, funcSig: str):
    if not vtable.get(name):
        return

    tables = vtable[name]
    if not tables:
        return
    for table in tables:
        try:
            return table.index(funcSig)
        except ValueError:
            continue


def isVirtual(vtable: dict[str, list[list[str]]], name: str, funcSig: str):
    return vtableIndexForFunc(vtable, name, funcSig)


def groupForFunction(vtable: dict[str, list[list[str]]], className: str, funcSig: str):
    if funcSig.startswith((f"{className}(", "~")):
        return -3
    elif is_static_function(className, funcSig):
        return -2

    vi = vtableIndexForFunc(vtable, className, funcSig)
    if vi is None:
        return -1

    return [0, vi]


def sortFuncFor(getter):
    return lambda a, b: getter(a) - getter(b)


def reorderFuncs(vtable: dict[str, list[list[str]]], className: str, funcs: list[str]):
    out: dict[int, list[tuple[str, int]]] = {}
    for sig in funcs:
        index = groupForFunction(vtable, className, sig)

        if isinstance(index, list):
            group = 0
            order = index[1]
        else:
            group = index
            order = 0
        if not out.get(group):
            out[group] = []
        out[group].append((sig, order))

    new_funcs: dict[int, list[list[str]]] = {}
    for k, v in sorted(out.items()):
        # Sort by name and then by order
        inner_dict: dict[int, list[str]] = {}
        for func, pos in v:
            if not inner_dict.get(pos):
                inner_dict[pos] = [func]
            else:
                inner_dict[pos].append(func)

        if not new_funcs.get(k):
            new_funcs[k] = []

        for _, v in sorted(inner_dict.items()):
            v.sort()
            new_funcs[k].append(v)
    return new_funcs


@click.command
@click.argument(
    "android_symbols", type=click.Path(exists=True, file_okay=True, readable=True)
)
def cli(android_symbols: str):
    """Meant to make geometry dash bindings for 1.8 or earlier version of the game,
    this is a small modification over generate.mjs found in the geode-sdk/bindings repository

    be sure to run llvm-nm -gDCj <ELFFILE>


    """
    print(f"Loading Android symbols from {android_symbols}")
    classes, virtualTable = load_android_symbols(android_symbols)
    print(f"Writing Results...")
    res = {"GeometryDash.bro": "// clang-format off\n\n"}
    # print(classes)
    # Will use this later for looking up setAttrs...
    classesDict = classes["GeometryDash.bro"]

    for name, v in classes["GeometryDash.bro"].items():
        funcsOut = []
        funcs = reorderFuncs(virtualTable, name, v)
        for _, groups in sorted(funcs.items()):
            for group in groups:
                for func in group:
                    fullSig = bestEffortGuess(name, func, classesDict)
                    if isVirtual(virtualTable, name, func):
                        fullSig = "virtual " + fullSig
                    elif is_static_function(name, func):
                        fullSig + "static " + fullSig
                    if func.startswith("pure_virtual_"):
                        fullSig += " {} // TODO: figure out what function this is"
                    else:
                        fullSig += ";"

                    if shouldCommentOutFunction(name, func):
                        fullSig = "// " + fullSig

                    funcsOut.append("    " + fullSig)

                funcsOut.append(" ")

        if funcsOut:
            funcsOut.pop()

        if funcsOut:
            funcTxt = "\n".join(funcsOut)
            res["GeometryDash.bro"] += "[[link(android)]]\nclass %s {\n%s\n\n}\n\n" % (
                name,
                funcTxt,
            )

    if not os.path.exists("out"):
        os.mkdir("out")

    for k, v in res.items():
        print(f'writing "out\{k}"')
        with open(os.path.join("out", k), "w") as w:
            w.write(v)
    print("done")


if __name__ == "__main__":
    cli()

# Only 1 depedency is required and that is click, I made orjson optional.
