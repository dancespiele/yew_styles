use yew::prelude::*;
use yew_assets::{
    browser_assets::{BrowserAssets, BrowserIcon},
    business_assets::{BusinessAssets, BusinessIcon},
    communication_assets::{CommunicationAssets, CommunicationIcon},
    controller_assets::{ControllerAssets, ControllerIcon},
    dev_assets::{DevAssets, DevIcon},
    device_assets::{DeviceAssets, DeviceIcon},
    editing_assets::{EditingAssets, EditingIcon},
    env_assets::{EnvAssets, EnvIcon},
    file_assets::{FileAssets, FileIcon},
    info_assets::{InfoAssets, InfoIcon},
    multimedia_assets::{MultimediaAssets, MultimediaIcon},
    nav_assets::{NavAssets, NavIcon},
    object_assets::{ObjectAssets, ObjectIcon},
    social_assets::{SocialAssets, SocialIcon},
    ux_assets::{UxAssets, UxIcon},
};
use yew_prism::Prism;
use yew_styles::layouts::{
    container::{Container, Direction, JustifyContent, Mode, Wrap},
    item::{Item, ItemLayout},
};

pub struct AssetsPage;

impl Component for AssetsPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        AssetsPage {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let code_example = "<UxAssets
    icon = UxIcon::Activity
    fill = \"#fff\"
    size = (\"30\".to_string(),\"30\".to_string())
/>";

        html! {
            <div class="content">
                <h1>{"Yew Assets"}</h1>
                    <p>{"Yew assets is an external library for assets which you can use in "}<code>{"yew"}</code>{" projects. You can read more "}<a href="https://github.com/spielrs/yew_assets" target="_blank">{"here"}</a></p>
                <h2>{"Features"}</h2>
                <span>
                    {"browser_assets, business_assets, communication_assets, controller_assets, dev_assets, device_assets, editing_assets, env_assets, file_assets, info_assets, multimedia_assets, nav_assets, object_assets, social_assets, ux_assets"}
                </span>
                <h2>{"Code example"}</h2>
                <Prism
                    code=code_example
                    language="rust"
                />
                <h2>{"Propeties"}</h2>
                <ul>
                    <li><b>{"icon: "}</b>{"icon to show. Options included in "}<code>{"[name asset]Icon"}</code>{". Required."}</li>
                    <li><b>{"size: "}</b>{"size of the button (width, heigth). Default "}<code>{"(\"24\".to_string(), \"24\".to_string())"}</code>{"."}</li>
                    <li><b>{"view_box: "}</b>{"Defines the position and dimension of the icon. Default "}
                        <code>{"(\"0\".to_string(),\"0\".to_string(),\"24\".to_string(),\"24\".to_string())"}</code>{"."}</li>
                    <li><b>{"fill: "}</b>{"fill the color of the icon."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>
                <h2>{"Sources"}</h2>
                <span>{"The svgs are created by "}<a href="https://feathericons.com/" target="_blank">{"feather community"}</a>{" and all of them have the most permissive license (MIT)"}</span>
                <h2>{"Visual examples"}</h2>
                <Container
                    wrap=Wrap::Wrap
                    direction=Direction::Row
                    justify_content=JustifyContent::Center(Mode::NoMode)>
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Browser assets"}</h3>
                    </Item>
                    {get_browser_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Business assets"}</h3>
                    </Item>
                    {get_business_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Communication assets"}</h3>
                    </Item>
                    {get_communication_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Controller assets"}</h3>
                    </Item>
                    {get_controller_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Dev assets"}</h3>
                    </Item>
                    {get_dev_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Device assets"}</h3>
                    </Item>
                    {get_device_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Editing assets"}</h3>
                    </Item>
                    {get_editing_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Env assets"}</h3>
                    </Item>
                    {get_env_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"File assets"}</h3>
                    </Item>
                    {get_file_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Info assets"}</h3>
                    </Item>
                    {get_info_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Multimedia assets"}</h3>
                    </Item>
                    {get_multimedia_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Nav assets"}</h3>
                    </Item>
                    {get_nav_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Object assets"}</h3>
                    </Item>
                    {get_object_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Social assets"}</h3>
                    </Item>
                    {get_social_assets()}
                    <Item layouts=vec![ItemLayout::ItXs(12)]>
                        <h3>{"Ux assets"}</h3>
                    </Item>
                    {get_ux_assets()}
                </Container>
            </div>
        }
    }
}

fn get_browser_assets() -> Html {
    let browser_icons = vec![BrowserIcon::Compass, BrowserIcon::Chrome];
    let browser_names = vec!["Compass", "Chrome"];
    browser_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <BrowserAssets
                        icon = icon.clone()
                    />
                    <div>{browser_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_business_assets() -> Html {
    let business_icons = vec![
        BusinessIcon::DollarSign,
        BusinessIcon::Target,
        BusinessIcon::BarChart,
        BusinessIcon::CreditCard,
        BusinessIcon::TrendingDown,
        BusinessIcon::Percent,
        BusinessIcon::BarChart2,
        BusinessIcon::PieChart,
        BusinessIcon::TrendingUp,
        BusinessIcon::Award,
    ];

    let business_names = vec![
        "DollarSign",
        "Target",
        "BarChart",
        "CreditCard",
        "TrendingDown",
        "Percent",
        "BarChart2",
        "PieChart",
        "TrendingUp",
        "Award",
    ];

    business_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <BusinessAssets
                        icon = icon.clone()
                    />
                    <div>{business_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_communication_assets() -> Html {
    let communication_icons = vec![
        CommunicationIcon::Users,
        CommunicationIcon::User,
        CommunicationIcon::UserPlus,
        CommunicationIcon::Frown,
        CommunicationIcon::PhoneMissed,
        CommunicationIcon::PhoneCall,
        CommunicationIcon::UserX,
        CommunicationIcon::PhoneOff,
        CommunicationIcon::Star,
        CommunicationIcon::UserCheck,
        CommunicationIcon::Meh,
        CommunicationIcon::PhoneOutgoing,
        CommunicationIcon::Smile,
        CommunicationIcon::Bluetooth,
        CommunicationIcon::UserMinus,
        CommunicationIcon::Voicemail,
        CommunicationIcon::PhoneIncoming,
        CommunicationIcon::Phone,
        CommunicationIcon::WifiOff,
        CommunicationIcon::Mail,
        CommunicationIcon::MessageCircle,
        CommunicationIcon::PhoneForwarded,
        CommunicationIcon::Heart,
        CommunicationIcon::MessageSquare,
        CommunicationIcon::Wifi,
    ];

    let communication_names = vec![
        "Users",
        "User",
        "UserPlus",
        "Frown",
        "PhoneMissed",
        "PhoneCall",
        "UserX",
        "PhoneOff",
        "Star",
        "UserCheck",
        "Meh",
        "PhoneOutgoing",
        "Smile",
        "Bluetooth",
        "UserMinus",
        "Voicemail",
        "PhoneIncoming",
        "Phone",
        "WifiOff",
        "Mail",
        "MessageCircle",
        "PhoneForwarded",
        "Heart",
        "MessageSquare",
        "Wifi",
    ];

    communication_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <CommunicationAssets
                        icon = icon.clone()
                    />
                    <div>{communication_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_controller_assets() -> Html {
    let controller_icons = vec![
        ControllerIcon::ArrowRightCircle,
        ControllerIcon::CornerLeftDown,
        ControllerIcon::ArrowRight,
        ControllerIcon::PauseCircle,
        ControllerIcon::ArrowLeft,
        ControllerIcon::ChevronDown,
        ControllerIcon::StopCircle,
        ControllerIcon::Volume2,
        ControllerIcon::ArrowDownCircle,
        ControllerIcon::RotateCcw,
        ControllerIcon::Volume,
        ControllerIcon::CornerDownLeft,
        ControllerIcon::CornerRightDown,
        ControllerIcon::RefreshCcw,
        ControllerIcon::CornerDownRight,
        ControllerIcon::ArrowLeftCircle,
        ControllerIcon::MicOff,
        ControllerIcon::ArrowUpLeft,
        ControllerIcon::ArrowDownLeft,
        ControllerIcon::ArrowDown,
        ControllerIcon::ChevronRight,
        ControllerIcon::ChevronsDown,
        ControllerIcon::Power,
        ControllerIcon::SkipForward,
        ControllerIcon::Rewind,
        ControllerIcon::Pause,
        ControllerIcon::Volume1,
        ControllerIcon::CornerUpLeft,
        ControllerIcon::ChevronUp,
        ControllerIcon::CornerRightUp,
        ControllerIcon::ArrowDownRight,
        ControllerIcon::CornerLeftUp,
        ControllerIcon::ArrowUp,
        ControllerIcon::Repeat,
        ControllerIcon::Play,
        ControllerIcon::ChevronsLeft,
        ControllerIcon::PlayCircle,
        ControllerIcon::ArrowUpCircle,
        ControllerIcon::ChevronLeft,
        ControllerIcon::FastForward,
        ControllerIcon::Mic,
        ControllerIcon::ChevronsRight,
        ControllerIcon::SkipBack,
        ControllerIcon::ArrowUpRight,
        ControllerIcon::CornerUpRight,
        ControllerIcon::VolumeX,
        ControllerIcon::ChevronsUp,
    ];

    let controller_names = vec![
        "ArrowRightCircle",
        "CornerLeftDown",
        "ArrowRight",
        "PauseCircle",
        "ArrowLeft",
        "ChevronDown",
        "StopCircle",
        "Volume2",
        "ArrowDownCircle",
        "RotateCcw",
        "Volume",
        "CornerDownLeft",
        "CornerRightDown",
        "RefreshCcw",
        "CornerDownRight",
        "ArrowLeftCircle",
        "MicOff",
        "ArrowUpLeft",
        "ArrowDownLeft",
        "ArrowDown",
        "ChevronRight",
        "ChevronsDown",
        "Power",
        "SkipForward",
        "Rewind",
        "Pause",
        "Volume1",
        "CornerUpLeft",
        "ChevronUp",
        "CornerRightUp",
        "ArrowDownRight",
        "CornerLeftUp",
        "ArrowUp",
        "Repeat",
        "Play",
        "ChevronsLeft",
        "PlayCircle",
        "ArrowUpCircle",
        "ChevronLeft",
        "FastForward",
        "Mic",
        "ChevronsRight",
        "SkipBack",
        "ArrowUpRight",
        "CornerUpRight",
        "VolumeX",
        "ChevronsUp",
    ];

    controller_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <ControllerAssets
                        icon = icon.clone()
                    />
                    <div>{controller_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_dev_assets() -> Html {
    let dev_icons = vec![
        DevIcon::Database,
        DevIcon::GitBranch,
        DevIcon::Feather,
        DevIcon::Cpu,
        DevIcon::GitPullRequest,
        DevIcon::Github,
        DevIcon::Codesandbox,
        DevIcon::Server,
        DevIcon::GitCommit,
        DevIcon::Figma,
        DevIcon::Code,
        DevIcon::Gitlab,
        DevIcon::Droplet,
        DevIcon::Trello,
        DevIcon::Codepen,
        DevIcon::Terminal,
        DevIcon::GitMerge,
        DevIcon::Framer,
        DevIcon::Command,
    ];

    let dev_names = vec![
        "Database",
        "GitBranch",
        "Feather",
        "Cpu",
        "GitPullRequest",
        "Github",
        "Codesandbox",
        "Server",
        "GitCommit",
        "Figma",
        "Code",
        "Gitlab",
        "Droplet",
        "Trello",
        "Codepen",
        "Terminal",
        "GitMerge",
        "Framer",
        "Command",
    ];

    dev_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <DevAssets
                        icon = icon.clone()
                    />
                    <div>{dev_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_device_assets() -> Html {
    let device_icons = vec![
        DeviceIcon::HardDrive,
        DeviceIcon::Tv,
        DeviceIcon::CameraOff,
        DeviceIcon::BatteryCharging,
        DeviceIcon::Battery,
        DeviceIcon::Monitor,
        DeviceIcon::Printer,
        DeviceIcon::Cast,
        DeviceIcon::Tablet,
        DeviceIcon::Speaker,
        DeviceIcon::Camera,
        DeviceIcon::Watch,
        DeviceIcon::Smartphone,
    ];

    let device_names = vec![
        "HardDrive",
        "Tv",
        "CameraOff",
        "BatteryCharging",
        "Battery",
        "Monitor",
        "Printer",
        "Cast",
        "Tablet",
        "Speaker",
        "Camera",
        "Watch",
        "Smartphone",
    ];

    device_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <DeviceAssets
                        icon = icon.clone()
                    />
                    <div>{device_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_editing_assets() -> Html {
    let editing_icons = vec![
        EditingIcon::XCircle,
        EditingIcon::Crop,
        EditingIcon::Type,
        EditingIcon::Minimize2,
        EditingIcon::CheckCircle,
        EditingIcon::ZapOff,
        EditingIcon::Trash2,
        EditingIcon::MinusSquare,
        EditingIcon::AlignRight,
        EditingIcon::Bold,
        EditingIcon::X,
        EditingIcon::Italic,
        EditingIcon::XSquare,
        EditingIcon::Underline,
        EditingIcon::PlusSquare,
        EditingIcon::Minus,
        EditingIcon::Scissors,
        EditingIcon::ZoomIn,
        EditingIcon::Edit2,
        EditingIcon::Maximize2,
        EditingIcon::Edit,
        EditingIcon::AlignJustify,
        EditingIcon::List,
        EditingIcon::Delete,
        EditingIcon::ZoomOut,
        EditingIcon::XOctagon,
        EditingIcon::Minimize,
        EditingIcon::Save,
        EditingIcon::AlignLeft,
        EditingIcon::Zap,
        EditingIcon::MinusCircle,
        EditingIcon::CheckSquare,
        EditingIcon::AlignCenter,
        EditingIcon::Move,
        EditingIcon::Copy,
        EditingIcon::Trash,
        EditingIcon::Maximize,
        EditingIcon::Plus,
        EditingIcon::Check,
        EditingIcon::PlusCircle,
        EditingIcon::Edit3,
    ];

    let editing_names = vec![
        "XCircle",
        "Crop",
        "Type",
        "Minimize2",
        "CheckCircle",
        "ZapOff",
        "Trash2",
        "MinusSquare",
        "AlignRight",
        "Bold",
        "X",
        "Italic",
        "XSquare",
        "Underline",
        "PlusSquare",
        "Minus",
        "Scissors",
        "ZoomIn",
        "Edit2",
        "Maximize2",
        "Edit",
        "AlignJustify",
        "List",
        "Delete",
        "ZoomOut",
        "XOctagon",
        "Minimize",
        "Save",
        "AlignLeft",
        "Zap",
        "MinusCircle",
        "CheckSquare",
        "AlignCenter",
        "Move",
        "Copy",
        "Trash",
        "Maximize",
        "Plus",
        "Check",
        "PlusCircle",
        "Edit3",
    ];

    editing_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <EditingAssets
                        icon = icon.clone()
                    />
                    <div>{editing_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_env_assets() -> Html {
    let env_icons = vec![
        EnvIcon::Umbrella,
        EnvIcon::Cloud,
        EnvIcon::CloudLightning,
        EnvIcon::CloudOff,
        EnvIcon::CloudDrizzle,
        EnvIcon::Calendar,
        EnvIcon::Sunrise,
        EnvIcon::Clock,
        EnvIcon::Sunset,
        EnvIcon::CloudRain,
        EnvIcon::CloudSnow,
        EnvIcon::Wind,
        EnvIcon::Moon,
        EnvIcon::Thermometer,
        EnvIcon::Sun,
    ];

    let env_names = vec![
        "Umbrella",
        "Cloud",
        "CloudLightning",
        "CloudOff",
        "CloudDrizzle",
        "Calendar",
        "Sunrise",
        "Clock",
        "Sunset",
        "CloudRain",
        "CloudSnow",
        "Wind",
        "Moon",
        "Thermometer",
        "Sun",
    ];

    env_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <EnvAssets
                        icon = icon.clone()
                    />
                    <div>{env_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_file_assets() -> Html {
    let file_icons = vec![
        FileIcon::FileMinus,
        FileIcon::Image,
        FileIcon::FileText,
        FileIcon::FilePlus,
        FileIcon::File,
        FileIcon::FolderMinus,
        FileIcon::FolderPlus,
        FileIcon::Folder,
    ];

    let file_names = vec![
        "FileMinus",
        "Image",
        "FileText",
        "FilePlus",
        "File",
        "FolderMinus",
        "FolderPlus",
        "Folder",
    ];

    file_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <FileAssets
                        icon = icon.clone()
                    />
                    <div>{file_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_info_assets() -> Html {
    let info_icons = vec![
        InfoIcon::AlertCircle,
        InfoIcon::AlertTriangle,
        InfoIcon::Info,
        InfoIcon::AlertOctagon,
        InfoIcon::Aperture,
        InfoIcon::HelpCircle,
    ];

    let info_names = vec![
        "AlertCircle",
        "AlertTriangle",
        "Info",
        "AlertOctagon",
        "Aperture",
        "HelpCircle",
    ];

    info_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <InfoAssets
                        icon = icon.clone()
                    />
                    <div>{info_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_multimedia_assets() -> Html {
    let multimedia_icons = vec![
        MultimediaIcon::Video,
        MultimediaIcon::Film,
        MultimediaIcon::Radio,
        MultimediaIcon::Headphones,
        MultimediaIcon::Music,
        MultimediaIcon::VideoOff,
        MultimediaIcon::Youtube,
        MultimediaIcon::Airplay,
    ];

    let multimedia_names = vec![
        "Video",
        "Film",
        "Radio",
        "Headphones",
        "Music",
        "VideoOff",
        "Youtube",
        "Airplay",
    ];

    multimedia_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <MultimediaAssets
                        icon = icon.clone()
                    />
                    <div>{multimedia_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_nav_assets() -> Html {
    let nav_icons = vec![
        NavIcon::Flag,
        NavIcon::Navigation,
        NavIcon::Upload,
        NavIcon::Map,
        NavIcon::MapPin,
        NavIcon::ExternalLink,
        NavIcon::Download,
        NavIcon::Share2,
        NavIcon::Share,
        NavIcon::Navigation2,
        NavIcon::Inbox,
        NavIcon::UploadCloud,
        NavIcon::DownloadCloud,
        NavIcon::Send,
        NavIcon::AtSign,
    ];

    let nav_names = vec![
        "Flag",
        "Navigation",
        "Upload",
        "Map",
        "MapPin",
        "ExternalLink",
        "Download",
        "Share2",
        "Share",
        "Navigation2",
        "Inbox",
        "UploadCloud",
        "DownloadCloud",
        "Send",
        "AtSign",
    ];

    nav_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <NavAssets
                        icon = icon.clone()
                    />
                    <div>{nav_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_object_assets() -> Html {
    let object_icons = vec![
        ObjectIcon::Square,
        ObjectIcon::Briefcase,
        ObjectIcon::Box,
        ObjectIcon::Anchor,
        ObjectIcon::Paperclip,
        ObjectIcon::Triangle,
        ObjectIcon::Gift,
        ObjectIcon::Truck,
        ObjectIcon::PenTool,
        ObjectIcon::Book,
        ObjectIcon::Hexagon,
        ObjectIcon::Coffee,
        ObjectIcon::Disc,
        ObjectIcon::LifeBuoy,
        ObjectIcon::Key,
        ObjectIcon::Package,
        ObjectIcon::Globe,
        ObjectIcon::Octagon,
        ObjectIcon::Circle,
    ];

    let object_names = vec![
        "Square",
        "Briefcase",
        "Box",
        "Anchor",
        "Paperclip",
        "Triangle",
        "Gift",
        "Truck",
        "PenTool",
        "Book",
        "Hexagon",
        "Coffee",
        "Disc",
        "LifeBuoy",
        "Key",
        "Package",
        "Globe",
        "Octagon",
        "Circle",
    ];

    object_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <ObjectAssets
                        icon = icon.clone()
                    />
                    <div>{object_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_social_assets() -> Html {
    let social_icons = vec![
        SocialIcon::ThumbsUp,
        SocialIcon::Twitch,
        SocialIcon::Instagram,
        SocialIcon::Slack,
        SocialIcon::Hash,
        SocialIcon::Linkedin,
        SocialIcon::Twitter,
        SocialIcon::ThumbsDown,
        SocialIcon::Facebook,
        SocialIcon::Rss,
        SocialIcon::Slash,
    ];

    let social_names = vec![
        "ThumbsUp",
        "Twitch",
        "Instagram",
        "Slack",
        "Hash",
        "Linkedin",
        "Twitter",
        "ThumbsDown",
        "Facebook",
        "Rss",
        "Slash",
    ];

    social_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <SocialAssets
                        icon = icon.clone()
                    />
                    <div>{social_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}

fn get_ux_assets() -> Html {
    let ux_icons = vec![
        UxIcon::ShieldOff,
        UxIcon::Archive,
        UxIcon::Activity,
        UxIcon::Shield,
        UxIcon::Crosshair,
        UxIcon::BellOff,
        UxIcon::EyeOff,
        UxIcon::Sidebar,
        UxIcon::MoreVertical,
        UxIcon::Bell,
        UxIcon::RefreshCw,
        UxIcon::Clipboard,
        UxIcon::Layout,
        UxIcon::Loader,
        UxIcon::Grid,
        UxIcon::ToggleLeft,
        UxIcon::Sliders,
        UxIcon::Settings,
        UxIcon::Eye,
        UxIcon::Home,
        UxIcon::Link,
        UxIcon::LogIn,
        UxIcon::Menu,
        UxIcon::RotateCw,
        UxIcon::Tool,
        UxIcon::ShoppingCart,
        UxIcon::ToggleRight,
        UxIcon::Filter,
        UxIcon::Lock,
        UxIcon::Columns,
        UxIcon::Unlock,
        UxIcon::Search,
        UxIcon::ShoppingBag,
        UxIcon::LogOut,
        UxIcon::Layers,
        UxIcon::BookOpen,
        UxIcon::MoreHorizontal,
        UxIcon::MousePointer,
        UxIcon::Shuffle,
        UxIcon::Bookmark,
        UxIcon::Tag,
        UxIcon::Link2,
        UxIcon::Pocket,
    ];

    let ux_names = vec![
        "ShieldOff",
        "Archive",
        "Activity",
        "Shield",
        "Crosshair",
        "BellOff",
        "EyeOff",
        "Sidebar",
        "MoreVertical",
        "Bell",
        "RefreshCw",
        "Clipboard",
        "Layout",
        "Loader",
        "Grid",
        "ToggleLeft",
        "Sliders",
        "Settings",
        "Eye",
        "Home",
        "Link",
        "LogIn",
        "Menu",
        "RotateCw",
        "Tool",
        "ShoppingCart",
        "ToggleRight",
        "Filter",
        "Lock",
        "Columns",
        "Unlock",
        "Search",
        "ShoppingBag",
        "LogOut",
        "Layers",
        "BookOpen",
        "MoreHorizontal",
        "MousePointer",
        "Shuffle",
        "Bookmark",
        "Tag",
        "Link2",
        "Pocket",
    ];

    ux_icons
        .iter()
        .enumerate()
        .map(|(index, icon)| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(3), ItemLayout::ItM(2), ItemLayout::ItL(1)]>
                    <UxAssets
                        icon = icon.clone()
                    />
                    <div>{ux_names[index]}</div>
                </Item>

            }
        })
        .collect::<Html>()
}
