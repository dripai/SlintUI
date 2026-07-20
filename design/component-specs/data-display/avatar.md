# Avatar

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/avatar.slint`。公开名称：`Avatar`。

展示用户、团队或实体的紧凑身份图形，不承担图片上传。

## 公开 API

### 数据类型与枚举

#### `ControlSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `source` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `initials` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `background-color` | `in` | `color` | `Theme.color-primary-bg` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `foreground-color` | `in` | `color` | `Theme.color-primary` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `circular` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

当前实现由 `Image`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.avatar-size-large`、`Theme.avatar-size-medium`、`Theme.avatar-size-small`、`Theme.color-primary`、`Theme.color-primary-bg`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.radius-medium`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

非交互组件；图片存在时覆盖文字回退，支持三种 ControlSize。

### 无障碍与本地化

使用 image 角色和显式名称；缩写由宿主依据姓名与 Locale 生成。

### 验证、宿主职责与限制

Gallery 展示文字回退；编译和截图验证尺寸。Slint 图片类型不公开加载失败事件，网络加载与失败回退由宿主更新 `source`/`initials`。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
