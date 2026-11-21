// Authoring only: node tools/generate_default_cover.cjs <module-mark.png> <sharp-module-path>
// Use the approved product mark. Preserve its geometry; composite on pure black.
const fs = require("node:fs");
const path = require("node:path");
const sharp = require(process.argv[3] || "sharp");
(async () => {
  const mark = await sharp(process.argv[2])
    .resize({ width: 704, height: 704, fit: "inside" })
    .png()
    .toBuffer();
  const destination = path.join(
    __dirname,
    "../skills/steam-workshop/assets/default-cover.png",
  );
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  await sharp({
    create: { width: 1024, height: 1024, channels: 3, background: "#000000" },
  })
    .composite([{ input: mark, gravity: "centre" }])
    .removeAlpha()
    .png()
    .toFile(destination);
  const { data, info } = await sharp(destination)
    .raw()
    .toBuffer({ resolveWithObject: true });
  if (info.width !== 1024 || info.height !== 1024 || info.channels !== 3)
    throw Error("Invalid cover dimensions or opacity");
  for (let y = 0; y < 1024; y++)
    for (let x = 0; x < 1024; x++) {
      if (x >= 160 && x < 864 && y >= 160 && y < 864) continue;
      const i = (y * 1024 + x) * 3;
      if (data[i] || data[i + 1] || data[i + 2])
        throw Error("Cover margin is not pure black");
    }
  const bytes = fs.statSync(destination).size;
  if (bytes >= 900000) throw Error("Cover exceeds upload budget");
  console.log(
    JSON.stringify({
      path: destination,
      width: 1024,
      height: 1024,
      opaque: true,
      blackMargin: true,
      bytes,
    }),
  );
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
