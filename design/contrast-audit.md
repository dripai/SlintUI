# SlintUI 对比度检查记录

状态：已完成当前 Windows 软件渲染基线检查。最近核对：2026-07-20。

本文记录 Theme 语义色的可计算对比度，不替代三平台真机、屏幕阅读器或用户自定义系统高对比度验收。计算采用 WCAG 2.2 相对亮度公式；普通文字目标不低于 `4.5:1`，大文字和非文字控件边界、状态、焦点目标不低于 `3:1`。禁用内容不作为可操作内容计入门禁。

## 语义配对

| 配对 | 浅色 | 深色 | 高对比度 | 门禁 | 结果 |
|---|---:|---:|---:|---:|---|
| `color-primary` / `bg-surface` | 6.16:1 | 8.19:1 | 16.75:1 | 4.5:1 | 通过 |
| `color-primary-fill` / `text-on-accent` | 6.16:1 | 7.23:1 | 16.75:1 | 4.5:1 | 通过 |
| `color-error` / `bg-surface` | 5.57:1 | 7.19:1 | 8.65:1 | 4.5:1 | 通过 |
| `color-error-fill` / `text-on-accent` | 5.57:1 | 7.44:1 | 8.65:1 | 4.5:1 | 通过 |
| `color-success` / `bg-surface` | 5.59:1 | 9.58:1 | 15.30:1 | 4.5:1 | 通过 |
| `color-warning` / `bg-surface` | 6.79:1 | 11.67:1 | 19.56:1 | 4.5:1 | 通过 |
| `border-default` / `bg-surface` | 3.36:1 | 3.89:1 | 21.00:1 | 3:1 | 通过 |
| `text-tertiary` / `bg-surface` | 5.74:1 | 7.13:1 | 21.00:1 | 4.5:1 | 通过 |

## 实施约束

- 链接、图标、校验文字、状态文字和焦点环使用 `color-*` 前景系列。
- Primary/Danger 实心按钮、Checkbox、Switch、Steps 完成标记和 Calendar 选中态使用 `*-fill` 系列，与 `text-on-accent` 配对。
- 状态容器使用 `*-bg`，其边框和图标使用同语义的 `*-border` / `color-*`；不能把浅背景色当作文字色。
- Badge 在浅色主题使用白色文字，在深色与高对比度主题使用黑色文字，避免亮状态色与白字组合。
- `border-secondary`、Hover 填充和禁用态只承担层级或不可用提示，不作为唯一的可操作边界；可交互控件仍由 `border-default`、焦点环、文字或图标共同表达。
- 颜色不是状态的唯一通道：组件同时暴露文字、图标、形状、选中/展开语义或可访问状态。

代表性视觉证据位于 [`../tests/screenshots/`](../tests/screenshots/)，逐组件证据见 [`component-quality-audit.md`](component-quality-audit.md)。
