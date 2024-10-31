# 简历生成器
一个轻松生成简历的工具。

## 电梯演讲
作为软件开发人员的招聘过程可能非常繁琐。你申请的大多数公司都不会给你回复，如果他们联系你，你还得跳过很多环节才能得到工作。由于建议为每个申请定制简历，这可能意味着在最终被录用之前，你得创建数十份简历。谁有时间做这个？这个工具在这里通过生成一个高质量的简历来简化这个过程，所需的工作量非常少。

## 开始使用
目前，安装这个程序的唯一支持方法是通过下载或克隆这个仓库，并使用 `cargo` 或 `rustc` 自己构建二进制文件。

执行程序需要一个 Google Chrome 或 Chromedriver 的实例。

## 使用方法
`rsume` 应该像这样从命令行使用：
```bash
rsume gen /path/to/resume_data.yaml /target/path.pdf --template "coruscant" --language "english"
```
`--template` 和 `--language` 选项是可选的。

简历数据应该遵循 [JSONResume](https://jsonresume.org/) 模式，并且可以存储为 `.json` 或 `.yaml` 文件。并在JSONResume的基本上进行了升级。添加了Summary,features等内容。

## 已知问题
目前，只有一个模板可用。未来计划增加更多模板。
- 如果简历内容足够短，只填满一页，无论如何都会生成一个空的第二页。
- 在 coruscant 模板中，分页可能会将节标题（如“教育”）与第一个条目分开。

