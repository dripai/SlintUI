# AppShell

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/app-shell.slint`。公开名称：`AppShell`。

用于标题、导航、内容和状态区域的桌面窗口骨架；不负责真实窗口管理或平台命中测试。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `title-height` | `in` | `length` | `Theme.app-shell-title-height` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `navigation-width` | `in` | `length` | `Theme.app-shell-navigation-width` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `status-height` | `in` | `length` | `Theme.app-shell-status-height` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `navigation-visible` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `title-x` | `out` | `length` | `0px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `title-y` | `out` | `length` | `0px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `title-width` | `out` | `length` | `root.width` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `navigation-x` | `out` | `length` | `Theme.direction == Direction.rtl ? root.width - effective-navigation-width : 0px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `navigation-y` | `out` | `length` | `title-height` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `navigation-height` | `out` | `length` | `max(0px, root.height - title-height - status-height)` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `content-x` | `out` | `length` | `Theme.direction == Direction.rtl ? 0px : effective-navigation-width` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `content-y` | `out` | `length` | `title-height` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `content-width` | `out` | `length` | `max(0px, root.width - effective-navigation-width)` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `content-height` | `out` | `length` | `max(0px, root.height - title-height - status-height)` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `status-x` | `out` | `length` | `0px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `status-y` | `out` | `length` | `max(0px, root.height - status-height)` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `status-width` | `out` | `length` | `root.width` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `Rectangle` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.app-shell-navigation-width`、`Theme.app-shell-status-height`、`Theme.app-shell-title-height`、`Theme.bg-layout`、`Theme.bg-surface`、`Theme.border-secondary`、`Theme.border-width`、`Theme.direction`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖导航显示/隐藏、LTR/RTL、窄窗口和缩放；交互状态由区域内组件负责。

### 无障碍与本地化

根使用 groupbox；区域内容必须自行声明 landmark 语义。RTL 交换导航物理侧，源码焦点顺序不变。

### 验证、宿主职责与限制

Gallery 展示完整四区骨架；编译与截图验收。Slint 1.17.1 无命名 slot，采用显式几何输出，不提供平台窗口操作。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
