# Panel

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/panel.slint`。公开名称：`Panel`。

用于带固定标题区和内容区的桌面面板；强调独立信息块使用 Card，页面骨架使用 AppShell。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `description` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `bordered` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Rectangle`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`bordered`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-default`、`Theme.border-secondary`、`Theme.border-width`、`Theme.card-min-width`、`Theme.direction`、`Theme.font-size-caption`、`Theme.font-size-title-small`、`Theme.font-weight-medium`、`Theme.panel-header-min-height`、`Theme.radius-large`、`Theme.space-1`、`Theme.space-2`、`Theme.space-4`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

支持有/无标题、有/无边框、长标题与内容缩放；本身无 Hover、Pressed、Selected、Disabled、Loading 或 Error。

### 无障碍与本地化

使用 groupbox 角色，标题作为默认名称；标题和描述支持中文、英文、RTL 与换行。

### 验证、宿主职责与限制

Gallery“布局 / Panel”展示标题、说明、内容和操作；编译与截图验收。动作区由调用方在内容中显式编排。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
