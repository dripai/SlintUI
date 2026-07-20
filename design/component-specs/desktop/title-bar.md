# TitleBar

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/title-bar.slint`。公开名称：`TitleBar`。

提供自绘桌面标题栏外观、拖动请求和窗口操作槽位；真实拖动、缩放、系统菜单和窗口状态属于宿主/平台增强。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `icon` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `active` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `show-minimize` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `show-maximize` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `show-close` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `maximized` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `minimize-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `maximize-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `restore-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `close-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `drag-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `request-minimize()` | `无` | 当前实现约束：`if (show-minimize) { root.minimize-requested(); }` |
| `request-toggle-maximize()` | `无` | 当前实现约束：`if (show-maximize) { if (maximized) { root.restore-requested(); } else { root.maximize-requested(); } }` |
| `request-close()` | `无` | 当前实现约束：`if (show-close) { root.close-requested(); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Rectangle`、`Image`、`Text`、`TouchArea` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`active`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-secondary`、`Theme.border-width`、`Theme.color-error-fill`、`Theme.fill-hover`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.icon-default`、`Theme.icon-size-small`、`Theme.space-2`、`Theme.space-3`、`Theme.text-on-accent`、`Theme.text-primary`、`Theme.text-secondary`、`Theme.title-bar-height`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Active/Inactive、Hover close、Maximized/Restored、按钮隐藏、长标题和双击标题；请求不直接修改系统窗口。

### 无障碍与本地化

根使用 groupbox，窗口按钮分别使用 button；名称必须由宿主按平台语言提供。标题省略但完整名称仍可访问。

### 验证、宿主职责与限制

Gallery“布局 / TitleBar”展示图标与三种窗口按钮；Harness 覆盖最小化、最大化和关闭请求。平台命中测试、拖动和原生阴影不在核心实现。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
