use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct GitmojiType {
    pub emoji: &'static str,
    pub code: &'static str,
    pub description: &'static str,
    pub name: &'static str,
}

impl fmt::Display for GitmojiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.emoji, self.description)
    }
}

impl GitmojiType {
    pub fn all() -> Vec<&'static GitmojiType> {
        GITMOJIS.iter().collect()
    }

    pub fn from_code(code: &str) -> Option<&'static GitmojiType> {
        GITMOJIS.iter().find(|g| g.code == code)
    }

    pub fn from_emoji(emoji: &str) -> Option<&'static GitmojiType> {
        GITMOJIS.iter().find(|g| g.emoji == emoji)
    }
}

static GITMOJIS: &[GitmojiType] = &[
    GitmojiType {
        emoji: "🎨",
        code: ":art:",
        description: "Improve structure / format of the code.",
        name: "art",
    },
    GitmojiType {
        emoji: "⚡️",
        code: ":zap:",
        description: "Improve performance.",
        name: "zap",
    },
    GitmojiType {
        emoji: "🔥",
        code: ":fire:",
        description: "Remove code or files.",
        name: "fire",
    },
    GitmojiType {
        emoji: "🐛",
        code: ":bug:",
        description: "Fix a bug.",
        name: "bug",
    },
    GitmojiType {
        emoji: "🚑️",
        code: ":ambulance:",
        description: "Critical hotfix.",
        name: "ambulance",
    },
    GitmojiType {
        emoji: "✨",
        code: ":sparkles:",
        description: "Introduce new features.",
        name: "sparkles",
    },
    GitmojiType {
        emoji: "📝",
        code: ":memo:",
        description: "Add or update documentation.",
        name: "memo",
    },
    GitmojiType {
        emoji: "🚀",
        code: ":rocket:",
        description: "Deploy stuff.",
        name: "rocket",
    },
    GitmojiType {
        emoji: "💄",
        code: ":lipstick:",
        description: "Add or update the UI and style files.",
        name: "lipstick",
    },
    GitmojiType {
        emoji: "🎉",
        code: ":tada:",
        description: "Begin a project.",
        name: "tada",
    },
    GitmojiType {
        emoji: "✅",
        code: ":white_check_mark:",
        description: "Add, update, or pass tests.",
        name: "white-check-mark",
    },
    GitmojiType {
        emoji: "🔒️",
        code: ":lock:",
        description: "Fix security or privacy issues.",
        name: "lock",
    },
    GitmojiType {
        emoji: "🔐",
        code: ":closed_lock_with_key:",
        description: "Add or update secrets.",
        name: "closed-lock-with-key",
    },
    GitmojiType {
        emoji: "🔖",
        code: ":bookmark:",
        description: "Release / Version tags.",
        name: "bookmark",
    },
    GitmojiType {
        emoji: "🚨",
        code: ":rotating_light:",
        description: "Fix compiler / linter warnings.",
        name: "rotating-light",
    },
    GitmojiType {
        emoji: "🚧",
        code: ":construction:",
        description: "Work in progress.",
        name: "construction",
    },
    GitmojiType {
        emoji: "💚",
        code: ":green_heart:",
        description: "Fix CI Build.",
        name: "green-heart",
    },
    GitmojiType {
        emoji: "⬇️",
        code: ":arrow_down:",
        description: "Downgrade dependencies.",
        name: "arrow-down",
    },
    GitmojiType {
        emoji: "⬆️",
        code: ":arrow_up:",
        description: "Upgrade dependencies.",
        name: "arrow-up",
    },
    GitmojiType {
        emoji: "📌",
        code: ":pushpin:",
        description: "Pin dependencies to specific versions.",
        name: "pushpin",
    },
    GitmojiType {
        emoji: "👷",
        code: ":construction_worker:",
        description: "Add or update CI build system.",
        name: "construction-worker",
    },
    GitmojiType {
        emoji: "📈",
        code: ":chart_with_upwards_trend:",
        description: "Add or update analytics or track code.",
        name: "chart-with-upwards-trend",
    },
    GitmojiType {
        emoji: "♻️",
        code: ":recycle:",
        description: "Refactor code.",
        name: "recycle",
    },
    GitmojiType {
        emoji: "➕",
        code: ":heavy_plus_sign:",
        description: "Add a dependency.",
        name: "heavy-plus-sign",
    },
    GitmojiType {
        emoji: "➖",
        code: ":heavy_minus_sign:",
        description: "Remove a dependency.",
        name: "heavy-minus-sign",
    },
    GitmojiType {
        emoji: "🔧",
        code: ":wrench:",
        description: "Add or update configuration files.",
        name: "wrench",
    },
    GitmojiType {
        emoji: "🔨",
        code: ":hammer:",
        description: "Add or update development scripts.",
        name: "hammer",
    },
    GitmojiType {
        emoji: "🌐",
        code: ":globe_with_meridians:",
        description: "Internationalization and localization.",
        name: "globe-with-meridians",
    },
    GitmojiType {
        emoji: "✏️",
        code: ":pencil2:",
        description: "Fix typos.",
        name: "pencil2",
    },
    GitmojiType {
        emoji: "💩",
        code: ":poop:",
        description: "Write bad code that needs to be improved.",
        name: "poop",
    },
    GitmojiType {
        emoji: "⏪️",
        code: ":rewind:",
        description: "Revert changes.",
        name: "rewind",
    },
    GitmojiType {
        emoji: "🔀",
        code: ":twisted_rightwards_arrows:",
        description: "Merge branches.",
        name: "twisted-rightwards-arrows",
    },
    GitmojiType {
        emoji: "📦️",
        code: ":package:",
        description: "Add or update compiled files or packages.",
        name: "package",
    },
    GitmojiType {
        emoji: "👽️",
        code: ":alien:",
        description: "Update code due to external API changes.",
        name: "alien",
    },
    GitmojiType {
        emoji: "🚚",
        code: ":truck:",
        description: "Move or rename resources (e.g.: files, paths, routes).",
        name: "truck",
    },
    GitmojiType {
        emoji: "📄",
        code: ":page_facing_up:",
        description: "Add or update license.",
        name: "page-facing-up",
    },
    GitmojiType {
        emoji: "💥",
        code: ":boom:",
        description: "Introduce breaking changes.",
        name: "boom",
    },
    GitmojiType {
        emoji: "🍱",
        code: ":bento:",
        description: "Add or update assets.",
        name: "bento",
    },
    GitmojiType {
        emoji: "♿️",
        code: ":wheelchair:",
        description: "Improve accessibility.",
        name: "wheelchair",
    },
    GitmojiType {
        emoji: "💡",
        code: ":bulb:",
        description: "Add or update comments in source code.",
        name: "bulb",
    },
    GitmojiType {
        emoji: "🍻",
        code: ":beers:",
        description: "Write code drunkenly.",
        name: "beers",
    },
    GitmojiType {
        emoji: "💬",
        code: ":speech_balloon:",
        description: "Add or update text and literals.",
        name: "speech-balloon",
    },
    GitmojiType {
        emoji: "🗃️",
        code: ":card_file_box:",
        description: "Perform database related changes.",
        name: "card-file-box",
    },
    GitmojiType {
        emoji: "🔊",
        code: ":loud_sound:",
        description: "Add or update logs.",
        name: "loud-sound",
    },
    GitmojiType {
        emoji: "🔇",
        code: ":mute:",
        description: "Remove logs.",
        name: "mute",
    },
    GitmojiType {
        emoji: "👥",
        code: ":busts_in_silhouette:",
        description: "Add or update contributor(s).",
        name: "busts-in-silhouette",
    },
    GitmojiType {
        emoji: "🚸",
        code: ":children_crossing:",
        description: "Improve user experience / usability.",
        name: "children-crossing",
    },
    GitmojiType {
        emoji: "🏗️",
        code: ":building_construction:",
        description: "Make architectural changes.",
        name: "building-construction",
    },
    GitmojiType {
        emoji: "📱",
        code: ":iphone:",
        description: "Work on responsive design.",
        name: "iphone",
    },
    GitmojiType {
        emoji: "🤡",
        code: ":clown_face:",
        description: "Mock things.",
        name: "clown-face",
    },
    GitmojiType {
        emoji: "🥚",
        code: ":egg:",
        description: "Add or update an easter egg.",
        name: "egg",
    },
    GitmojiType {
        emoji: "🙈",
        code: ":see_no_evil:",
        description: "Add or update a .gitignore file.",
        name: "see-no-evil",
    },
    GitmojiType {
        emoji: "📸",
        code: ":camera_flash:",
        description: "Add or update snapshots.",
        name: "camera-flash",
    },
    GitmojiType {
        emoji: "⚗️",
        code: ":alembic:",
        description: "Perform experiments.",
        name: "alembic",
    },
    GitmojiType {
        emoji: "🔍️",
        code: ":mag:",
        description: "Improve SEO.",
        name: "mag",
    },
    GitmojiType {
        emoji: "🏷️",
        code: ":label:",
        description: "Add or update types.",
        name: "label",
    },
    GitmojiType {
        emoji: "🌱",
        code: ":seedling:",
        description: "Add or update seed files.",
        name: "seedling",
    },
    GitmojiType {
        emoji: "🚩",
        code: ":triangular_flag_on_post:",
        description: "Add, update, or remove feature flags.",
        name: "triangular-flag-on-post",
    },
    GitmojiType {
        emoji: "🥅",
        code: ":goal_net:",
        description: "Catch errors.",
        name: "goal-net",
    },
    GitmojiType {
        emoji: "💫",
        code: ":dizzy:",
        description: "Add or update animations and transitions.",
        name: "dizzy",
    },
    GitmojiType {
        emoji: "🗑️",
        code: ":wastebasket:",
        description: "Deprecate code that needs to be cleaned up.",
        name: "wastebasket",
    },
    GitmojiType {
        emoji: "🛂",
        code: ":passport_control:",
        description: "Work on code related to authorization, roles and permissions.",
        name: "passport-control",
    },
    GitmojiType {
        emoji: "🩹",
        code: ":adhesive_bandage:",
        description: "Simple fix for a non-critical issue.",
        name: "adhesive-bandage",
    },
    GitmojiType {
        emoji: "🧐",
        code: ":monocle_face:",
        description: "Data exploration/inspection.",
        name: "monocle-face",
    },
    GitmojiType {
        emoji: "⚰️",
        code: ":coffin:",
        description: "Remove dead code.",
        name: "coffin",
    },
    GitmojiType {
        emoji: "🧪",
        code: ":test_tube:",
        description: "Add a failing test.",
        name: "test-tube",
    },
    GitmojiType {
        emoji: "👔",
        code: ":necktie:",
        description: "Add or update business logic.",
        name: "necktie",
    },
    GitmojiType {
        emoji: "🩺",
        code: ":stethoscope:",
        description: "Add or update healthcheck.",
        name: "stethoscope",
    },
    GitmojiType {
        emoji: "🧱",
        code: ":bricks:",
        description: "Infrastructure related changes.",
        name: "bricks",
    },
    GitmojiType {
        emoji: "🧑‍💻",
        code: ":technologist:",
        description: "Improve developer experience.",
        name: "technologist",
    },
    GitmojiType {
        emoji: "💸",
        code: ":money_with_wings:",
        description: "Add sponsorships or money related infrastructure.",
        name: "money-with-wings",
    },
    GitmojiType {
        emoji: "🧵",
        code: ":thread:",
        description: "Add or update code related to multithreading or concurrency.",
        name: "thread",
    },
    GitmojiType {
        emoji: "🦺",
        code: ":safety_vest:",
        description: "Add or update code related to validation.",
        name: "safety-vest",
    },
    GitmojiType {
        emoji: "✈️",
        code: ":airplane:",
        description: "Improve offline support.",
        name: "airplane",
    },
    GitmojiType {
        emoji: "🦖",
        code: ":t-rex:",
        description: "Code that adds backwards compatibility.",
        name: "t-rex",
    },
];
