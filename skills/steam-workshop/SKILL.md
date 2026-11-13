---
name: steam-workshop
description: 将当前 Mod 上传到 Steam Workshop，或更新已有条目的文件、封面、截图画廊及各语言标题和描述。适用于发布已经制作并验证的 Mod。
---

# Steam Workshop 发布

使用本目录随附的 Rust CLI，复用本机已登录的 Steam。玩家须持有相应游戏和发布权限；无需 API Key，也不向 Agent 提供密码或 Steam Guard 验证码。连接失败时让玩家在官方 Steam 客户端检查登录，再按返回错误处理。

## 工作空间与工具

- 从当前游戏仓库根目录的 `mod-repo.json` 读取 `game.steamAppId`。缺失时说明缺项，不猜测 App ID。
- 当前 Mod 的工作空间是当前会话对应的 `your_mods/<folder name>/`。没有明确当前 Mod 时先定位或询问，不跨到另一个 Mod。
- Mod 的编译和游戏内验证由已有制作流程负责。上传任务直接使用已验证的成果。
- 根据当前 Mod 的实际文件准备发布目录，文件选择由 Agent 判断；不要求游戏适配器或固定文件清单。发布中间文件与记录放在当前 Mod 内，例如 `.workshop/`。
- 将本 `SKILL.md` 所在目录记为技能目录。按 `bundle.json` 选择其中可用的平台二进制，始终用绝对路径调用；不依赖 cwd、全局 PATH 或本机开发仓库。原生动态库须保留在可执行文件旁边。平台缺失时说明缺哪个包，不用旧版 CLI 顶替。
- macOS 可执行路径为 `bin/macos-universal/workshop`（平台专用包也可能为 `macos-arm64`）；Windows 为 `bin/windows-x64/workshop.exe`。命令语法见 [CLI 操作参考](references/cli.md)。技能目录只保存技能和工具，不保存玩家数据。

## 发布流程

1. 确定目标：区分新建与更新。更新使用用户指定或当前 Mod 已记录的 Workshop ID；只有 ID 存在不能证明归属，先查询目标游戏与账号可见的实际条目。无法确定更新目标时不自动新建。
2. 准备展示内容：标题、描述、封面和可选画廊。处理图片时读 [图片规范](references/images.md)；需要多语言时读 [本地化](references/localization.md)。这些内容来自当前 Mod 和玩家要求，不添加未经要求的宣传或发布声明。
3. 准备上传目录，并用 CLI 的 `files` 检查实际上传清单。它会应用忽略规则，编译产物可能因此被遗漏。清单应与本次选择一致；不要上传整个游戏仓库。
4. 执行创建或更新。新建未指定可见性时使用 private 并如实告知；明确要求公开时直接使用 public。更新未指定时保留原可见性。用户已明确要求该次发布或更新时，不重复确认。
5. 保存 CLI 返回的 Workshop ID 和 `workshop.toml`，后续更新复用同一条目。将记录保留在当前 Mod 内，且与运行时 Mod 身份中的 ID 保持一致（该格式由当前 Mod 决定）。超时或结果未知时先查询并核对，不盲目重试创建或追加画廊。
6. 回读：标题与完整描述按语言分别核对；封面核对原图 URL，画廊核对数量、文件名和加载结果。返回 Workshop 链接与实际验证范围。详细结果与失败处理见 [发布规则](references/publishing.md)。

分享到我们的社区属于独立动作。只有玩家要求时，才接续使用可用的 community-share 技能。
