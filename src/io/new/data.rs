use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// 本地缓存信息
#[derive(Clone, Debug, Serialize, Deserialize, Default, Builder)]
pub(super) struct LocalData {
    /// 基础信息
    #[builder(default, setter(strip_option))]
    basics: Basic,
    /// 语言
    #[builder(default, setter(strip_option))]
    languages: Vec<Language>,
    /// 技能
    #[builder(default, setter(strip_option))]
    skills: Vec<Skill>,
    /// 参考,如书籍
    #[builder(default, setter(strip_option))]
    references: Vec<Reference>,
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
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub(super) struct Profile {
    /// 网站名称
    #[builder(default, setter(strip_option))]
    network: String,
    /// 网站地址
    #[builder(default, setter(strip_option))]
    url: String,
    /// 用户名
    #[builder(default, setter(strip_option))]
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
    country_code: Option<String>,
}

/// 语言
#[derive(Clone, Debug, Serialize, Deserialize, Default, Builder)]
#[builder(pattern = "mutable")]
pub(super) struct Language {
    /// 语言名称
    #[builder(default, setter(strip_option))]
    language: Option<String>,
    /// 熟练度
    #[builder(default, setter(strip_option))]
    fluency: Option<String>,
}
/// 技能
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub(super) struct Skill {
    /// 名称
    #[builder(default, setter(strip_option))]
    name: Option<String>,
}
/// 参考，如书籍
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub(super) struct Reference {
    /// 名称
    #[builder(default, setter(strip_option))]
    name: Option<String>,
}
/// 工作经历
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(pattern = "mutable")]
pub(super) struct Work {
    #[builder(default, setter(strip_option))]
    name: Option<String>,
    #[builder(default, setter(strip_option))]
    description: Option<String>,
    #[builder(default, setter(strip_option))]
    start_date: Option<String>,
    #[builder(default, setter(strip_option))]
    end_date: Option<String>,
    #[builder(default, setter(strip_option))]
    highlights: Vec<String>,
}
/// 教育经历
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
#[serde(rename_all = "camelCase")]
pub(super) struct Education {
    /// 学校
    #[builder(default, setter(strip_option))]
    institution: Option<String>,
    /// 开始日期
    #[builder(default, setter(strip_option))]
    start_date: Option<String>,
    /// 结束日期
    #[builder(default, setter(strip_option))]
    end_date: Option<String>,
    /// 学习类型，如全日制，成人教育等
    #[builder(default, setter(strip_option))]
    study_type: Option<String>,
    /// 领域或专业
    #[builder(default, setter(strip_option))]
    area: Option<String>,
}
