# Theme

成熟度：`Alpha`。源码：`crates/slint-ui/ui/tokens/theme.slint`。公开名称：`Theme`。

为每个顶层窗口提供主题、密度、Locale、Direction、文本/预览缩放、减少动效和语义 Token。它不负责读取操作系统设置；系统高对比度、减少动效和多窗口同步由宿主或平台增强层完成。

## 公开 API

### 数据类型与枚举

#### `Density`

| 字段或值 | 类型/语义 |
|---|---|
| `compact` | `枚举值` |
| `regular` | `枚举值` |
| `comfortable` | `枚举值` |

#### `Direction`

| 字段或值 | 类型/语义 |
|---|---|
| `ltr` | `枚举值` |
| `rtl` | `枚举值` |

#### `ThemeMode`

| 字段或值 | 类型/语义 |
|---|---|
| `light` | `枚举值` |
| `dark` | `枚举值` |
| `high-contrast` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `mode` | `in-out` | `ThemeMode` | `ThemeMode.light` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `density` | `in-out` | `Density` | `Density.regular` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `direction` | `in-out` | `Direction` | `Direction.ltr` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `locale` | `in-out` | `string` | `"zh-CN"` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `text-scale` | `in-out` | `float` | `1.0` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `preview-scale` | `in-out` | `float` | `1.0` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `reduced-motion` | `in-out` | `bool` | `false` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `bg-layout` | `out` | `color` | `mode == ThemeMode.dark ? #000000 : mode == ThemeMode.high-contrast ? #000000 : #f5f5f5` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `bg-surface` | `out` | `color` | `mode == ThemeMode.dark ? #141414 : mode == ThemeMode.high-contrast ? #000000 : #ffffff` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `bg-elevated` | `out` | `color` | `mode == ThemeMode.dark ? #1f1f1f : mode == ThemeMode.high-contrast ? #000000 : #ffffff` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `bg-control` | `out` | `color` | `bg-surface` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `bg-disabled` | `out` | `color` | `mode == ThemeMode.dark ? #ffffff0a : mode == ThemeMode.high-contrast ? #202020 : #0000000a` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `text-primary` | `out` | `color` | `mode == ThemeMode.dark ? #ffffffd9 : mode == ThemeMode.high-contrast ? #ffffff : #000000e0` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `text-secondary` | `out` | `color` | `mode == ThemeMode.dark ? #ffffffa6 : mode == ThemeMode.high-contrast ? #ffffff : #000000a6` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `text-tertiary` | `out` | `color` | `mode == ThemeMode.dark ? #ffffff99 : mode == ThemeMode.high-contrast ? #ffffff : #00000099` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `text-disabled` | `out` | `color` | `mode == ThemeMode.dark ? #ffffff40 : mode == ThemeMode.high-contrast ? #a0a0a0 : #00000040` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `text-placeholder` | `out` | `color` | `text-disabled` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `text-inverse` | `out` | `color` | `mode == ThemeMode.dark \|\| mode == ThemeMode.high-contrast ? #000000e0 : #ffffff` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `text-on-accent` | `out` | `color` | `mode == ThemeMode.high-contrast ? #000000 : #ffffff` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `icon-default` | `out` | `color` | `text-tertiary` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `icon-hover` | `out` | `color` | `text-primary` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `border-default` | `out` | `color` | `mode == ThemeMode.dark ? #737373 : mode == ThemeMode.high-contrast ? #ffffff : #8c8c8c` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `border-secondary` | `out` | `color` | `mode == ThemeMode.dark ? #fdfdfd1f : mode == ThemeMode.high-contrast ? #ffffff : #0505050f` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `fill-hover` | `out` | `color` | `mode == ThemeMode.dark ? #ffffff14 : mode == ThemeMode.high-contrast ? #ffffff26 : #0000000f` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `fill-pressed` | `out` | `color` | `mode == ThemeMode.dark ? #ffffff1f : mode == ThemeMode.high-contrast ? #ffffff40 : #0000001a` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `selection-bg` | `out` | `color` | `mode == ThemeMode.dark ? #111a2c : mode == ThemeMode.high-contrast ? #ffffff : #e6f4ff` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `overlay-mask` | `out` | `color` | `mode == ThemeMode.dark ? #000000a6 : #00000073` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-primary` | `out` | `color` | `mode == ThemeMode.dark ? #69b1ff : mode == ThemeMode.high-contrast ? #00ffff : #0958d9` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-primary-hover` | `out` | `color` | `mode == ThemeMode.dark ? #91caff : mode == ThemeMode.high-contrast ? #ffffff : #003eb3` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-primary-active` | `out` | `color` | `mode == ThemeMode.dark ? #4096ff : mode == ThemeMode.high-contrast ? #00ffff : #002c8c` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-primary-fill` | `out` | `color` | `mode == ThemeMode.dark ? #1554ad : mode == ThemeMode.high-contrast ? #00ffff : #0958d9` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-primary-fill-hover` | `out` | `color` | `mode == ThemeMode.dark ? #1668dc : mode == ThemeMode.high-contrast ? #ffffff : #003eb3` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-primary-fill-active` | `out` | `color` | `mode == ThemeMode.dark ? #15325b : mode == ThemeMode.high-contrast ? #00ffff : #002c8c` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-primary-bg` | `out` | `color` | `mode == ThemeMode.dark ? #111a2c : mode == ThemeMode.high-contrast ? #000000 : #e6f4ff` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-primary-bg-hover` | `out` | `color` | `mode == ThemeMode.dark ? #15325b : mode == ThemeMode.high-contrast ? #202020 : #bae0ff` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-primary-border` | `out` | `color` | `mode == ThemeMode.dark ? #69b1ff : mode == ThemeMode.high-contrast ? #00ffff : #0958d9` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-success` | `out` | `color` | `mode == ThemeMode.dark ? #73d13d : mode == ThemeMode.high-contrast ? #00ff00 : #237804` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-success-hover` | `out` | `color` | `mode == ThemeMode.dark ? #95de64 : mode == ThemeMode.high-contrast ? #ffffff : #135200` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-success-active` | `out` | `color` | `mode == ThemeMode.dark ? #52c41a : mode == ThemeMode.high-contrast ? #00ff00 : #092b00` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-success-bg` | `out` | `color` | `mode == ThemeMode.dark ? #162312 : mode == ThemeMode.high-contrast ? #000000 : #f6ffed` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-success-border` | `out` | `color` | `color-success` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-warning` | `out` | `color` | `mode == ThemeMode.dark ? #ffc53d : mode == ThemeMode.high-contrast ? #ffff00 : #874d00` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-warning-hover` | `out` | `color` | `mode == ThemeMode.dark ? #ffd666 : mode == ThemeMode.high-contrast ? #ffffff : #613400` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-warning-active` | `out` | `color` | `mode == ThemeMode.dark ? #d89614 : mode == ThemeMode.high-contrast ? #ffff00 : #3d1f00` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-warning-bg` | `out` | `color` | `mode == ThemeMode.dark ? #2b2111 : mode == ThemeMode.high-contrast ? #000000 : #fffbe6` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-warning-border` | `out` | `color` | `color-warning` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-error` | `out` | `color` | `mode == ThemeMode.dark ? #ff7875 : mode == ThemeMode.high-contrast ? #ff8080 : #cf1322` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-error-hover` | `out` | `color` | `mode == ThemeMode.dark ? #ffa39e : mode == ThemeMode.high-contrast ? #ffffff : #a8071a` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-error-active` | `out` | `color` | `mode == ThemeMode.dark ? #ff4d4f : mode == ThemeMode.high-contrast ? #ff8080 : #820014` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-error-fill` | `out` | `color` | `mode == ThemeMode.dark ? #a61d24 : mode == ThemeMode.high-contrast ? #ff8080 : #cf1322` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-error-fill-hover` | `out` | `color` | `mode == ThemeMode.dark ? #c41d2c : mode == ThemeMode.high-contrast ? #ffffff : #a8071a` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-error-fill-active` | `out` | `color` | `mode == ThemeMode.dark ? #820014 : mode == ThemeMode.high-contrast ? #ff8080 : #820014` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-error-bg` | `out` | `color` | `mode == ThemeMode.dark ? #2c1618 : mode == ThemeMode.high-contrast ? #000000 : #fff2f0` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `color-error-border` | `out` | `color` | `color-error` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `focus-outline` | `out` | `color` | `mode == ThemeMode.dark ? #69b1ff : mode == ThemeMode.high-contrast ? #ffff00 : #0958d9` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `font-size-caption` | `out` | `length` | `12px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `font-size-body` | `out` | `length` | `14px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `font-size-title-small` | `out` | `length` | `16px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `font-size-title-medium` | `out` | `length` | `20px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `font-size-title-large` | `out` | `length` | `24px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `line-height-caption` | `out` | `length` | `20px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `line-height-body` | `out` | `length` | `22px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `line-height-title-small` | `out` | `length` | `24px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `line-height-title-medium` | `out` | `length` | `28px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `line-height-title-large` | `out` | `length` | `32px * text-scale` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `font-weight-regular` | `out` | `int` | `400` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `font-weight-medium` | `out` | `int` | `500` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `font-weight-semibold` | `out` | `int` | `600` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `space-1` | `out` | `length` | `4px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `space-2` | `out` | `length` | `8px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `space-3` | `out` | `length` | `12px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `space-4` | `out` | `length` | `16px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `space-6` | `out` | `length` | `24px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `space-8` | `out` | `length` | `32px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `space-12` | `out` | `length` | `48px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `radius-small` | `out` | `length` | `4px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `radius-medium` | `out` | `length` | `6px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `radius-large` | `out` | `length` | `8px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `border-width` | `out` | `length` | `mode == ThemeMode.high-contrast ? 2px : 1px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `focus-ring-width` | `out` | `length` | `2px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `focus-ring-offset` | `out` | `length` | `2px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `control-height-small` | `out` | `length` | `density == Density.comfortable ? 32px : 24px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `control-height-medium` | `out` | `length` | `density == Density.compact ? 28px : density == Density.comfortable ? 40px : 32px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `control-height-large` | `out` | `length` | `density == Density.compact ? 36px : density == Density.comfortable ? 48px : 40px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `control-padding-horizontal` | `out` | `length` | `density == Density.compact ? 8px : density == Density.comfortable ? 16px : 12px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `row-height` | `out` | `length` | `density == Density.compact ? 32px : density == Density.comfortable ? 48px : 40px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `icon-size-x-small` | `out` | `length` | `12px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `icon-size-small` | `out` | `length` | `16px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `icon-size-medium` | `out` | `length` | `20px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `icon-size-large` | `out` | `length` | `24px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `icon-size-x-large` | `out` | `length` | `32px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `icon-size-empty-state` | `out` | `length` | `48px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `field-affix-slot` | `out` | `length` | `24px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `control-min-width` | `out` | `length` | `120px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `card-min-width` | `out` | `length` | `160px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `empty-state-min-height` | `out` | `length` | `160px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `progress-min-width` | `out` | `length` | `80px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `progress-track-height` | `out` | `length` | `8px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `progress-indicator-min-width` | `out` | `length` | `24px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `status-indicator-size` | `out` | `length` | `8px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `separator-min-extent` | `out` | `length` | `1px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `layout-unbounded-extent` | `out` | `length` | `100000px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `switch-width-small` | `out` | `length` | `32px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `switch-width-medium` | `out` | `length` | `40px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `switch-width-large` | `out` | `length` | `48px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `switch-height-small` | `out` | `length` | `16px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `switch-height-medium` | `out` | `length` | `20px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `switch-height-large` | `out` | `length` | `24px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `switch-thumb-inset` | `out` | `length` | `2px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `overlay-content-min-width` | `out` | `length` | `240px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `popup-min-width` | `out` | `length` | `180px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `popup-max-height` | `out` | `length` | `360px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `tab-height` | `out` | `length` | `control-height-medium` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `split-handle-size` | `out` | `length` | `8px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `split-pane-min-size` | `out` | `length` | `120px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `dialog-min-width` | `out` | `length` | `360px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `dialog-max-width` | `out` | `length` | `640px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `toast-width` | `out` | `length` | `360px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `toast-duration` | `out` | `duration` | `4000ms` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `navigation-rail-width` | `out` | `length` | `72px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `command-palette-width` | `out` | `length` | `560px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `drawer-width` | `out` | `length` | `400px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `avatar-size-small` | `out` | `length` | `24px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `avatar-size-medium` | `out` | `length` | `32px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `avatar-size-large` | `out` | `length` | `40px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `calendar-cell-size` | `out` | `length` | `36px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `panel-header-min-height` | `out` | `length` | `48px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `app-shell-title-height` | `out` | `length` | `48px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `app-shell-navigation-width` | `out` | `length` | `220px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `app-shell-status-height` | `out` | `length` | `32px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `text-area-min-height` | `out` | `length` | `96px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `slider-min-width` | `out` | `length` | `160px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `badge-min-size` | `out` | `length` | `20px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `tag-height` | `out` | `length` | `24px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `popover-width` | `out` | `length` | `320px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `progress-ring-size` | `out` | `length` | `48px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `result-state-min-height` | `out` | `length` | `240px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `title-bar-height` | `out` | `length` | `40px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `motion-fast` | `out` | `duration` | `reduced-motion ? 0ms : 100ms` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `motion-medium` | `out` | `duration` | `reduced-motion ? 0ms : 200ms` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `motion-slow` | `out` | `duration` | `reduced-motion ? 0ms : 300ms` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `progress-cycle` | `out` | `duration` | `1200ms` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `spinner-cycle` | `out` | `duration` | `1000ms` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `skeleton-cycle` | `out` | `duration` | `800ms` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：`color-primary-active`、`color-primary-fill-active`、`color-success-active`、`color-warning-active`、`color-error-active`、`color-error-fill-active`、`font-size-caption`、`font-size-body`、`font-size-title-small`、`font-size-title-medium`、`font-size-title-large`、`icon-size-x-small`、`icon-size-small`、`icon-size-medium`、`icon-size-large`、`icon-size-x-large`、`icon-size-empty-state`、`empty-state-min-height`、`status-indicator-size`、`split-handle-size`、`split-pane-min-size`、`avatar-size-small`、`avatar-size-medium`、`avatar-size-large`、`calendar-cell-size`、`badge-min-size`、`progress-ring-size`、`result-state-min-height`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：不直接消费 Theme Token。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

Theme 不接收指针或键盘输入，也没有焦点和可访问节点。环境切换必须由宿主原子更新；组件通过绑定自动响应。

### 无障碍与本地化

Theme 不接收指针或键盘输入，也没有焦点和可访问节点。环境切换必须由宿主原子更新；组件通过绑定自动响应。

Gallery 顶栏覆盖三种主题、三种密度、100%–200% 缩放、中文/英文/阿拉伯语和 LTR/RTL。编译与截图验证 Token 在这些环境下可用；操作系统自动探测和多平台真机结果单独记录。

### 验证、宿主职责与限制

Gallery 顶栏覆盖三种主题、三种密度、100%–200% 缩放、中文/英文/阿拉伯语和 LTR/RTL。编译与截图验证 Token 在这些环境下可用；操作系统自动探测和多平台真机结果单独记录。

Slint global 不跨窗口共享，宿主必须逐窗口同步。完整约束遵循 [`foundations.md`](../../foundations.md)、[`accessibility.md`](../../accessibility.md) 和 [`content-and-localization.md`](../../content-and-localization.md)。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
