# Independent language fields

Steam stores titles and descriptions separately for each language. Do not concatenate translations into one description. Common client API codes are `schinese` for Simplified Chinese, `english` for English, and `japanese` for Japanese, rather than the Web API codes `zh-CN`, `en`, and `ja`.

`set-text` requires an explicit `--language`, title, and UTF-8 description file. It updates only that language's title and description, preserving the cover, gallery, content files, and visibility. It calls SetItemUpdateLanguage before setting and submitting the text. Steam defaults to English when no API language is set; do not infer the write target from the player's client language.

```text
<CLI> set-text --app-id <APP_ID> --item-id <ITEM_ID> --language schinese --title "Localized title" --description-file <ABSOLUTE_PATH/zh.txt>
<CLI> get-text --app-id <APP_ID> --item-id <ITEM_ID> --language schinese
```

Update each requested language separately, then read back that same language and compare the complete title and description. Queries use SetLanguage. Missing translations may fall back, so report localization success only when the response matches the expected language-specific text. Preserve languages outside the requested changes.

Descriptions support Steam BBCode: `[h1]Heading[/h1]`, `[b]Emphasis[/b]`, `[list][*]Item[/list]`, and `[img]URL[/img]`. Different language descriptions can reference different images; screenshots without text can share one URL. These references are part of each localized description, not a separate image-language field.

For new items, explicitly provide a usable English fallback title and description, then add the other requested languages. Specify `--language english` or the intended language on ordinary update commands as well.

Sources: [write language](https://partner.steamgames.com/doc/api/ISteamUGC#SetItemUpdateLanguage), [query language](https://partner.steamgames.com/doc/api/ISteamUGC#SetLanguage), [language code table](https://partner.steamgames.com/doc/store/localization/languages).
