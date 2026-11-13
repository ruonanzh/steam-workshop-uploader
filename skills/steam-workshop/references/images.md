# Image rules

Treat the primary cover, gallery, and inline description images as separate resources. Do not automatically apply a new screenshot to every placement.

| Resource | Recommended production size | Verification |
| --- | --- | --- |
| Primary cover | Default 1024 x 1024; 512 x 512 can suit simple artwork | Recognizable at about 215 x 215; prominent subject, little small text, margins around logos |
| Gallery screenshot | Prefer 1920 x 1080 for new material; use 1280 x 720 when useful; preserve existing screenshot proportions | Check the roughly 16:9 player and 116 x 65 thumbnails; do not stretch or casually crop characters or UI |
| Inline image | Suggested width 1260 px, content-dependent height; do not upscale adequate existing material | Check text at about 630 px display width; split long explanations into several images |

These dimensions are project recommendations, not Valve requirements. Desktop measurements on 2026-11-13 found roughly 215 x 215 Duckov listing cards, a 637 x 358 gallery request, and a 630 px CSS maximum for images in the reference item's description. Responsive layouts and Steam updates can change display sizes. 256 x 256 is not a universal Steam upload requirement; treat game-local preview.png requirements separately.

The SDK preview upload documentation requires images smaller than 1 MB and suggests JPG, PNG, and GIF. Aim for at most 900,000 bytes to leave margin; this is not an official 900 KB limit. Inline network images also inherit this restriction if uploaded through that SDK route. Do not substitute Steam store asset rules or browser batch-upload limits for this CLI's limits.

Preserve originals. Prefer deterministic tools for format conversion, compression, resizing, and logo overlays so gameplay screenshots remain faithful. Make creative edits only as requested and distinguish promotional artwork from screenshots. Prefer JPEG for photos and PNG for flat artwork or clear UI. Keep GIF frames necessary for the demonstration and within the upload budget. Do not enlarge blurry material just to reach a suggested dimension.

- Cover: `--preview` writes the primary preview. It is not gallery index 0.
- Gallery: read with `get-previews`, append with `add-preview`. Duplicate filenames are rejected. This CLI currently has no gallery replacement, removal, or ordering command; do not invent flags or substitute repeated appends.
- Inline: use `[img]https://...[/img]` with an accessible original-image URL. An uploaded gallery or cover URL can be reused. Do not use disk paths or low-resolution thumbnail URLs containing imw/imh transformations.

After uploading, check each placement and actual browser image loading. No per-language primary-cover or gallery field has been found in the current SDK. Localized descriptions can reference different images. Steam store localized image groups are a separate feature.

Sources: [preview API](https://partner.steamgames.com/doc/api/ISteamUGC#AddItemPreviewFile), [upload errors and size limits](https://partner.steamgames.com/doc/api/ISteamUGC#SubmitItemUpdateResult_t), [separate store asset management](https://partner.steamgames.com/doc/store/page/assets).
