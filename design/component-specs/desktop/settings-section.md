# SettingsSection

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/settings-section.slint`。公开名称：`SettingsSection`。

用于设置页标题、说明和 FormRow 组合。普通卡片内容使用 `Card`；跨分组提交逻辑由页面或后续 `Form` 管理。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `description` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.direction`、`Theme.font-size-body`、`Theme.font-size-title-small`、`Theme.font-weight-medium`、`Theme.line-height-body`、`Theme.line-height-title-small`、`Theme.space-3`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

没有独立交互状态或焦点；子控件按声明顺序参与 Tab。间距、排版和颜色来自 Theme。

### 无障碍与本地化

角色为 `groupbox`，标题作为名称，说明作为描述。调用方负责本地化；标题和说明支持长文本、RTL 和文本缩放。

### 验证、宿主职责与限制

Gallery“布局 / SettingsSection”页提供单一设置分组示例；错误、选择、开关与多语言分别在对应组件页和全局环境中验证。容器本身通过编译和截图验收。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
