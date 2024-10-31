use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// 本地缓存信息
#[derive(Clone, Debug, Serialize, Deserialize, Default, Builder)]
pub(super) struct LocalData {
    /// 基础信息
    #[builder(default, setter(strip_option))]
    basic: Basic,
    /// 语言
    #[builder(default, setter(strip_option))]
    languages: Language,
    /// 技能
    #[builder(default, setter(strip_option))]
    skill: Vec<Skill>,
    /// 参考,如书籍
    #[builder(default)]
    references: Vec<Refernce>,
    /// 工作经历
    #[builder(default)]
    work: Vec<Work>,
    /// 教育经历
    #[builder(default)]
    education: Vec<Education>,
}

/// 基础信息
#[derive(Clone, Debug, Serialize, Deserialize, Default, Builder)]
#[builder(pattern = "mutable")]
pub(super) struct Basic {
    /// 姓名
    #[builder(default, setter(strip_option))]
    name: Option<String>,
    /// 职位，标签
    #[builder(default, setter(strip_option))]
    label: Option<String>,
    /// 个人照片
    #[builder(default, setter(strip_option))]
    image: Option<String>,
    /// 电子邮箱
    #[builder(default, setter(strip_option))]
    email: Option<String>,
    /// 联系电话
    #[builder(default, setter(strip_option))]
    phone: Option<String>,
    /// 自我评价
    #[builder(default, setter(strip_option))]
    summary: Option<String>,
    /// 个人社交网站
    #[builder(default, setter(strip_option))]
    profiles: Option<Vec<Profile>>,
    /// 个人住址
    #[builder(default, setter(strip_option))]
    location: Option<Location>,
}

/// 个人社交网站
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Profile {
    /// 网站名称
    network: String,
    /// 网站地址
    url: String,
    /// 用户名
    username: String,
}

/// 个人住址
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(pattern = "mutable")]
pub(super) struct Location {
    #[builder(default, setter(strip_option))]
    address: Option<String>,
    #[builder(default, setter(strip_option))]
    postal_code: Option<String>,
    #[builder(default, setter(strip_option))]
    city: Option<String>,
    #[builder(default, setter(strip_option))]
    coutry_code: Option<String>,
}

/// 语言
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct Language {
    /// 语言名称
    language: String,
    /// 熟练度
    fluency: String,
}
/// 技能
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Skill {
    /// 名称
    name: String,
}
/// 参考，如书籍
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Refernce {
    /// 名称
    name: String,
}
/// 工作经历
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Work {}
/// 教育经历
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Education {}