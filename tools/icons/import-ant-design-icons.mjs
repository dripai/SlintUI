import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDirectory, "../..");
const sourceRoot = path.resolve(process.argv[2] ?? "node_modules/@ant-design/icons-svg/inline-svg");
const packageRoot = path.dirname(sourceRoot);
const packageMetadataPath = path.join(packageRoot, "package.json");
const assetRoot = path.join(workspaceRoot, "crates/slint-ui/assets/icons/ant-design");
const moduleRoot = path.join(workspaceRoot, "crates/slint-ui/ui/icons");
const themes = ["outlined", "filled"];

function assertInsideWorkspace(target) {
  const relative = path.relative(workspaceRoot, target);
  if (relative.startsWith("..") || path.isAbsolute(relative)) {
    throw new Error(`refusing to write outside workspace: ${target}`);
  }
}

function pascalCase(value) {
  return value
    .split("-")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

function normalizeSvg(svg, sourcePath) {
  if (!svg.startsWith("<svg ") || !svg.includes("viewBox=")) {
    throw new Error(`unsupported SVG root: ${sourcePath}`);
  }
  if (/<script\b|https?:\/\/|xlink:href=/i.test(svg)) {
    throw new Error(`external or executable SVG content: ${sourcePath}`);
  }
  return svg.replace("<svg ", '<svg xmlns="http://www.w3.org/2000/svg" ');
}

if (!fs.existsSync(packageMetadataPath)) {
  throw new Error(`missing package metadata: ${packageMetadataPath}`);
}

const packageMetadata = JSON.parse(fs.readFileSync(packageMetadataPath, "utf8"));
if (packageMetadata.name !== "@ant-design/icons-svg" || packageMetadata.license !== "MIT") {
  throw new Error("unexpected icon package or license");
}

for (const target of [assetRoot, moduleRoot]) {
  assertInsideWorkspace(target);
  fs.mkdirSync(target, { recursive: true });
}

const manifest = {
  source: {
    package: packageMetadata.name,
    version: packageMetadata.version,
    license: packageMetadata.license,
    homepage: packageMetadata.homepage,
  },
  styles: {},
};

for (const theme of themes) {
  const sourceThemeRoot = path.join(sourceRoot, theme);
  const assetThemeRoot = path.join(assetRoot, theme);
  const moduleThemeRoot = path.join(moduleRoot, theme);
  for (const target of [assetThemeRoot, moduleThemeRoot]) {
    assertInsideWorkspace(target);
    fs.rmSync(target, { recursive: true, force: true });
  }
  fs.mkdirSync(assetThemeRoot, { recursive: true });
  fs.mkdirSync(moduleThemeRoot, { recursive: true });

  const names = fs
    .readdirSync(sourceThemeRoot)
    .filter((fileName) => fileName.endsWith(".svg"))
    .map((fileName) => path.basename(fileName, ".svg"))
    .sort();
  const suffix = theme === "outlined" ? "Outlined" : "Filled";
  const exports = [];

  for (const name of names) {
    if (!/^[a-z][a-z0-9-]*$/.test(name)) {
      throw new Error(`invalid icon name: ${name}`);
    }

    const sourcePath = path.join(sourceThemeRoot, `${name}.svg`);
    const svg = normalizeSvg(fs.readFileSync(sourcePath, "utf8"), sourcePath);
    fs.writeFileSync(path.join(assetThemeRoot, `${name}.svg`), `${svg.trim()}\n`, "utf8");

    const globalName = `${pascalCase(name)}${suffix}`;
    const module = `export global ${globalName} {\n    out property <image> source: @image-url("../../../assets/icons/ant-design/${theme}/${name}.svg");\n}\n`;
    fs.writeFileSync(path.join(moduleThemeRoot, `${name}.slint`), module, "utf8");
    exports.push(`export { ${globalName} } from "${theme}/${name}.slint";`);
  }

  fs.writeFileSync(path.join(moduleRoot, `${theme}.slint`), `${exports.join("\n")}\n`, "utf8");
  manifest.styles[theme] = { count: names.length, icons: names };
}

fs.writeFileSync(
  path.join(assetRoot, "manifest.json"),
  `${JSON.stringify(manifest, null, 2)}\n`,
  "utf8",
);

const catalogEntries = (theme) =>
  manifest.styles[theme].icons
    .map(
      (name) =>
        `        { name: "${name}", source: @image-url("../../assets/icons/ant-design/${theme}/${name}.svg") },`,
    )
    .join("\n");
const catalogModule = `export struct IconCatalogEntry {\n    name: string,\n    source: image,\n}\n\nexport global IconCatalog {\n    out property <[IconCatalogEntry]> outlined: [\n${catalogEntries("outlined")}\n    ];\n    out property <[IconCatalogEntry]> filled: [\n${catalogEntries("filled")}\n    ];\n}\n`;
fs.writeFileSync(path.join(moduleRoot, "catalog.slint"), catalogModule, "utf8");

console.log(
  `Imported ${manifest.styles.outlined.count} outlined and ${manifest.styles.filled.count} filled icons from ${packageMetadata.name}@${packageMetadata.version}.`,
);
