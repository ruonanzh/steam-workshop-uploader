# CLI 操作参考

下文 `<CLI>` 代表技能目录中当前平台可执行文件的绝对路径，`<CONTENT>` 是当前 Mod 内准备好的发布目录。参数值使用宿主 shell 的正确引用；含空格或中文的路径必须引用。

macOS/Bash：`"<技能绝对目录>/bin/macos-universal/workshop" --help`

Windows/PowerShell：`& "<技能绝对目录>/bin/windows-x64/workshop.exe" --help`

所有修改前先确定 App ID、目标条目和玩家的本次意图。使用 `--no-prompt` 禁用终端交互。运行时无需 Rust、Node 或 Python；构建源码才需要开发工具。

```text
<CLI> files <CONTENT>
<CLI> --no-prompt create --app-id <APP_ID> --content <CONTENT> --language english --title "Title" --description-file <绝对路径/en.txt> --visibility private --preview <封面绝对路径>
<CLI> --no-prompt update --content <CONTENT> --language english --visibility public --description-file <绝对路径/en.txt>
<CLI> --no-prompt update --content <CONTENT> --language english --no-content-update --preview <封面绝对路径>
<CLI> get-text --app-id <APP_ID> --item-id <ITEM_ID> --language english
<CLI> set-text --app-id <APP_ID> --item-id <ITEM_ID> --language japanese --title "日本語タイトル" --description-file <绝对路径/ja.txt>
<CLI> get-previews --app-id <APP_ID> --item-id <ITEM_ID>
<CLI> add-preview --app-id <APP_ID> --item-id <ITEM_ID> --file <截图绝对路径>
```

create 将返回 ID 写入 `<CONTENT>/workshop.toml`。update 从该文件读取 app_id/item_id。已有条目但尚无文件时，可用纯本地命令创建记录：

```text
<CLI> --no-prompt init --app-id <APP_ID> --item-id <ITEM_ID> --content <CONTENT>
```

init 不证明条目归属。先通过 get-previews/get-text 查询并核对目标。`workshop.toml` 自动排除出上传文件，不要把发布记录放进技能安装目录。

get-text、set-text、get-previews、add-preview 在 stdout 输出 JSON；Steam 原生日志可能出现在 stderr。通用 create/update 仍为人类可读输出和文件记录，不能假设所有命令都输出 JSON。以成功返回、远端回读和记录三者判断结果。

add-preview 成功会返回新画廊及 verified；超时或 verified=false 时先 get-previews 核对。即使本地等待失败，Steam 也可能已完成上传。
