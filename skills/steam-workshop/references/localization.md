# 独立语言字段

Steam 将各语言的标题和描述分开保存。不要把多语文本拼进一个描述。常用 API 代码：简体中文 `schinese`、英文 `english`、日文 `japanese`；不是 `zh-CN`、`en`、`ja`。

本目录 CLI 的 `set-text` 必须显式指定 `--language`、标题和 UTF-8 描述文件。它只写该语言的标题、描述，不改变封面、画廊、文件和可见性。内部使用 `SetItemUpdateLanguage`，再设置标题、描述并提交。未指定 API 语言时 Steam 默认英文，因此不能依赖客户端语言猜写入目标。

```text
<CLI> set-text --app-id <APP_ID> --item-id <ITEM_ID> --language schinese --title "中文标题" --description-file <绝对路径/zh.txt>
<CLI> get-text --app-id <APP_ID> --item-id <ITEM_ID> --language schinese
```

分别对 english、schinese、japanese 执行所需更新，然后按同一语言回读，比较完整标题与描述。读接口使用 `SetLanguage`；某语言缺失时可能回退，只有读回与该语言预期文本一致，才能报告已本地化。仅修改某语言时，其他语言保留。

正文支持 Steam BBCode：`[h1]标题[/h1]`、`[b]重点[/b]`、`[list][*]条目[/list]`、`[img]URL[/img]`。各语言可引用不同插图；无文字的实机图可以共用。图片引用仍属于该语言的描述，不是新的“图片语言”字段。

新建时先用 english 明确写入可用的默认标题和描述，再按需要添加其他语言。普通 update 也应明确 `--language english` 或所需语言，避免把翻译写到默认字段。

来源：[写入语言](https://partner.steamgames.com/doc/api/ISteamUGC#SetItemUpdateLanguage)、[查询语言](https://partner.steamgames.com/doc/api/ISteamUGC#SetLanguage)、[语言代码表](https://partner.steamgames.com/doc/store/localization/languages)。
