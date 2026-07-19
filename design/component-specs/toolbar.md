# Toolbar

状态：已实现。源码：`ui/layout/toolbar.slint`。

## 用途与边界

用于组合 `ToolButton`、`IconButton` 和垂直 `Divider`。复杂溢出菜单和任意子项方向键导航不属于当前 P0。

## 公开 API

`accessible-name: string = "Toolbar"` 和 `@children`；表面、边框、圆角和最小高度来自 Theme。

## 状态与交互

Toolbar 不截获子控件事件。当前焦点顺序是声明顺序的 Tab/Shift+Tab；按钮自身处理 Enter/Space。分组和分隔由子项显式表达。

## 无障碍与本地化

角色为 `groupbox`，调用方必须提供已本地化的可访问名称。无文字按钮仍需名称和 Tooltip；RTL 下业务顺序由调用方明确编排。

## Gallery、测试与限制

Gallery“操作”页覆盖 IconButton、ToolButton、checked 和 Divider；自动测试覆盖按钮激活与 checked 转换。Slint 稳定 API 无法枚举任意 `@children` 实现 roving focus 和溢出，相关能力保持待实现。遵循四份全局规范。
