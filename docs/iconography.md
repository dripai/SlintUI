# SlintUI 图标库

SlintUI 图标库固定引入 `@ant-design/icons-svg` 4.5.0 的单色资源：

| 风格 | 数量 | 用途 |
|---|---:|---|
| Outlined | 447 | 默认操作、导航、对象和状态图标 |
| Filled | 249 | 选中、强状态和品牌图形 |

未引入 150 个 Two Tone 图标。上游 Two Tone SVG 内置 `#333` 和 `#E6E6E6` 等固定颜色，Slint 当前 `Image.colorize` 只能把整张图作为单一遮罩着色，无法分别映射主色和辅助色 Token。将其直接引入会破坏深色与高对比度主题，因此保持明确不支持，不做静默单色回退。

## 按需导入

每个资源都有独立 Slint 模块。推荐直接导入具体文件：

```slint
import { Icon, IconSize } from "@slint-ui/index.slint";
import { SearchOutlined } from "@slint-ui/icons/outlined/search.slint";
import { StarFilled } from "@slint-ui/icons/filled/star.slint";

HorizontalLayout {
    Icon {
        source: SearchOutlined.source;
        size: IconSize.small;
        accessible-name: "搜索";
    }
    Icon {
        source: StarFilled.source;
        size: IconSize.small;
        accessible-name: "已收藏";
    }
}
```

也提供聚合入口：

```slint
import { SearchOutlined, CloseOutlined } from "@slint-ui/icons/outlined.slint";
```

主 `@slint-ui/index.slint` 不重新导出图标目录。Slint 要求 `@image-url` 在编译期确定，独立模块使编译器只接触业务实际导入的资源；禁止用运行时字符串拼接图标路径。

## 命名和使用规则

- SVG 文件名使用上游 kebab-case，例如 `check-circle.svg`。
- Slint 全局名使用 `PascalCase + Outlined/ Filled`，例如 `CheckCircleOutlined`。
- 普通界面默认使用 Outlined；Filled 不作为 Hover 的替代样式。
- 图标颜色由 `Icon.color` 和 Theme 语义 Token 提供，不在业务页面写固定色值。
- 纯装饰图像使用原生 `Image { accessible-role: none; }`；传递信息的 `Icon` 必须提供本地化 `accessible-name`。
- Arrow、Caret、Left、Right、Undo、Redo 等方向图标的 RTL 镜像由调用组件显式处理；资源文件本身不隐式判断 Locale。
- Checkbox、TextField 清除入口等组件内部状态使用 SVG，不使用字体字符模拟。

## 完整清单与更新

机器可读清单位于 [`../crates/slint-ui/assets/icons/ant-design/manifest.json`](../crates/slint-ui/assets/icons/ant-design/manifest.json)，包含每种风格的全部名称和固定上游版本。

Gallery 的“图标”页面使用虚拟化列表展示全部 696 个资源，可切换 Outlined 与 Filled，用于开发期检索和主题验收；业务界面仍应按需导入单个图标模块。

更新流程：

```powershell
npm.cmd pack @ant-design/icons-svg@4.5.0
tar -xf ant-design-icons-svg-4.5.0.tgz
node tools/icons/import-ant-design-icons.mjs package/inline-svg
```

生成器会校验包名、MIT 许可、图标名称、SVG 根节点和外部资源，并重新生成资源、独立模块、聚合入口和 manifest。更新后必须运行编译、Gallery、UTF-8、资源数量与截图基线检查。

## 许可

Ant Design Icons 使用 MIT 许可，允许商业使用、修改和分发。分发时必须保留原版权声明和许可文本。详见 [`../THIRD_PARTY_NOTICES.md`](../THIRD_PARTY_NOTICES.md) 和资源目录内的 `LICENSE`。
